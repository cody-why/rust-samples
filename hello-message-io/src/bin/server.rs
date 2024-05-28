/*
 * @Author: plucky
 * @Date: 2023-02-04 18:28:57
 * @LastEditTime: 2023-02-05 18:11:53
 * @Description: 
 */

use std::collections::HashMap;
use std::time::Duration;

use message_io::node::{self, NodeEvent};
use message_io::network::{NetEvent, Transport, Endpoint};

enum Signal {
    Heartbeat,
    // Any other app event here.
}
struct ClientInfo {
    count: usize,
}

//cargo run --bin server
// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
fn main() {
    // Create a node, the main message-io entity. It is divided in 2 parts:
    // The 'handler', used to make actions (connect, send messages, signals, stop the node...)
    // The 'listener', used to read events from the network or signals.
    let (handler, listener) = node::split();

    // Listen for TCP, UDP and WebSocket messages at the same time.
    handler.network().listen(Transport::FramedTcp, "0.0.0.0:3042").unwrap();
    handler.network().listen(Transport::Ws, "0.0.0.0:3043").unwrap();
    // handler.network().listen(Transport::Udp, "0.0.0.0:3044").unwrap();
    println!("Listening at tcp 3042, ws 3043, udp 3044");
    
    handler.signals().send_with_timer(Signal::Heartbeat, Duration::from_secs(10));
    let mut clients: HashMap<Endpoint, ClientInfo> = HashMap::new();
    
    // Read incoming network events.
    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_, _) => unreachable!(), // client
            NetEvent::Accepted(endpoint, _listener) => {
                clients.insert(endpoint, ClientInfo { count: 0 });
                println!("Client ({}) connected (total clients: {})", endpoint.addr(), clients.len());
            }, // Tcp or Ws
            NetEvent::Message(endpoint, data) => {
                println!("Received: {}", String::from_utf8_lossy(data));
                handler.network().send(endpoint, data);
                match clients.get_mut(&endpoint) {
                    Some(client) => {
                        // For connection oriented protocols
                        client.count += 1;
                        println!("Received from {}, {} times", endpoint.addr(), client.count);
                        
                    }
                    None => {
                        // For non-connection oriented protocols
                        println!("Received from {} non-connection", endpoint.addr());
                        
                    }
                };
                
            },
            NetEvent::Disconnected(endpoint) => {
                clients.remove(&endpoint);
                println!(
                    "Client ({}) disconnected (total clients: {})",
                    endpoint.addr(),
                    clients.len()
                );
            }, //Tcp or Ws
        }
        NodeEvent::Signal(signal) => match signal {
            Signal::Heartbeat => {
                println!("Heartbeat");
                
                handler.signals().send_with_timer(Signal::Heartbeat, Duration::from_secs(10));
            }
        }
    });
}