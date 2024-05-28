/*
 * @Author: plucky
 * @Date: 2022-08-30 10:20:56
 * @LastEditTime: 2023-02-16 20:26:57
 * @Description: 
 */

use futures_util::{SinkExt, StreamExt};
use log::*;
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Error};
use tokio_tungstenite::tungstenite::{Result, Message};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    let addr = "0.0.0.0:9001";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    // 用broadcast channel来广播消息
    let (tx, _rx) = tokio::sync::broadcast::channel::<(String,SocketAddr)>(1024);

    while let Ok((stream, addr)) = listener.accept().await {
        let tx = tx.clone();
        tokio::spawn(accept_connection(stream, addr, tx));
    }
}

async fn accept_connection(stream: TcpStream, addr: SocketAddr, tx: Sender<(String,SocketAddr)>){
    // let peer = stream.peer_addr().expect("connected streams should have a peer address");
        // info!("Peer address: {}", peer);
    if let Err(e) = handle_connection(stream, addr, &tx).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr, tx: &Sender<(String,SocketAddr)>) -> Result<()> {
    // let ws_stream = accept_async(stream).await.expect("Failed to accept");
    let copy_headers_callback = |request: &Request,  response: Response| -> Result<Response, ErrorResponse> {
        for (name, value) in request.headers().iter() {
            println!("Name: {}, value: {}", name.to_string(), value.to_str().expect("expected a value"));
        }

        //access the protocol in the request, then set it in the response
        // protocol = request.headers().get(SEC_WEBSOCKET_PROTOCOL).expect("the client should specify a protocol").to_owned(); //save the protocol to use outside the closure
        // let response_protocol = request.headers().get(SEC_WEBSOCKET_PROTOCOL).expect("the client should specify a protocol").to_owned();
        // response.headers_mut().insert(SEC_WEBSOCKET_PROTOCOL, response_protocol);
        Ok(response)
    };
    let ws_stream = accept_hdr_async(stream, copy_headers_callback).await?;
    info!("New ws connection: {}", addr);

    // while let Some(msg) = ws_stream.next().await {
    //     let msg = msg?;
    //     if msg.is_text() || msg.is_binary() {
    //         ws_stream.send(msg).await?;
    //     }
    // }

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    // 10 seconds timeout disconnect
    let mut interval = tokio::time::interval(Duration::from_millis(10000));
    let mut is_active = true;
    
    let mut rx = tx.subscribe();
    
    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() ||msg.is_binary() {
                            is_active = true;
                            // 广播
                            tx.send((msg.clone().to_string(), addr)).unwrap();
                            // 返回
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                println!("tick");
                if !is_active {
                    // ws_sender.send(Message::Close(None)).await?;
                    break;
                }
                is_active = false;
                ws_sender.send(Message::Binary(Vec::from("tick"))).await?;
            }

            msg = rx.recv() => {
                match msg {
                    Ok(msg) => {
                        let (msg, add) = msg;
                        if add != addr {
                            ws_sender.send(Message::Text(msg)).await?;
                        }
                    }
                    Err(_) => break,
                }
               
            }
        }
    }
    
    info!("Disconnection: {}", addr);
    Ok(())
}

