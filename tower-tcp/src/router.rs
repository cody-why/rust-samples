/*
 * @Author: plucky
 * @Date: 2023-05-31 18:25:08
 * @LastEditTime: 2023-07-04 23:01:35
 * @Description: 
 */

use std::{collections::HashMap, task::{Poll, Context}};
use futures::Future;
use tower::Service;
use crate::{service_fn::ServiceFn, request::Request};


#[derive(Clone)]
pub struct Router<F,R> 
{
    handlers: HashMap<u16, ServiceFn<F,R>>,
}

impl <F,R> Router<F,R> 
where F: Service<R>,R: 
{
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn route(&mut self, msg_id: u16, handler: ServiceFn<F,R>) {
        self.handlers.insert(msg_id, handler);
    }

    pub fn get_handler(&self, msg_id: u16) -> Option<&ServiceFn<F,R>> {
        self.handlers.get(&msg_id)
    }

    pub fn get_handler_mut(&mut self, msg_id: u16) -> Option<&mut ServiceFn<F,R>> {
        self.handlers.get_mut(&msg_id)
    }

    pub fn merge(&mut self, router: Router<F,R>) {
        self.handlers.extend(router.handlers);
    }

}

// 路由实现Service
impl <F,R> Service<R> for Router<F,R>
where F: Service<R>,R: 
{
    type Response = F::Response;
    type Error = F::Error;
    type Future = RouterFuture<F,R>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: R) -> Self::Future {
        RouterFuture {
            // handlers: self.handlers.clone(),
            handlers: HashMap::new(),
            req,
        }
    }


    
}

pub struct RouterFuture<F,R> 
where F: Service<R>,R:
{
    handlers: HashMap<u16, ServiceFn<F,R>>,
    req: R,
}

impl <F,R> Future for RouterFuture<F,R>
where F: Service<R>,R: 
{
    type Output = Result<F::Response, F::Error>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // let handler = self.handlers.get_mut(&self.req.msg_id).unwrap();
        // let res = handler(self.req);
        // Poll::Ready(res)
        // let msg_id = self.req.msg_id;
        
        
        Poll::Pending

    }
}

