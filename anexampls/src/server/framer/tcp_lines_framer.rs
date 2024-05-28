/*
 * @Author: plucky
 * @Date: 2023-07-12 21:25:00
 * @LastEditTime: 2023-07-17 13:14:29
 */

use std::{ io, pin::Pin, task::{ Context, Poll } };
use bytes::{ BytesMut, Bytes };
use futures::{ Stream, Sink, StreamExt, SinkExt };
use tokio::net::TcpStream;
use tokio_util::codec::{ Framed, LinesCodec };

use super::{ Communication, Framer, FramerBuilder };

pub struct TcpLinesFramer {
    inner: Communication<Framed<TcpStream, LinesCodec>, String>,
}

impl TcpLinesFramer {

    pub fn new(stream: TcpStream) -> Self {
        let framed = Framed::new(stream, LinesCodec::new());
        let inner = Communication::new(framed);
        TcpLinesFramer { inner }
    }

    pub fn builder() -> LinesBuilder {
        LinesBuilder::new()
    }

}

pub struct LinesBuilder {
    pub max_length: usize,
}

impl FramerBuilder for LinesBuilder {
    type Stream = TcpStream;
    type Framed = TcpLinesFramer;

    fn build(&self, stream: Self::Stream)-> Result<Self::Framed, io::Error> {
        let code = LinesCodec::new_with_max_length(self.max_length);
        let framed: Framed<TcpStream, LinesCodec> = Framed::new(stream, code);
        let inner = Communication::new(framed);
        Ok(TcpLinesFramer { inner })
    }
}

impl LinesBuilder {
    pub fn new() -> Self {
        LinesBuilder { max_length: 8 * 1024 * 1024 }
    }

    pub fn max_length(mut self, val: usize) -> Self {
        self.max_length = val;
        self
    }
}

impl Framer for TcpLinesFramer {}


impl Stream for TcpLinesFramer {
    type Item = Result<BytesMut, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // 要把String转成BytesMut
        self.inner.reader
            .poll_next_unpin(cx)
            .map_ok(|item| { BytesMut::from(item.as_bytes()) })
            .map_err(|e| { io::Error::new(io::ErrorKind::Other, e.to_string()) })
    }
}


impl Sink<Bytes> for TcpLinesFramer {
    type Error = io::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.writer
            .poll_ready_unpin(cx)
            .map_err(|e| { io::Error::new(io::ErrorKind::Other, e.to_string()) })
    }

    fn start_send(mut self: Pin<&mut Self>, item: Bytes) -> Result<(), Self::Error> {
        // 要把Bytes转成String
        let item = String::from_utf8_lossy(&item).to_string();
        self.inner.writer
            .start_send_unpin(item)
            .map_err(|e| { io::Error::new(io::ErrorKind::Other, e.to_string()) })
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.writer
            .poll_flush_unpin(cx)
            .map_err(|e| { io::Error::new(io::ErrorKind::Other, e.to_string()) })
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.writer
            .poll_close_unpin(cx)
            .map_err(|e| { io::Error::new(io::ErrorKind::Other, e.to_string()) })
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tcp() {
        let addr = "127.0.0.1:3000";
        let stream = TcpStream::connect(addr).await.unwrap();
        let mut tcp_communication = TcpLinesFramer::builder().build(stream).unwrap();
        tcp_communication.send(Bytes::from("hello")).await.unwrap();
        while let Some(data) = tcp_communication.next().await {
            println!("data: {:?}", data);
        }
    }
}
