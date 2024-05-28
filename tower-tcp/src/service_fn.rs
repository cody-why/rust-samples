/*
 * @Author: plucky
 * @Date: 2023-06-06 20:38:29
 * @LastEditTime: 2023-06-08 11:33:03
 * @Description: 
 */
#![allow(dead_code)]
use std::{error::Error as StdError, task::Context};
use std::fmt;
use std::marker::PhantomData;
use std::task::Poll;

use futures::Future;
use tower::Service;

use crate::request::{Request, Response, Body};


/// Create a `Service` from a function.
///
/// # Example
///
/// ```
/// let service = service_fn(|req: Request<Body>| async move {
///     if req.version() == Version::HTTP_11 {
///         Ok(Response::new(Body::from("Hello World")))
///     } else {
///         Err("not HTTP/1.1, abort connection")
///     }
/// });
/// ```
pub fn service_fn<F, R, S>(f: F) -> ServiceFn<F, R>
where
    F: FnMut(Request<R>) -> S,
    S: Future,
{
    ServiceFn {
        f,
        _req: PhantomData,
    }
}

/// Service returned by [`service_fn`]
pub struct ServiceFn<F, R> {
    f: F,
    _req: PhantomData<fn(R)>,
}

impl<F, ReqBody, Ret, ResBody, E> Service<Request<ReqBody>>
    for ServiceFn<F, ReqBody>
where
    F: FnMut(Request<ReqBody>) -> Ret,
    ReqBody: Body,
    Ret: Future<Output = Result<Response<ResBody>, E>>,
    E: Into<Box<dyn StdError + Send + Sync>>,
    ResBody: Body,
{
    type Response = Response<ResBody>;
    type Error = E;
    type Future = Ret;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        (self.f)(req)
    }
}

impl<F, R> fmt::Debug for ServiceFn<F, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("impl Service").finish()
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


#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_service_fn() {
        
        let mut service = service_fn(|_req: Request<String>| async move {
            if 1==1 {
                let body = "Hello World".to_string();
                Ok(Response::new(body))
            }else {
                Err("not support")
            }
            
        });
        let body = "Hello World".to_string();
        let req = Request::new(body);
        let resp = service.call(req).await;
        println!("{:?}", resp);


    }
}