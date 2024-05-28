/*
 * @Author: plucky
 * @Date: 2023-06-04 19:16:46
 * @LastEditTime: 2023-06-04 19:17:49
 * @Description: 
 */

use std::{time::Duration, pin::Pin, task::{Context, Poll}};

use futures::Future;
use pin_project_lite::pin_project;
use tokio::time::Sleep;
use tower::{Service, BoxError};


/// tower的timeout源码, 里面用到了pin_project宏,实现了Future的pin, 比box 更加高效, 有比较好的学习价值
#[derive(Debug, Clone)]
pub struct Timeout<T> {
    // 包装的服务
    inner: T,
    timeout: Duration,
}

// ===== impl Timeout =====
impl<T> Timeout<T> {
    /// Creates a new [`Timeout`]
    pub fn new(inner: T, timeout: Duration) -> Self {
        Timeout { inner, timeout }
    }

    /// Get a reference to the inner service
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the inner service
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consume `self`, returning the inner service
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => Poll::Ready(r.map_err(Into::into)),
            // Poll::Ready(r) => Poll::Ready(r.map_err(|e|e.into())),
        }
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let response = self.inner.call(request);
        let sleep = tokio::time::sleep(self.timeout);
        // 这里返回一个Future
        ResponseFuture::new(response, sleep)
    }
}

pin_project! {
    /// [`Timeout`] response future
    #[derive(Debug)]
    pub struct ResponseFuture<T> {
        #[pin]
        response: T,
        #[pin]
        sleep: Sleep,
    }
}

impl<T> ResponseFuture<T> {
    pub(crate) fn new(response: T, sleep: Sleep) -> Self {
        ResponseFuture { response, sleep }
    }
}

impl<F, T, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: Into<BoxError>,
{
    type Output = Result<T, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        // First, try polling the future
        match this.response.poll(cx) {
            Poll::Ready(v) => return Poll::Ready(v.map_err(Into::into)),
            Poll::Pending => {}
        }

        // Now check the sleep
        match this.sleep.poll(cx) {
            Poll::Pending => Poll::Pending,
            // 如果sleep完成了, 说明超时了
            Poll::Ready(_) => Poll::Ready(Err("request timed out".into())),
        }
    }
}

#[tokio::main]
async fn main() {
    
}