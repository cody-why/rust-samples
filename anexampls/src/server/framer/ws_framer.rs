/*
 * @Author: plucky
 * @Date: 2023-07-12 21:33:17
 * @LastEditTime: 2023-07-17 13:18:16
 */

use super::{ Communication, Framer, FramerBuilder };
use bytes::{ Bytes, BytesMut };
use futures::{ Sink, SinkExt, Stream, StreamExt };
use std::{ io, pin::Pin, task::{ Context, Poll } };
use tokio::net::TcpStream;
use tokio_tungstenite::{ tungstenite::Message, WebSocketStream, accept_async };

pub struct WsFramer {
    inner: Communication<WebSocketStream<TcpStream>, Message>,
}

impl WsFramer {
    
    pub fn new(stream: WebSocketStream<TcpStream>) -> Self {
        let inner = Communication::new(stream);
        WsFramer { inner }
    }
    
    pub fn builder() -> Builder {
        Builder{}
    }

}
pub struct Builder;

impl FramerBuilder for Builder {
    type Stream = TcpStream;
    type Framed = WsFramer;

    fn build(&self, stream: Self::Stream)-> Result<Self::Framed, io::Error> {
        // 同步运行异步代码
        let stream=futures::executor::block_on(
            async move {
                accept_async(stream).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
            })?;
        // let h = tokio::runtime::Handle::current();
        // let  t =    std::thread::spawn(move || {
        // let stream = h.block_on(
        //         async move {
        //             accept_async(stream).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        //         }
        //     );
        //     stream
        // });
        // let stream = t.join().unwrap()?;
        let inner = Communication::new(stream);
        Ok(WsFramer { inner })
    }
}

impl Framer for WsFramer {}

impl Stream for WsFramer {
    type Item = Result<BytesMut, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let resp = self.inner.reader.poll_next_unpin(cx);
        // 把Message转换成BytesMut,Error转换成io::Error
        resp.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string())).map_ok(|msg|
            BytesMut::from(msg.into_data().as_slice())
        )
    }
}


impl Sink<Bytes> for WsFramer {
    type Error = io::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.writer
            .poll_ready_unpin(cx)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn start_send(mut self: Pin<&mut Self>, item: Bytes) -> Result<(), Self::Error> {
        // 把Bytes转换成Message
        self.inner.writer
            .start_send_unpin(Message::Binary(item.to_vec()))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.writer
            .poll_flush_unpin(cx)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.writer
            .poll_close_unpin(cx)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_ws() {
        let addr = "127.0.0.1:3000";
        let stream = TcpStream::connect(addr).await.unwrap();
        let (ws_stream, _) = tokio_tungstenite::client_async(format!("ws://{}", addr), stream).await.unwrap();

        let mut framer = WsFramer::new(ws_stream);
        
        framer.send(Bytes::from("hello")).await.unwrap();
        while let Some(data) = framer.next().await {
            println!("recv data: {:?}", data);
        }
    }
}
