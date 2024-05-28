/*
 * @Author: plucky
 * @Date: 2023-05-14 09:58:41
 * @LastEditTime: 2023-06-06 20:02:05
 * @Description: 
 */

use bytes::{BytesMut};
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use tower::{ServiceBuilder, ServiceExt, Service};

use crate::service::MyService;

pub struct Server {
    addr: String,
    // listener: TcpListener,
}

impl Server {

    pub fn bind(addr: impl Into<String>) -> Self {
        let addr = addr.into();
        Self {
            addr,
        }
    }
    
    pub async fn serve(&mut self, router: Router) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Listening on: {}", self.addr);

        //let service_maker = ServiceBuilder::new().buffer(100).service(my_service);
        let mut my_service = MyService::new();
        my_service.set_router(router);

        loop {
            let (mut socket, _) = listener.accept().await?;
            let service = ServiceBuilder::new().buffer(10).service(my_service.clone());
            let addr = socket.peer_addr().unwrap();
            tokio::spawn(async move {
                println!("accept: {:?}", addr);
                let (mut reader, mut writer) = socket.split();
                let size = 1024;
                let buf = BytesMut::with_capacity(size);
                loop {
                    let mut buf = buf.clone();
                    match reader.read_buf(&mut buf).await {
                        Ok(n) if n == 0 => {
                            break;
                        }
                        Ok(_) => {
                            let mut s = service.clone();
                            let ready = s.ready().await;
                            if ready.is_err() {
                                println!("ready error: {:?}", ready.err());
                                break;
                            }
                            
                            let resp = ready.unwrap().call(buf.freeze()).await;
                            match resp {
                                Ok(resp) => {
                                    writer.write_all(&resp).await.ok();
                                }
                                Err(_e) => {
                                    println!("error: {:?}", _e);
                                    break;}
                            }
                        }
                        Err(_e) => {break;}
                    }
                }
                println!("close: {:?}", addr);
            });
        }
    }
    
}