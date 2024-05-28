/*
 * @Author: plucky
 * @Date: 2023-06-06 20:40:10
 * @LastEditTime: 2023-06-08 11:32:03
 * @Description: 
 */
#![allow(dead_code)]
use std::{task::{Context, Poll}, convert::Infallible};

use bytes::{Buf, Bytes};
// use hyper::body::HttpBody;

#[derive(Debug)]
pub struct Request<R>{
    pub msg_id: u16,
    pub body: R,
}

impl <R>  Request<R> {
    pub fn new(body: R) -> Self {
        Request {
            msg_id: 0,
            body 
        }
    }
}
 
#[derive(Debug)]
pub struct Response<R>{
    pub body: R,
}

impl <R>  Response<R> {
    pub fn new(body: R) -> Self {
        Response { body }
    }
}

// hyper::body::HttpBody

pub trait Body{
    type Data: Buf;
    type Error;
    fn poll_data(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>>;
}

impl Body for String {
    type Data = Bytes;
    type Error = Infallible;
    fn poll_data(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>>{
        if !self.is_empty() {
            let s = std::mem::take(&mut *self);
            Poll::Ready(Some(Ok(s.into_bytes().into())))
        } else {
            Poll::Ready(None)
        }
    }
}

impl Body for Bytes {
    type Data = Bytes;
    type Error = Infallible;
    fn poll_data(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>>{
        if !self.is_empty() {
            let s = std::mem::take(&mut *self);
            Poll::Ready(Some(Ok(s.into())))
        } else {
            Poll::Ready(None)
        }
    }
}