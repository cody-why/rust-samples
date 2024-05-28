/*
 * @Author: plucky
 * @Date: 2023-07-11 20:41:37
 * @LastEditTime: 2023-07-12 01:16:56
 */

use bytes::{BytesMut, Bytes};
use futures::stream::SplitStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use tokio_util::codec::Framed;


/// 把流统一编码为Bytes
pub trait StreamCodec{
    /// Stream::Item, 接收流的Item
    type Req;
    /// Sink::Item, 发送流的Item
    type Resp;

    /// 解码,把接收流的Item转换为消息
    fn decode(&self, data: Self::Req) -> Option<BytesMut>;
    /// 编码,把消息转换为发送流的Item
    fn encode(&self, data: BytesMut) -> Self::Resp;
}

// /// 为TcpStream实现解码器
// impl<T, U> StreamCodec for Framed<T, U> {
//     type Req = Result<BytesMut, std::io::Error>;
//     type Resp = Bytes;
    
//     fn decode(&self, data: Self::Req) -> Option<BytesMut> {
//         match data {
//             Ok(data) => Some(data),
//             _ => None,
//         }
//     }

//     fn encode(&self, data: BytesMut) -> Bytes {
//         data.freeze()
//     }
// }

// /// 为WebSocketStream实现解码器
// impl<S> StreamCodec for WebSocketStream<S> {
//     type Req = Result<Message, tokio_tungstenite::tungstenite::Error>;
//     type Resp = Message;

//     fn decode(&self, data: Self::Req ) -> Option<BytesMut> {
//         match data {
//             Ok(Message::Binary(data)) => Some(BytesMut::from(&data[..])),
//             _ => None,
//         }
            
//     }
//     fn encode(&self, data: BytesMut) ->  Message{
//         Message::Binary(data.to_vec())
//     }
// }

/// 为TcpStream实现SplitStream<S>解码器
impl<T, U> StreamCodec for SplitStream<Framed<T, U> >
{

    type Req = Result<BytesMut, std::io::Error>;
    type Resp = Bytes;

    fn decode(&self, data: Self::Req) -> Option<BytesMut> {
        match data {
            Ok(data) => Some(data),
            _ => None,
        }
    }

    fn encode(&self, data: BytesMut) -> Bytes {
        data.freeze()
    }

}

/// 为WebSocketStream实现SplitStream<WebSocketStream<S>> 解码器
impl<S> StreamCodec for SplitStream<WebSocketStream<S>>
{

    type Req = Result<Message, tokio_tungstenite::tungstenite::Error>;
    type Resp = Message;

    fn decode(&self, data: Self::Req ) -> Option<BytesMut> {
        match data {
            Ok(Message::Binary(data)) => Some(BytesMut::from(&data[..])),
            _ => None,
        }
            
    }
    fn encode(&self, data: BytesMut) ->  Message{
        Message::Binary(data.to_vec())
    }

}