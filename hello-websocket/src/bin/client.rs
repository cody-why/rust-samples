/*
 * @Author: plucky
 * @Date: 2022-10-31 21:55:45
 * @LastEditTime: 2022-11-03 17:53:04
 * @Description: 
 */



use std::process;

use futures_util::{future,  StreamExt, pin_mut};
use tokio::io::{AsyncReadExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    // let connect_addr = env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));
    let connect_addr = "ws://localhost:9001/getCaseCount";
    
    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));
    
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();
    
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|msg| async {
            match msg {
                Ok(msg) => {
                    println!("Received message: {}", msg);
                },
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(1);
                }
            }
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

#[allow(dead_code)]
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}