/*
 * @Author: plucky
 * @Date: 2023-08-23 23:15:18
 * @LastEditTime: 2023-08-23 23:39:19
 */


 use bytes::{Buf, BufMut};
 use tokio_util::codec::{Decoder, Encoder};

use super::Packet;
 
 // 自定义解码器
 #[derive(Clone)]
 pub struct WsCodec;
 
 impl Decoder for WsCodec {
     type Item = Packet;
     type Error = std::io::Error;
 
     fn decode(&mut self, buf: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
         // 解析数据包
         // 数据长度2个字节
         if buf.len() < 2 {
             return Ok(None);
         }
         let packet_len = buf.get_u16() as usize;
         if buf.len() < packet_len {
             return Ok(None);
         }
         let packet = Packet {
             some_field: String::from_utf8_lossy(&buf[..packet_len]).to_string(),
         };
         buf.advance(packet_len);
 
         Ok(Some(packet))
     }
 }
 
 impl Encoder<Packet> for WsCodec {
     type Error = std::io::Error;
 
     fn encode(&mut self, item: Packet, buf: &mut bytes::BytesMut) -> Result<(), Self::Error> {
         // 序列化数据包，并将结果写入 buf
         let len = item.some_field.len();
         buf.reserve(2 + len);
         buf.put_u16(len as u16);
         buf.put(item.some_field.as_bytes());
         
         Ok(())
     }
 }
 
 