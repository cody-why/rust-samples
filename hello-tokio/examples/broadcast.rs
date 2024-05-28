/*
 * @Author: plucky
 * @Date: 2023-03-16 19:03:10
 * @LastEditTime: 2023-03-18 10:49:54
 * @Description: 
 */

use std::{sync::{Arc, atomic::AtomicBool}};

use dashmap::DashMap;
use tokio::sync::{broadcast, mpsc};

#[tokio::main]
async fn main(){
    let (tx, mut rx) = broadcast::channel(32);
    
    let is_exit = Arc::new(AtomicBool::new(false));
    let is_exit2 = is_exit.clone();
    let tx2 = tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            tx2.send(1).unwrap();
            if is_exit.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
        }
    });

    tokio::spawn(async move {
        loop {
            let recv = rx.recv().await;
            println!("recv = {:?}", recv);
            if recv.is_err() {
                break;
            }
        }
    });

    is_exit2.store(true, std::sync::atomic::Ordering::Relaxed);
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    
    

}

// 封装一个简单的广播类,所有的订阅者都可以收到消息
pub struct Broadcaster<T> 
    where T: Clone+Send+ 'static
{
    tx: broadcast::Sender<T>,
    // member: Arc<Mutex<Vec<mpsc::Sender<T>>>> ,
    member: Arc<DashMap<usize,Vec<mpsc::Sender<T>>>>,
}

impl<T> Broadcaster<T>  
    where T: Clone+Send+ 'static
{
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(32);
        Self { tx, member: Arc::new(DashMap::new())}

    }
    pub fn add(&self, tx: mpsc::Sender<T>){
        self.member.entry(0).or_insert_with(||Vec::new()).push(tx);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<T> {
        self.tx.subscribe()
    }

    pub fn send(&self, msg: T) -> Result<usize, broadcast::error::SendError<T>> {
        self.tx.send(msg)
    }

    pub fn run(&self) {
        let mut rx = self.tx.subscribe();
        let member = self.member.clone();
        tokio::spawn(async move {
            loop {
                let recv = rx.recv().await;
                if recv.is_err() {
                    println!("Broadcaster recv error = {:?}", recv.err());
                    break;
                }
                let recv = recv.unwrap();
                if let Some(member) = member.get(&0) {
                    for tx in member.iter() {
                        let msg = recv.clone();
                        tx.send(msg).await.unwrap_or_else(|e| {
                            println!("Broadcaster send error = {}", e);
                        });
                    }
                }
                
            }
        });
    }
}


#[cfg(test)]
mod tests{

    use std::time::Duration;

    use super::*;
    use tokio::{time, sync::{mpsc, oneshot}};


    #[tokio::test]
    async fn feature() {
        let (tx, mut rx) = mpsc::channel(32);
        tokio::spawn(async move {
            while let Some(recv) = rx.recv().await {
                println!("got = {:?}", recv);
            }
            println!("exit");
            
        });
        {
            let broadcaster = Broadcaster::new();
            broadcaster.run();

            broadcaster.add(tx);
            broadcaster.send((1,"abc")).unwrap();
            broadcaster.send((2,"abc")).unwrap();
        }

        time::sleep(Duration::from_secs(3)).await
    }

    #[tokio::test]
    async fn test_time() {
        let (tx, rx) = oneshot::channel();
        tx.send(1).unwrap();
        //超时2秒如果没有收到消息则返回错误
        let v = time::timeout(Duration::from_millis(2000), async move{
            time::sleep(Duration::from_secs(1)).await;
            rx.await.unwrap()
        }).await;

        if v.is_err() {
            println!("did not receive value timeout");
        } else {
            println!("got = {:?}", v.unwrap());
        }
        
        // interval对比sleep,每次间隔不包括循环中的sleep时间
        let mut interval = time::interval(Duration::from_secs(1));

        for _i in 0..3 {
            interval.tick().await;
            
            println!("hello {_i}");
            time::sleep(Duration::from_secs(1)).await
        }

    }
    
}