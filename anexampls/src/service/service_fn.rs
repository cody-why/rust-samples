/*
 * @Author: plucky
 * @Date: 2023-07-18 21:40:15
 * @LastEditTime: 2023-07-18 21:40:40
 */

use futures::Future;
use std::fmt;

pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    
    fn call(&mut self, args: Request) -> Self::Future;
}

pub fn service_fn<T>(f: T) -> ServiceFn<T> {
    ServiceFn { f }
}

#[derive(Copy, Clone)]
pub struct ServiceFn<T> {
    f: T,
}

impl<T, F, Request, R, E> Service<Request> for ServiceFn<T>
where
    T: FnMut(Request) -> F,
    F: Future<Output = Result<R, E>>,
{
    type Response = R;
    type Error = E;
    type Future = F;

   
    fn call(&mut self, req: Request) -> Self::Future {
        (self.f)(req)
    }
}

impl<T> fmt::Debug for ServiceFn<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServiceFn")
            .field("f", &format_args!("{}", std::any::type_name::<T>()))
            .finish()
    }
}