/*
 * @Author: plucky
 * @Date: 2023-08-22 11:04:46
 * @LastEditTime: 2023-08-29 18:33:19
 */
#![allow(unused)]

use bytes::{Buf, BufMut};
use tokio::{net::{TcpListener, TcpStream}, io::AsyncRead};
use tokio_tungstenite::WebSocketStream;
use tokio_util::codec::{Decoder, Encoder};
use futures::{StreamExt, SinkExt, Future, TryStreamExt};

#[derive(Debug)]
// 自定义数据包类型
pub struct Packet {
    // 数据包字段
    pub some_field: String,
}

impl Drop for Packet {
    fn drop(&mut self) {
        println!("drop packet: {}", self.some_field);
    }
}
    

// 自定义解码器
#[derive(Clone)]
pub struct PacketCodec;

impl Decoder for PacketCodec {
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

impl Encoder<Packet> for PacketCodec {
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


// 传入异步函数回调
async fn start_server<Fut>(handle: impl Fn(&'static Packet)->Fut +Send+Sync+Copy+'static) 
    where Fut: Future<Output = ()> + Send
{
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    let codec = PacketCodec;
    
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let codec = codec.clone();
        
        println!("New connection");
        tokio::spawn(async move {
            // ws 是不可以的,因为ws解析framed有自己的数据长度协议
            // let mut socket = tokio_tungstenite::accept_async(socket).await.unwrap();
            // let socket = socket.get_mut();
            let (mut writer, mut reader) = codec.framed(socket).split();

            while let Some(packet) = reader.next().await {
                match packet {
                    Ok(packet) => {
                        // leak the packet to 'static
                        let packet =Box::leak(Box::new(packet));
                        // Drop the packet after {}
                        let _b =unsafe{Box::from_raw(packet)};
                        
                        handle(packet).await;
                        

                        let packet = Packet { some_field: "hello from server".to_string(), };
                        writer.send(packet).await.unwrap();
                    }
                    Err(err) => {
                        eprintln!("Error decoding packet: {:?}", err);
                        break;
                    }
                }
            }

            // Clean up resources
            println!("Connection closed");
        });
    }
}



#[tokio::main]
async fn main() {
    // start_server(|p|{
    //     async move {
    //         handle_packet(p).await;
            
    //     }
    // }).await;
    start_server(handle_packet).await;
}

async fn handle_packet(packet: &Packet){
    println!("Received packet: {:?}", packet);
}


#[tokio::test]
async fn feature() {
    // 测试客户端
    let codec = PacketCodec;
    let socket = tokio::net::TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let (mut writer, mut reader) = codec.framed(socket).split();

    tokio::spawn(async move {
        while let Some(packet) = reader.next().await {
            match packet {
                Ok(packet) => {
                    handle_packet(&packet).await;
                },
                Err(err) => {
                    eprintln!("Error decoding packet: {:?}", err);
                    break;
                }
            }
        }
    });

    let packet = Packet {
        some_field: "hello from client".to_string(),
    };
    writer.send(packet).await.unwrap();
    let packet = Packet {
        some_field: "hello from client2".to_string(),
    };
    writer.send(packet).await.unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}

