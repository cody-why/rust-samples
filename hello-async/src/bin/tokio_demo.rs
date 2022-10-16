/***
 * @Author: plucky
 * @Date: 2022-06-26 11:13:47
 * @LastEditTime: 2022-06-26 13:24:03
 * @Description: tokio 异步的例子
 */

use std::{task::Poll, time::Duration, thread};

use futures::Future;
use tokio::{self};

// 假如这个是读文件的异步
struct ReadFileFuture {
    times:u32,
}
impl ReadFileFuture {
    fn new(t:u32) -> Self {
        assert!(t>0);
        ReadFileFuture {times:t}
    }
}

// 实现了Future,就可以异步调用啦
impl Future for ReadFileFuture {
    type Output = String;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        
        println!("tokio 调用 poll,times:{}",self.times);
        if self.times<1{
            // 返回结果,表示任务完成了
            return Poll::Ready("超时了".to_string())
        }
        self.times-=1;
        thread::sleep(Duration::new(1, 0));
        
        // 把他放入waker,下次poll
        cx.waker().wake_by_ref();
        // 返回Pending,表示任务没有完成
        Poll::Pending
    }
}


#[tokio::main]
async fn main() {
    let h1 = tokio::spawn(async { ReadFileFuture::new(5).await });
    let h2 = tokio::spawn(read_file());
    if let (Result::Ok(a),Result::Ok(b)) = tokio::join!(h1,h2) {
        print!("{},{}", a,b);
    }
}

#[allow(dead_code)]
async fn read_file() -> String {
    thread::sleep(Duration::new(1, 0));
    println!("Processing file 1");
    String::from("read_file_ok")
}
