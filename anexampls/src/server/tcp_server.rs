// @Author: plucky
// @Date: 2023-06-25 11:47:05
// @LastEditTime: 2023-08-10 12:26:56

use bytes::Bytes;
use futures::{Future, SinkExt, StreamExt};
use std::{error::Error,io, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc, Semaphore},
};
use tower::{Service, ServiceExt};
use tracing::{error, info};

use super::{framer::FramerBuilder, AnError, BroadcastSender, NetEvent};
use crate::server::Connection;

pub struct Server<E, FB> {
    listen_addr: String,
    // max_conns: usize,
    // 最大连接数许可
    max_conns_semaphore: Arc<Semaphore>,

    // 心跳时间,2个心跳周期内没有收到客户端消息,则关闭连接
    heartbeat_interval: Duration,
    notify_shutdown: broadcast::Sender<()>,
    event_service: Option<E>,
    framer_builder: FB,
    broadcast_sender: BroadcastSender,
    
    // _p: PhantomData<F>,
}

impl<E, FB> Server<E, FB>
where
    E: Service<NetEvent> + Send  + Clone + 'static,
    // E: for<'a> Service<Reqest<'a>, Error = AnError, Future = F> + Send + Clone + 'static,
    E::Error: std::fmt::Debug + Send,
    E::Future: Future<Output = Result<Option<Bytes>, E::Error>> + Send,
    FB: FramerBuilder<Stream = TcpStream> + Send + Sync,
    FB::Framed: Send + 'static,
{
    pub async fn bind(listen_addr: impl Into<String>) -> Result<TcpListener, io::Error> {
        TcpListener::bind(&listen_addr.into()).await
    }

    pub fn new(
        listen_addr: impl Into<String>,
        max_conns: usize,
        heartbeat_interval: Duration,
        framer_builder: FB,
    ) -> Self {
        let listen_addr = listen_addr.into();
        let max_conns_semaphore = Arc::new(Semaphore::new(max_conns));
        let (notify_shutdown, _) = broadcast::channel(1);
        let (broadcast_sender, _) = broadcast::channel(1024);

        Self {
            listen_addr,
            max_conns_semaphore,
            heartbeat_interval,
            notify_shutdown,
            event_service: None,
            framer_builder,
            broadcast_sender,
            // _p: PhantomData,
        }
    }

    pub fn serve(&mut self, event_service: E) {
        self.event_service = Some(event_service);
    }

    pub fn get_broadcast_sender(&self) -> BroadcastSender {
        self.broadcast_sender.clone()
    }

    pub fn get_connections(&self) -> usize {
        self.notify_shutdown.receiver_count().saturating_sub(1)
    }

    pub fn close(&self) {
        let _ = self.notify_shutdown.send(());
    }

    pub fn with_shutdown<F>(&self, shutdown: F) -> &Self 
    where  F: Future + Send + 'static
    {
        let notify_shutdown = self.notify_shutdown.clone();
        tokio::spawn(async move {
            shutdown.await;
            info!("Server is shutting down!!! from signal");
            let _=notify_shutdown.send(());
        });
        self
    }

    pub async fn run(&mut self) -> Result<(), AnError> {
        let listener = TcpListener::bind(&self.listen_addr).await?;

        info!("Listening on {} ......", self.listen_addr);

        // 广播channel，用于给各子线程发送关闭信息
        let notify_shutdown = self.notify_shutdown.clone();
        let mut shutdown_receiver = notify_shutdown.subscribe();

        // 用于通知主线程，各子线程执行完成。
        let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel::<()>(1);

        tokio::select! {
            r =self.handle(&listener, &notify_shutdown, &shutdown_complete_tx) =>{
                if let Err(err) = r {
                    error!(cause = %err, "failed to accept");
                }
            }
            // 收到退出信号
            // _ = shutdown => {
            //     info!("Server is shutting down!!!");
            // }
            _ = shutdown_receiver.recv() => {
                info!("Server is shutting down!!! from shutdown receiver");
            }

        }

        // 关闭信号量,让监听线程退出
        self.max_conns_semaphore.close();
        drop(listener);
        let _ = notify_shutdown.send(());
        
        let conns = notify_shutdown.receiver_count().saturating_sub(1);

        tokio::time::timeout(Duration::from_secs(10), async move {
            // 等待所有子线程退出
            for _ in 0..conns {
                let _ = shutdown_complete_rx.recv().await;
            }
        }).await.unwrap();

        info!("Server all process completed!");

        Ok(())
    }

    // 与客户端建立连接
    async fn handle(
        &mut self,
        listener: &TcpListener,
        notify_shutdown: &broadcast::Sender<()>,
        shutdown_complete_tx: &mpsc::Sender<()>,
    ) -> Result<(), Box<dyn Error>> {
        let heartbeat_interval = self.heartbeat_interval;

        let event_service = self.event_service.take().expect("service must be set!!!");

        loop {
            // 限制最大连接数
            let permit = self.max_conns_semaphore.clone().acquire_owned().await?;
            let (stream, addr) = listener.accept().await?;
            let conns = notify_shutdown.receiver_count();

            info!("Client: {} connected, Current connections: {},", addr, conns);

            let mut shutdown = notify_shutdown.subscribe();
            let shutdown_complete = shutdown_complete_tx.clone();
            let mut broadcast = self.broadcast_sender.subscribe();

            let (sender, mut receiver) = mpsc::channel::<Bytes>(10);
            let conn = Connection::new(addr.to_string(), sender);
            let conn = Arc::new(conn);

            let mut servic = event_service.clone();

            let stream = match self.framer_builder.build(stream) {
                Ok(stream) => stream,
                Err(err) => {
                    error!("Failed to build framer: {}", err);

                    continue;
                },
            };

            // new handle
            let handle_connection = async move {
                let _permit = permit;

                let mut sv = servic.ready().await;
                if sv.is_err() {
                    error!("Connection denial");
                    return;
                }

                // 通知有新的连接
                let ev = NetEvent::Connect(conn.clone());
                // let ev = (&conn, NetEvent::Connect);
                if let Err(err) = sv.unwrap().call(ev).await {
                    error!("Connection denial: {:?}", err);
                    return;
                }

                let (mut writer, mut reader) = stream.split();

                loop {
                    tokio::select! {
                        buf = reader.next() => match buf {
                            Some(Ok(buf)) => {
                                // Call service
                                sv = servic.ready().await;
                                while sv.is_err(){
                                    sv = servic.ready().await;
                                }
                                let ev = NetEvent::Message(conn.clone(), buf);
                                // let ev = (&conn, NetEvent::Message(buf));
                                let _req = sv.unwrap().call(ev).await;

                                match _req{
                                    Ok(buf) => {
                                        if let Some(data) = buf {
                                            if let Err(e) = writer.send(data).await {
                                                error!("Failed to send response: {}", e);
                                                break;
                                            }
                                        }

                                    },
                                    Err(err) => {
                                        error!("Failed to execute: {:?}", err);
                                        break;
                                    },


                                }


                            },
                            _ => {
                                break;
                            }
                        },
                        // 接收广播通知
                        Ok(publish) = broadcast.recv() => {
                            // 如果没有订阅主题,跳过
                            if !conn.is_subscribed(&publish.0) {
                                continue;
                            }
                            if let Err(e) = writer.send(publish.1).await {
                                error!("Failed to send response: {}", e);
                                break;
                            }

                        },
                        // 发送消息
                        Some(data) = receiver.recv() => {
                            if let Err(e) = writer.send(data).await {
                                error!("Failed to send response: {}", e);
                                break;
                            }
                        },

                        // 心跳超时
                        _ = tokio::time::sleep(heartbeat_interval) => {
                            info!("Client: {} heartbeat timeout", conn.addr);
                            break;
                        },
                        // 接收关闭信息
                        _ = shutdown.recv() => {
                            // 清理工作
                            info!("Process resource release shutdown ......");
                            let _ = shutdown_complete.send(()).await;
                            break ;
                        }

                    }
                }

                // 通知连接断开
                sv = servic.ready().await;

                while sv.is_err() {
                    sv = servic.ready().await;
                }

                let ev = NetEvent::Disconnect(conn.clone());
                // let ev = (&conn, NetEvent::Disconnect);
                _ = sv.unwrap().call(ev).await;
            };

            tokio::spawn(async move {
                handle_connection.await;
                
            });
        }
    }

    // #[inline]
    // async fn handle_connection(
    //     mut servic: E,
    //     stream: FB::Framed,
    //     conn: Arc<Connection>,
    //     mut broadcast: BroadcastReceiver,
    //     mut receiver: mpsc::Receiver<Bytes>,
    //     mut shutdown: broadcast::Receiver<()>,
    //     shutdown_complete: mpsc::Sender<()>,
    //     heartbeat_interval: Duration
    // ) {
    // }
}
