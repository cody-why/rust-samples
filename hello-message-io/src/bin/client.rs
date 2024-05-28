/*
 * @Author: plucky
 * @Date: 2023-02-04 18:29:47
 * @LastEditTime: 2023-02-05 17:15:24
 * @Description: 
 */

use message_io::node::{self, NodeEvent};
use message_io::network::{NetEvent, Transport};
use std::time::Duration;

enum Signal {
    Greet,
    // Any other app event here.
}

fn main() {
    let (handler, listener) = node::split();

    // You can change the transport to Udp or Ws (WebSocket).
    let (server, local_addr) = handler.network().connect(Transport::FramedTcp, "127.0.0.1:3042").unwrap();

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_endpoint, ok) => {
                if ok {
                    println!("Connected to server at {}, local port: {}", server.addr(),local_addr.port());
                    handler.signals().send(Signal::Greet);
                }
                else {
                    println!("Can not connect to server at {}", server.addr())
                }
            }
            NetEvent::Accepted(_, _) => unreachable!(), // Only generated by listening
            NetEvent::Message(_endpoint, data) => {
                println!("Received: {}", String::from_utf8_lossy(data));
            },
            NetEvent::Disconnected(_endpoint) => handler.stop(),
        }
        NodeEvent::Signal(signal) => match signal {
            Signal::Greet => { // computed every second
                handler.network().send(server, "Hello server!".as_bytes());
                handler.signals().send_with_timer(Signal::Greet, Duration::from_secs(1));
            }
        }
    });
}