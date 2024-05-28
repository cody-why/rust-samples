/*
 * @Author: plucky
 * @Date: 2023-06-02 21:28:34
 * @LastEditTime: 2023-06-18 17:27:00
 * @Description: 
 */
// #![feature(type_alias_impl_trait)]

use futures::future::Future;
use pin_project_lite::pin_project;
use std::fmt::Debug;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Service, ServiceExt};


 // 定义一个服务
#[derive(Clone)]
struct MyService{
}

// type F = impl Future<Output = Result<String, ()>> + Send + 'static;
impl Service<&'static str> for MyService
{
    type Response = String;
    type Error = ();
    // type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    type Future = MyResponseFuture<MyFuture>;

    
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
     
    fn call(&mut self, request: &'static str) -> Self::Future {
        // let response = async move{
        //     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        //     Ok(format!("Hello, {}!", request))
        // };
        // Box::pin
        MyResponseFuture {
            response: MyFuture(request)
        }
    }
}
 
 // 定义一个包含Service的结构体
struct MyStruct<S>
{
    service: S,
    name: &'static str,
    // _phantom: std::marker::PhantomData<T>,
}

impl<S> MyStruct<S>

where S: Service<&'static str>, S::Response:Debug, S::Error:Debug

{
    fn new(service: S, name: &'static str) -> Self {
        Self {
            service,
            name,
            // _phantom: std::marker::PhantomData,
        }
    }
    async fn my_async_method(mut self) -> S::Response {
        
        self.service.ready().await.unwrap().call(self.name).await.unwrap()
    }
}


#[tokio::main]
async fn main() {
    let service = MyService {};
    let name = "John";
    let service = LogService::new(service);
    let my_struct = MyStruct::new(service, name);
    let response = my_struct.my_async_method().await;
    println!("{:?}", response);
}


// 定义一个异步future
struct MyFuture(&'static str);

impl Future for MyFuture {
    type Output = Result<String, ()>;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        std::thread::sleep(std::time::Duration::from_secs(1));
        Poll::Ready(Ok(format!("Hello, {}!", self.0)))
    }
}

// 仿照tower的timeout实现一个异步服务future
pin_project! {
    struct MyResponseFuture<F> {
        #[pin]
        response: F,
    }
}

impl<F> Future for MyResponseFuture<F>
where
    F: Future,
{
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.response.poll(cx)
    }
}


/// 一个简单的日志服务
struct LogService<S> {
    inner: S,
}

impl<S> LogService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S,T> Service<T> for LogService<S>
where S: Service<T>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LogFuture<S::Future>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(_cx)
    }

    fn call(&mut self, request: T) -> Self::Future {
        println!("LogService call start");
        LogFuture::new(self.inner.call(request))
    }
}

// 日志服务的异步future
pin_project! {
    struct LogFuture<F> {
        #[pin]
        future: F,
        start: std::time::Instant,
    }
}

impl<F> LogFuture<F> {
    pub fn new(future: F) -> Self {
        Self {
            future,
            start: std::time::Instant::now(),
        }
    }
}

impl <F> Future for LogFuture<F>
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
        let duration = this.start.elapsed();
        println!("LogService call end {duration:?}");
        Poll::Ready(res)
    }
}
    
