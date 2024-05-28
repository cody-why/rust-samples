/*
 * @Author: plucky
 * @Date: 2023-07-11 22:45:35
 * @LastEditTime: 2023-08-19 12:20:03
 */

 use std::fmt::Debug;

 use bytes::Bytes;
 use dashmap::DashMap;
 use parking_lot::RwLock;
 use state::TypeMap;
 use tokio::sync::mpsc;
 
 
 #[derive(Debug)]
 /// Represents a connection information.
 pub struct Connection {
     /// The address of the connection.
     pub addr: String,
     /// 订阅主题
     pub topics: DashMap<String,()>,
     /// The sender for sending messages.
     pub sender: mpsc::Sender<Bytes>,
     /// The associated states, such as player information.
     pub state: TypeMap![Send+Sync],
 
 }
 
 
 impl Connection {
     /// Creates a new connection.
     pub fn new(addr: String, sender: mpsc::Sender<Bytes>) -> Self {
         Self {
             topics: DashMap::new(),
             state: <TypeMap![Send+Sync]>::new(),
             addr,
             sender,
         }
     }
 
     /// Sends a message asynchronously.
     #[inline]
     pub async fn send(&self, msg: Bytes)-> Result<(), String>{
         self.sender.send(msg).await.map_err(|e| format!("Connection send erro: {}",e))
     }
     /// Tries to send a message. If the connection is full, the message is dropped.
     #[inline]
     pub fn try_send(&self, msg: Bytes)-> Result<(), String>{
         self.sender.try_send(msg).map_err(|e| format!("Connection send erro: {}",e))
     }
     /// Subscribes to a topic.
     pub fn subscribe(&self, topic: impl Into<String>) {
         self.topics.insert(topic.into(), ());
     }
     /// Unsubscribes from a topic.
     pub fn unsubscribe(&self, topic: &str) {
         self.topics.remove(topic);
     }
     /// Checks if the connection is subscribed to a topic.
     pub fn is_subscribed(&self, topic: &str) -> bool {
         self.topics.contains_key(topic)
     }
     
     /// Stores a state of a specific type. The state is read-only.
     /// 和get_state搭配使用
     #[inline]
     pub fn set_state<T: Send + Sync + 'static>(&self, state: T) {
         self.state.set(state);
     }
     /// Retrieves a state of a specific type. The state is read-only.
     /// 和set_state搭配使用
     #[inline]
     pub fn get_state<T: Send + Sync + 'static>(&self) -> Option<&T> {
         self.state.try_get::<T>()
     }
     /// 储存一个类型, 通过get_state_mut获取,可以获取读写状态
     /// 和get_state_mut搭配使用
     /// Stores a state of a specific type. The state can be accessed for both reading and writing.
     #[inline]
     pub fn set_state_mut<T: Send + Sync + 'static>(&self, state: T) {
         let state = RwLock::new(state);
         self.state.set(state);
     }
 
     /// 和set_state_mut搭配使用,才能获取到读写状态
     /// It must be used with with_state_mut storage to obtain the read and write state
     #[inline]
     pub fn get_state_mut<T: Send + Sync + 'static>(&self) -> Option<&RwLock<T>>{
         self.state.try_get::<RwLock<T>>()
         
     }
 
 }
     
 
 impl Drop for Connection {
     fn drop(&mut self) { 
         // self.conns.fetch_sub(1, Ordering::SeqCst);
         // info!("Client: {} disconnected",self.addr);
      }
 }
 
 
 impl Default for Connection {
     fn default() -> Self {
         let (s, _) = mpsc::channel(1);
         Self {
             addr: "127.0.0.1:3000".parse().unwrap(),
             topics: Default::default(),
             sender: s,
             state: Default::default(),
         }
     }
     
 }