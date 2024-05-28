/*
 * @Author: plucky
 * @Date: 2023-07-02 22:05:29
 * @LastEditTime: 2023-07-03 12:01:36
 */
#![allow(clippy::new_without_default)]

use tokio::sync::broadcast;
use std::collections::HashMap;

pub struct EventManager<T> {
    event_receivers: HashMap<String, broadcast::Sender<T>>,
}

impl<T> EventManager<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        EventManager {
            event_receivers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event_name: &str) -> broadcast::Receiver<T> {
        if let Some(event_sender) = self.event_receivers.get(event_name) {
            return event_sender.subscribe();
        }
        let (event_sender, event_receiver) = broadcast::channel(100);
        self.event_receivers.insert(event_name.to_string(), event_sender);
        event_receiver
    }

    pub fn unsubscribe(&mut self, event_name: &str) {
        self.event_receivers.remove(event_name);
    }

    pub async fn send_event(&self, event_name: &str, event: T) {
        if let Some(event_sender) = self.event_receivers.get(event_name){
            event_sender.send(event).ok();
        }
        
    }
}
#[tokio::main]
async fn main() {
    let mut event_manager = EventManager::new();

    let _receiver1 = event_manager.subscribe("event1");
    let mut receiver1 = event_manager.subscribe("event1");
    let mut receiver2 = event_manager.subscribe("event2");

    tokio::spawn(async move {
        loop {
            let event = receiver1.recv().await;
            match event {
                Ok(event) => println!("Received event1: {:?}", event),
                Err(_) => break,
            }
        }
    });

    tokio::spawn(async move {
        loop {
            let event = receiver2.recv().await;
            match event {
                Ok(event) => println!("Received event2: {:?}", event),
                Err(_) => break,
            }
        }
    });
    // 好啊
    event_manager.send_event("event1", "Hello from event1").await;
    event_manager.send_event("event2", "Hello from event2").await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    event_manager.unsubscribe("event1");
    event_manager.unsubscribe("event2");

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}
