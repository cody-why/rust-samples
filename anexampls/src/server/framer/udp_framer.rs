#![allow(dead_code)]

use std::{pin::Pin, task::{Context, Poll}};

use bytes::Bytes;
use tokio::{net::UdpSocket, io::{AsyncRead, AsyncWrite}};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use super::Communication;

pub struct UdpFramer {
    inner: Communication<Framed<UdpStream, LengthDelimitedCodec>, Bytes>,
}

impl UdpFramer {
    pub fn new(stream: UdpStream) -> Self {
        let framed = Framed::new(stream, LengthDelimitedCodec::new());
        let inner = Communication::new(framed);
        UdpFramer { inner }
    }
}

pub struct UdpStream(UdpSocket);

impl AsyncRead for UdpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.get_mut().0.poll_recv(cx, buf)
    }
}

impl AsyncWrite for UdpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        self.get_mut().0.poll_send(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        // self.get_mut().0.poll_send_ready(cx)
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }
}