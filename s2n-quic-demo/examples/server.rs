/*
 * @Author: plucky
 * @Date: 2022-10-31 16:47:53
 * @LastEditTime: 2022-10-31 18:00:36
 * @Description: 
 */

use bytes::{BytesMut, BufMut};
use s2n_quic::Server;
use std::{error::Error};

pub static CERT_PEM: &str = include_str!(concat!(
    "../certs/cert.pem"
));
pub static KEY_PEM: &str = include_str!(concat!(
    "../certs/key.pem"
));


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut server = Server::builder()
        .with_tls((CERT_PEM, KEY_PEM))?
        .with_io("127.0.0.1:4433")?
        .start()?;
    println!("server started {:?}",server.local_addr());

    while let Some(mut connection) = server.accept().await {
        // spawn a new task for the connection
        tokio::spawn(async move {
            while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
                // spawn a new task for the stream
                tokio::spawn(async move {
                    // echo any data back to the stream
                    while let Ok(Some(data)) = stream.receive().await {
                        let mut buf = BytesMut::with_capacity(1024);
                        buf.put(&b"hello world "[..]);
                        buf.put(data);
                        let data = bytes::Bytes::from(buf);
                        stream.send(data).await.expect("stream should be open");
                    }
                });
            }
        });
    }

    Ok(())
}