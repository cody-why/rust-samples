/*
 * @Author: plucky
 * @Date: 2023-06-24 21:30:49
 * @LastEditTime: 2023-07-20 12:40:33
 */

mod service_fn;
use bytes::Bytes;

pub use service_fn::*;

use tokio::sync::broadcast;



type Receiver = broadcast::Receiver<(String, Bytes)>;

pub trait PublishService {
    // 订阅消息
    fn subscribe(&self) -> Receiver;
    // 广播消息
    fn broadcast(&self, topic: String, msg: Bytes)-> Result<usize, String>;
}


