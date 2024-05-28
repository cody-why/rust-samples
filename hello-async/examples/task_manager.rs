/*
 * @Author: plucky
 * @Date: 2023-07-03 08:07:55
 * @LastEditTime: 2023-07-03 16:17:18
 */

#![allow(unused)]

use tokio::sync::broadcast;
use tokio::time::{Duration, self};
use tokio::task::JoinHandle;
use std::future::Future;

pub struct TaskManager {
    singler: broadcast::Sender<()>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { 
            singler: broadcast::channel(1).0,
        }
    }

    pub fn add_interval_task<F,Fut>(&mut self, duration: Duration, task: F)-> JoinHandle<()>
        where F: Clone+Send+Sync+'static + Fn() -> Fut,
            Fut: Future<Output = ()> + Send,
    {
        let mut singler = self.singler.subscribe();
        tokio::spawn(async move {
            let mut interval = time::interval(duration);
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                         task().await;
                    },
                    _ = singler.recv() => {
                        break ;
                    },
                }
            }
           println!("任务执行完毕");
        })
        
    }

    pub async fn stop_all(&mut self) {
        self.singler.send(());
    }
    
}

#[tokio::main]
async fn main() {
    let mut task_manager = TaskManager::new();
    let handle = || async move {
        println!("间隔任务执行");
    };
    let t = task_manager.add_interval_task(Duration::from_secs(1), handle);
    // t.abort();
    // 等待一段时间后停止任务
    time::sleep(Duration::from_secs(3)).await;
    // task_manager.stop_all().await;
    drop(task_manager);
    time::sleep(Duration::from_secs(1)).await;

}


async fn my_async_fn() {
    println!("异步函数执行");
}

async fn test_async() {
    let my_future = MyFuture::new(my_async_fn);
    my_future.call().await;
    let my_future = MyFuture::new(|| async move {
        println!("异步函数2执行");
    });
    my_future.call().await;
}

struct MyFuture<F> {
    future: F,
}

impl <F, Fut> MyFuture<F>
    where F: Clone + Fn() -> Fut,
        Fut: Future<Output = ()>,
{
    fn new(future: F) -> Self {
        MyFuture { future }
    }

    async fn call(&self){
        (self.future)().await;
    }
    
}