/*
 * @Author: plucky
 * @Date: 2023-07-12 21:25:00
 * @LastEditTime: 2023-07-17 18:34:26
 */

use std::{ io, pin::Pin, task::{ Context, Poll }};
use bytes::{ BytesMut, Bytes };
use futures::{ Stream, Sink, StreamExt, SinkExt};
use tokio_util::codec::{ Framed, LengthDelimitedCodec, length_delimited::Builder };
use tokio::net::TcpStream;
use super::{ Communication, Framer, FramerBuilder };

pub struct TcpFramer {
    inner: Communication<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
}

impl TcpFramer {
    pub fn new(stream: TcpStream) -> Self {
        let framed = Framed::new(stream, LengthDelimitedCodec::new());
        let inner = Communication::new(framed);
        TcpFramer { inner }
    }

    pub fn builder() -> Builder {
        Builder::new()
    }
    
}

impl Framer for TcpFramer
{}

impl FramerBuilder for Builder{
    type Stream = TcpStream;
    type Framed = TcpFramer;

    fn build(&self, stream: Self::Stream)-> Result<Self::Framed, io::Error>{
        let codec = self.new_codec();
        let framed= Framed::new(stream, codec);
        let inner = Communication::new(framed);
        Ok(TcpFramer {inner})
    }
}



// impl LengthBuilder {
//     pub fn new() -> Self {
//         LengthBuilder(Builder::new())
//     }

//     /// 构建 TcpCommunication
//     pub fn build(&self) -> TcpCommunication {
//         let codec = self.0.new_codec();
//         TcpCommunication { inner: None, codec: Some(codec) }
//     }

    // /// 设置最大帧长度
    // /// 默认值为 8MB
    // pub fn max_frame_length(&mut self, val: usize) -> &mut Self {
    //     self.0.max_frame_length(val);
    //     self
    // }

    // // /// 设置标头用于表示长度的字节数
    // // /// 默认值为 4。最大值为 8。
    // // pub fn length_field_length(&mut self, val: usize) -> &mut Self {
    // //     self.0.length_field_length(val);
    // //     self
    // // }

    // /// 将标头长度读取为大端整数
    // /// 默认设置 big_endian
    // pub fn big_endian(&mut self) -> &mut Self {
    //     self.0.big_endian();
    //     self
    // }

    // /// 将标头长度读取为小端整数
    // /// 默认 big_endian
    // pub fn little_endian(&mut self) -> &mut Self {
    //     self.0.little_endian();
    //     self
    // }

    // /// 标头中指定的数据长度与实际数据长度之间的增量
    // pub fn length_adjustment(&mut self, val: isize) -> &mut Self {
    //     self.0.length_adjustment(val);
    //     self
    // }

    // /// 设置标头中长度字段之前的字节数
    // /// 此配置选项仅适用于解码。
    // pub fn length_field_offset(&mut self, val: usize) -> &mut Self {
    //     self.0.length_field_offset(val);
    //     self
    // }

    // /// 设置读取数据之前要跳过的字节数  默认值为 length_field_len + length_field_offset
    // /// 此配置选项仅适用于解码
    // pub fn num_skip(&mut self, val: usize) -> &mut Self {
    //     self.0.num_skip(val);
    //     self
    // }
// }


impl Stream for TcpFramer
{
    type Item = Result<BytesMut, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.reader.poll_next_unpin(cx)
    }
}

impl Sink<Bytes> for TcpFramer
{
    type Error = io::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.writer.poll_ready_unpin(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: Bytes) -> Result<(), Self::Error> {
        self.inner.writer.start_send_unpin(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.writer.poll_flush_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.writer.poll_close_unpin(cx)
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use tokio::net::TcpStream;

    use super::*;

    #[tokio::test]
    async fn test_tcp() {
        let builder = TcpFramer::builder()
            .length_field_length(4).to_owned();

        let service = MyService1::new(builder);
        service.call_communication().await;
       
        
    }
    
    struct MyService1<CB>{
        builder: CB,
        _item: PhantomData<()>
    }

    impl <CB> MyService1<CB> 
    where CB: FramerBuilder<Stream = TcpStream>,
    {
        pub fn new(builder: CB) -> Self {
            Self { builder, _item: PhantomData }
        }

        pub async fn call_communication(&self) {
            let addr = "127.0.0.1:3000";
            let stream = TcpStream::connect(addr).await.unwrap();

            let mut framer = self.builder.build(stream).unwrap();
            framer.send(Bytes::from("hello")).await.unwrap();
            while let Some(data) = framer.next().await {
                println!("data: {:?}", data);
            }
        }
    }

    #[allow(dead_code)]
    async fn call_communication<C: FramerBuilder<Stream = S>, S>(builder: &C, stream: S) {
        let mut framer = builder.build(stream).unwrap();
        framer.send(Bytes::from("hello")).await.unwrap();
        while let Some(data) = framer.next().await {
            println!("data: {:?}", data);
        }
        
    }
    
}
