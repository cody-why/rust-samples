/*
 * @Author: plucky
 * @Date: 2023-07-06 18:13:44
 * @LastEditTime: 2023-07-07 09:43:06
 */

use std::{time::{Instant, Duration}, pin::Pin, task::{Context, Poll}};

use futures::Future;
use pin_project::pin_project;

/// 一个计算异步函数用时的实现
#[pin_project]
pub struct TimedWrapper<Fut: Future> {
    start: Option<Instant>,
    // 需要固定的引用,into pinned references
    #[pin]
    future: Fut,
}

impl<Fut: Future> TimedWrapper<Fut> {
    pub fn new(future: Fut) -> Self {
        Self {
            start: None,
            future,
        }
    }
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
    // 返回包装异步函数的结果和用时
    type Output = (Fut::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        let start = this.start.get_or_insert_with(Instant::now);

        let inner_poll = this.future.as_mut().poll(cx);
        match inner_poll {
            // 还没完成,还需要更多时间
            Poll::Pending => Poll::Pending,
            // Success!
            Poll::Ready(output) => Poll::Ready((output, start.elapsed())),
        }
    }
    
}

#[tokio::main]
async fn main(){
    let async_fn = |n:u64|async move{
        tokio::time::sleep(Duration::from_secs(n)).await;
        println!("hello");

        "world".to_string()
    };

    // 将异步函数包装在我的包装器中。
    let timed_async_fn = TimedWrapper::new(async_fn(1));

    // 调用 async 函数，并为其计时。
    let (resp, time) = timed_async_fn.await;
    println!("Got {} in {}ms", resp, time.as_millis());



}
