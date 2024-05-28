/*
 * @Author: plucky
 * @Date: 2023-11-05 21:04:05
 * @LastEditTime: 2023-11-07 12:20:05
 */
// 根据延时来切换服务的Channel
#![allow(unused)]

use std::{sync::{Arc, atomic::AtomicU64, RwLock}, str::FromStr, thread, task::{Context, Poll}, pin::Pin, time::Duration};

use futures::Future;
use rand::{distributions::WeightedIndex, prelude::Distribution};
use tokio::runtime::Runtime;
use tonic::{transport::{Channel, Endpoint, channel}, body::BoxBody};
use tonic::codegen::http;
use tonic::codegen::Service;

pub enum Change {
    Insert(String, String),
    Remove(String),
}
/// 根据延时来切换服务的Channel
#[derive(Clone)]
pub struct MyChannel {
    rt: Arc<Runtime>,
    chans: Arc<RwLock<Chans>>,
    
    // 用于更新chans
    change_tx: flume::Sender<Change>,
    change_rx: flume::Receiver<Change>,
    
    // 用于选择Channel
    chan_tx: flume::Sender<(Channel, Arc<AtomicU64>)>,
    chan_rx: flume::Receiver<(Channel, Arc<AtomicU64>)>,
}


impl MyChannel{
    pub fn new(rt: Arc<Runtime>) -> Self{
        let chans = Chans::new();
        let (tx, rx) = flume::bounded(1);
        let (chan_tx, chan_rx) = flume::bounded(1);
        
        let this = Self{
            rt,
            chans: Arc::new(RwLock::new(chans)),
            change_tx: tx,
            change_rx: rx,
            chan_tx,
            chan_rx,
        };
        
        this.start_change_loop();
        this.start_update_index_loop();
        this
    }

    pub fn get_change_tx(&self) -> flume::Sender<Change> {
        self.change_tx.clone()
    }

    // 同步的方法连接
    fn connect(rt: &Arc<Runtime>, uri: String, timeout: u64) -> Result<Channel, &str>{
        let (tx, rx) = flume::bounded(1);
        rt.spawn(async move {
            let channel =  Endpoint::from_str(&uri).unwrap()
            .timeout(Duration::from_secs(timeout))
            .connect().await;
            tx.send_async(channel).await.unwrap();
        });
        rx.recv().unwrap().map_err(|_| "connect error")

    }

    /// 在新线程中接收change的消息
    fn start_change_loop(&self) {
        let rt = self.rt.clone();
        let chans = self.chans.clone();
        let change_rx = self.change_rx.clone();
        
        thread::spawn(move || {
            while let Ok(change) = change_rx.recv() {
                match change {
                    Change::Insert(key, uri) => {
                        if let Ok(channel) = MyChannel::connect(&rt, uri.clone(), 3){
                            chans.write().unwrap().add_chan(uri, channel);
                        }
                        else{
                            tracing::info!("connect error: {:?}", uri);
                        }
                    }
                    Change::Remove(key) => {
                        chans.write().unwrap().remove_chan(&key);
                    }
                }
            }
        });
    }

    /// 在新线程里定时更新权重
    fn start_update_index_loop(&self) {
        let chans = self.chans.clone();
        thread::spawn(move || {
            loop {
                chans.write().unwrap().update_index();
                thread::sleep(std::time::Duration::from_secs(3));
            }
        });
    }

}


type BodyRequest = http::Request<BoxBody>;

// 为MyChannel实现Service
impl Service<http::Request<BoxBody>> for MyChannel
 {
    type Response = <Channel as Service<BodyRequest>>::Response;
    type Error = <Channel as Service<BodyRequest>>::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // 从chans中获取一个连接,如果没有则等待
        let channel = self.chans.read().unwrap().pick_chan();
        if let Some(mut channel) = channel {
            // 调用channel的poll_ready
            let p = channel.0.poll_ready(cx);
            if p.is_ready() {
                // 发送channel到chan_tx
                self.chan_tx.send(channel).unwrap();
                
            }
            return p
        }
        // Poll::Pending 需要唤醒
        let waker = cx.waker().clone();
        let rt = self.rt.clone();
        rt.spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            waker.wake();

        });
        Poll::Pending
        
    }

    fn call(&mut self, request: http::Request<BoxBody>) -> Self::Future {
        // 从chan_rx中获取一个channel
        let mut channel = self.chan_rx.recv().unwrap();
        let fut = async move {
            let now = std::time::Instant::now();
            // 调用channel的call
            let resp = channel.0.call(request).await;
            // 记录延迟
            channel.1.store(now.elapsed().as_millis() as u64, std::sync::atomic::Ordering::Relaxed);
            resp
        };
        Box::pin(fut)
    }
}



// key, channel, 权重
type ChannelItem = (String, Channel, Arc<AtomicU64>);

/// Channel的列表
struct Chans {
    chans: Vec<ChannelItem>,
    index: WeightedIndex<f32>,
}

impl Chans {
    fn new() -> Self {
        Self {
            chans: vec![],
            index: WeightedIndex::new(vec![1.0]).unwrap(),
        }
    }
    /// 添加连接
    fn add_chan(&mut self, name: String, channel: Channel) {
        self.chans.push((name, channel, Arc::new(AtomicU64::new(1))));
        self.update_index();
    }

    /// 移除连接
    fn remove_chan(&mut self, name: &String) {
        self.chans.retain(|(n, _, _)| n != name);
        self.update_index();
    }
    
    /// 更新权重
    fn update_index(&mut self) {
        use std::sync::atomic::Ordering;
        if let Ok(index) = WeightedIndex::new(self.chans.iter().map(|(_, _, w)| 1.0 / w.load(Ordering::Relaxed) as f32)){
            self.index = index;
        }
        
    }

    fn pick_chan(&self) -> Option<(Channel, Arc<AtomicU64>)> {
        if self.chans.is_empty() {
            return None;
        }
        // 权重随机选择一个
        let index = self.index.sample(&mut rand::thread_rng());
        let (_, channel, l) = &self.chans[index];
        Some((channel.clone(), l.clone()))
        
    }
}