/*
 * @Author: plucky
 * @Date: 2023-07-11 20:44:32
 * @LastEditTime: 2023-08-19 12:23:50
 */

//  #![allow(clippy::module_inception)]

mod tcp_server;
mod connection;
pub mod framer;


use std::{sync::Arc, fmt::Debug};

use bytes::{BytesMut, Bytes};
pub use tcp_server::*;
pub use connection::*;


// pub type Reqest<'a> = (&'a Arc<Connection>, NetEvent);
pub type Reqest = NetEvent;
pub type Response = Result<Option<Bytes>, AnError>;

// #[derive(Debug)]
pub enum NetEvent {
    Connect(Arc<Connection>),
    Disconnect(Arc<Connection>),
    Message(Arc<Connection>,BytesMut),
}

impl Debug for NetEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connect(arg0) => f.write_fmt(format_args!("Connect({:?})", arg0.addr)),
            Self::Disconnect(arg0) => f.write_fmt(format_args!("Disconnect({:?})", arg0.addr)),
            Self::Message(arg0, arg1) => f.debug_tuple("Message").field(&arg0.addr).field(&arg1).finish(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AnError {
    #[error("Generic: {0}")]
    Generic(String),
  
    #[error("Data base: {0}")]
    DataBase(String),
    #[error("Service: {0}")]
    Service(String),
    #[error("Codec: {0}")]
    Codec(String),
    
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("unknown data error")]
    Unknown,
    
}

impl From<&str> for AnError {
    fn from(s: &str) -> Self {
        AnError::Generic(s.to_string())
    }
}

impl From<String> for AnError {
    fn from(s: String) -> Self {
        AnError::Generic(s)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for AnError {
    fn from(error: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        AnError::Generic(error.to_string())
    }
}



use tokio::sync::broadcast;


pub type BroadcastSender = broadcast::Sender<(String, Bytes)>;
pub type BroadcastReceiver = broadcast::Receiver<(String, Bytes)>;

pub trait PublishService {
    // 订阅消息
    fn subscribe(&self) -> BroadcastReceiver;
    // 广播消息
    fn broadcast(&self, topic: String, msg: Bytes)-> Result<usize, String>;
}