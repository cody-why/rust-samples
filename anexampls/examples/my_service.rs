/*
 * @Author: plucky
 * @Date: 2023-07-04 12:11:46
 * @LastEditTime: 2023-07-08 18:33:41
 */

use std::{marker::PhantomData, borrow::BorrowMut};
use futures::{Future, future::BoxFuture};

#[tokio::main]
async fn main(){
    let mut myservice = MyService::new();
    myservice.set_cb(|req|async move{
        println!("req: {}", req);
    });
    myservice.execute("hello").await;
    
   
    myservice.execute("world").await;
}

trait CmdService<F, Fut> {
    // type req: ToString;
    fn set_cb(&mut self, f: F);
}

// 用callback封装Future
struct MyService<F, Fut, T>{
    id: u64,
    f: Option<F>,
    p: PhantomData<(Fut, T)>
}

impl <F, Fut,T> MyService<F, Fut,T>
    where F: Fn(T) -> Fut, Fut: Future<Output = ()>
{
    fn set_cb(&mut self, f: F){
        self.f = Some(f);
    }

    pub fn new() -> Self {
        Self {
            id: 0,
            f: None,
            p: PhantomData,
        }
    }

    pub async fn execute(&mut self, req: T) 
    {
        if let Some(f) = self.f.borrow_mut(){
            self.id += 1;
            println!("execute id: {}", self.id);
            f(req).await;
        }
    }

}



/// 用BoxFuture封装Future
trait AsyncFn {
    fn call2(&self, args: u8) -> BoxFuture<'static, u8>;
}

impl<T, F> AsyncFn for T
where
    T: Fn(u8) -> F,
    F: Future<Output = u8> + 'static + Send,
{
    fn call2(&self, args: u8) -> BoxFuture<'static, u8> {
        Box::pin(self(args))
    }
}

#[allow(unused)]
struct S {
    foo: Box<dyn AsyncFn>,
}
#[allow(unused)]
impl S {
    pub fn new(f: impl AsyncFn + 'static) -> Self {
        Self {
            foo: Box::new(f),
        }
    }
    
}

#[tokio::test]
async fn feature() {
    async fn foo(x: u8) -> u8 {
        x * 2
    }
    let s = S::new(foo);
    let r = s.foo.call2(12).await; // => 24
    println!("{}", r);
}


/// 用service封装Future
// 1. service_fn创建ServiceFn
// 2. ServiceFn实现了AnService
// 3. ServiceFn的call方法调用异步函数,返回Future
pub fn service_fn<F, R, S>(f: F) -> ServiceFn<F, R>
where
    F: FnMut(R) -> S,
    S: Future,
{
    ServiceFn {
        f,
        _req: PhantomData,
    }
}


pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    
    fn call(&mut self, args: Request) -> Self::Future;
}

pub struct ServiceFn<F, Req>

{
    f: F,
    _req: PhantomData<fn(Req)>,
}

impl <F, Req> ServiceFn<F, Req> 

{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _req: PhantomData,
        }
    }
}
    

impl<F, Req, Ret, Res, E> Service<Req>
    for ServiceFn<F, Req>
where
    F: FnMut(Req) -> Ret,
    Req: Body,
    Ret: Future<Output = Result<Res, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
    Res: Body,
{
    type Response = Res;
    type Error = E;
    type Future = Ret;

    fn call(&mut self, req: Req) -> Self::Future {
        (self.f)(req)
    }
}


impl<F, R> Clone for ServiceFn<F, R>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        ServiceFn {
            f: self.f.clone(),
            _req: PhantomData,
        }
    }
}

impl<F, R> Copy for ServiceFn<F, R> where F: Copy {}



pub trait Body {
    type Data;
    fn data(&mut self) -> &mut Self::Data;
       
}

impl Body for String {
    type Data = String;
    fn data(&mut self) -> &mut Self::Data {
        self
    }
}


#[tokio::test]
async fn feature1() {
    let mut s = service_fn(|req| async move {
        println!("req: {}", req);
        Ok("world".to_string())
    });
    let req = "hello".to_string();

    let r: Result<String, String> = s.call(req).await;
    println!("{:?}", r);

    let s = ServiceFn::new(|req| async move {
        println!("req: {}", req);
        let r: Result<String, String> =Ok("world".to_string());
        r
    });
    
    let mut server = MyServer::<_, String>::new(s);
    server.execute("good".to_string()).await;
    
}


// 封装一个struct, 功能是可以设置Service, 并且可以执行Service,通过ServiceFn封装,可以配置任何类型的参数的Service
pub struct MyServer<F,Req>
{
    id: u64,
    f: Option<F>,
    p: PhantomData<Req>,
}

impl <F,Req> MyServer<F,Req>
    where F: Service<Req>, F::Future: futures::Future
{
    pub fn new(f: F) -> Self {
        Self {
            id: 0,
            f: Some(f),
            p: PhantomData,
        }
    }


    pub async fn execute(&mut self, req: Req)
    {
        println!("execute id: {}", self.id);
        if let Some(f) = self.f.as_mut(){
            self.id += 1;
            println!("execute id: {}", self.id);
            let _ = f.call(req).await;
        }
    }

    pub async fn receive_some(&mut self)
    {
        // let req = "hello".to_string();
        // self.execute(req).await;
    }
    
}
