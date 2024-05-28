/*
 * @Author: plucky
 * @Date: 2023-05-06 17:14:53
 * @LastEditTime: 2023-06-06 20:24:50
 * @Description:
 */

use std::{ future, task::{ Context, Poll }, pin::Pin};
use bytes::{ Bytes};
use futures::Future;
use pin_project_lite::pin_project;
use tower::Service;



use crate::router::Router;

#[derive(Clone)]
pub struct MyService
{
    times: usize,
    router: Option<Router>,
}
    
impl MyService {
        pub fn new() -> Self {
            Self {
                times: 0,
                router: None,
            }
        }
    
        pub fn set_router(&mut self, router: Router)-> &mut Self {
            self.router = Some(router);
            self
        }

        async fn dispatch(&self, message_id: u16, message: Bytes) -> Option<Bytes> {
            if let Some(router) = &self.router {
                router.dispatch(message_id, message).await
            } else {
                None
            }
        }
    }

impl Service<Bytes> for MyService {
    type Response = Bytes;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    // type Future = future::Ready<Result<Self::Response, Self::Error>>;
    type Future = MyFuture<>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Bytes) -> Self::Future {
        self.times += 1;
        println!("times: {}, req: {}", self.times, String::from_utf8_lossy(&req));
        // if req.len() < 2 {
        //     return future::ready(Ok(Bytes::new()));
        // }
        // let resp = {
        //     let message_id = u16::from_be_bytes([req[0], req[1]]);
        //     let message = req.slice(2..);
        //     if let Some(resp) = self.dispatch(message_id, message).await {
        //         Ok(resp)
        //     } else {
        //         Err(format!("no handler: {message_id}") .into())
        //     }
        // };
        // future::ready(resp)
        
        let asy = async move {
            Ok(Bytes::new())
        };
        MyFuture {
            future: asy,
        }
    }
}

pin_project! {
    struct MyFuture<F> {
        #[pin]
        future: F,
    }
}

impl <F> Future for MyFuture<F>
where F: Future,
{
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let result = this.future.poll(cx);
        let res = match result {
            Poll::Ready(res) => res,
            Poll::Pending => return Poll::Pending,
        };
        // let duration = this.start.elapsed();
        // println!("LogService call end {duration:?}");
        Poll::Ready(res)
    }
}
    