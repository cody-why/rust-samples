/*
 * @Author: plucky
 * @Date: 2023-07-11 23:28:15
 * @LastEditTime: 2023-07-17 18:19:43
 */

// #![allow(dead_code)]

use std::io;
use bytes::{ Bytes, BytesMut };
use futures::{ stream::{ SplitSink, SplitStream }, Sink, SinkExt, Stream, StreamExt };
use tokio::io::{AsyncRead, AsyncWrite};

mod tcp_framer;
mod ws_framer;
mod tcp_lines_framer;
mod udp_framer;

pub use tcp_framer::*;
pub use ws_framer::*;
pub use tcp_lines_framer::*;

/// 分帧器
pub trait Framer: Stream<Item = Result<BytesMut, io::Error>> + Sink<Bytes, Error = io::Error> +Unpin+Send
{
}

/// 分帧器构造者
pub trait FramerBuilder
{
    type Stream: AsyncRead + AsyncWrite;
    type Framed: Framer;
    fn build(&self, stream: Self::Stream)-> Result<Self::Framed, io::Error>;
}

/// 通讯者
pub struct Communication<S, Item> {
    // pub stream: S,
    pub reader: SplitStream<S>,
    pub writer: SplitSink<S, Item>,
    // _item: PhantomData<Item>,
}

impl<S, Item> Communication<S, Item>
    where
        S: StreamExt + SinkExt<Item> //+ StreamCodec<Resp = Item, Req = <S as Stream>::Item>
{
    pub fn new(inner: S) -> Self {
        let (writer, reader) = inner.split();
        Communication { reader, writer}
    }

    //  pub async fn next(&mut self) -> Option<BytesMut> {
    //      let data = self.reader.next().await;
    //      match data {
    //          Some(data) => self.reader.decode(data),
    //          None => None,
    //      }
    //  }

    //  pub async fn send(&mut self, data: BytesMut) -> Result<(), MyError>
    //  where <S as futures::Sink<Item>>::Error: std::fmt::Display
    //  {
    //      self.writer.send(self.reader.encode(data)).await.map_err(|e| MyError::Other(e.to_string()))?;
    //      Ok(())
    //  }
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpStream;
    use tokio_util::codec::{ Framed, LengthDelimitedCodec };

    use super::*;

    #[tokio::test]
    async fn test_communication() {
        let use_websocket = false;

        let addr = "127.0.0.1:3000";
        let stream = TcpStream::connect(addr).await.unwrap();

        if use_websocket {
            let (ws_stream, _) = tokio_tungstenite::client_async(addr, stream).await.unwrap();
            let _c = Communication::new(ws_stream);
        } else {
            let framed = Framed::new(stream, LengthDelimitedCodec::new());
            let _c = Communication::new(framed);
        }
    }

}
