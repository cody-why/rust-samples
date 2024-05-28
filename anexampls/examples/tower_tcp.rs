/*
 * @Author: plucky
 * @Date: 2023-07-09 01:30:36
 * @LastEditTime: 2023-07-19 00:27:55
 */

use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};


use bytes::{Bytes, BytesMut};
use futures::Future;
use ::time::UtcOffset;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tower::buffer::Buffer;
use tower::util::ServiceFn;
use tower::{ServiceBuilder, ServiceExt, Service};
use tracing::info;
use tracing_subscriber::fmt::time::OffsetTime;



#[tokio::main]
async fn main(){
    // 设置时区
    let offset = UtcOffset::from_hms(8, 0, 0).unwrap();
    // let timer = OffsetTime::new(offset, time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]").unwrap());
    let timer = OffsetTime::new(offset, time::format_description::well_known::Rfc3339);
    tracing_subscriber::fmt().with_timer(timer).init();

    let addr ="127.0.0.1:3000";

    let sev=  MyService::new();
    // 创建一个处理事件的Tower服务
    let event_service = ServiceBuilder::new().buffer(10)
        .service_fn(move |e|process_event(e, sev.clone()));
    
    // let process_event_service=service_fn2(move |e|process_event(e, sev.clone()));
    let server = MyServer::new(event_service);

    server.start(addr).await.unwrap();


}

pub fn service_fn2<F, R>(process_event: F) -> Buffer<ServiceFn<F>, NetEvent>
where
    F: FnMut(NetEvent) -> R + Clone + Send + Sync + 'static,
    R: Future<Output = Result<Option<Bytes>, AnError>> + Send + Sync + 'static,
{
    ServiceBuilder::new().buffer(10)
        .service_fn(process_event)
}

pub struct MyServer<E>
{
    process_event_service: E,
}

impl<E> MyServer<E>
where
    E: Service<NetEvent> + Clone+ Send+ Sync+ 'static,
    E::Error: Debug+Send,
    E::Future: futures::Future<Output = Result<Option<Bytes>,E::Error>>+Send,
    AnError: From<E::Error>,
{
    pub fn new(process_event_service: E) -> Self {
        MyServer {
            process_event_service,
        }
    }

    pub async fn start<A>(&self, addr: A) -> Result<(), AnError>
    where A: Into<String>,
    {
        let addr = addr.into();
        let listener = TcpListener::bind(addr.clone()).await?;
        info!("Server listening on {}", addr);
        loop {
            let (stream, _) = listener.accept().await?;
            let process_event_service = self.process_event_service.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, process_event_service).await {
                    tracing::error!("Error: {}", e);
                }
            });
        }
    }
}

async fn handle_connection<E>(mut stream: TcpStream, mut process_event_service: E) -> Result<(), AnError>
where
    E: Service<NetEvent> + Clone,
    E::Error: Debug,
    E::Future: futures::Future<Output = Result<Option<Bytes>,E::Error>>+Send,
    AnError: From<E::Error>,
{
    let mut buffer = [0; 1024];
    let addr = stream.peer_addr().unwrap().to_string();
    let _ = process_event_service.ready().await?.call(NetEvent::Connect(addr.clone())).await;

    loop {
        let bytes_read = stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        // let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let request = BytesMut::from(&buffer[..bytes_read]);
        let response = process_event_service.ready().await?.call(NetEvent::Message(request)).await?;
        if let Some(response) = response {
            stream.write_all(response.as_ref()).await?;
        }

    }

    let _ = process_event_service.ready().await?.call(NetEvent::Disconnect(addr)).await;
    Ok(())
}


#[derive(Debug)]
pub enum NetEvent {
    Connect(String),
    Disconnect(String),
    Message(BytesMut),
}

pub trait GetU8 {
    fn as_ref(&self) -> Option<&[u8]>;
}
impl GetU8 for Option<Bytes> {
    fn as_ref(&self) -> Option<&[u8]> {
        match self {
            Some(bytes) => Some(bytes.as_ref()),
            None => None,
        }
    }
}

async fn process_event(event: NetEvent, mut sev: MyService) -> Result<Option<Bytes>, AnError>{
    info!("process_event: {:?} , {}", event, sev.add());
    
    match event {
        NetEvent::Message(_bytes) => {
            // let mut bytes = BytesMut::new();
            // bytes.extend_from_slice(&_bytes);
            Ok(Some(_bytes.freeze()))
        },
        _ => {
            Ok(None)
        }
    }
        
}

#[derive(Debug, thiserror::Error)]
pub enum AnError {
    #[error("Generic: {0}")]
    Generic(String),
    
    #[error("Other: {0}")]
    Other(String),

    #[error("Data base: {0}")]
    DataBase(String),
    
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

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for AnError {
    fn from(error: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        AnError::Generic(error.to_string())
    }
}
   
#[derive(Clone)]
pub struct MyService{
    pub id: Arc<AtomicU32>,
}

impl MyService {
    pub fn new() -> Self {
        Self{
            id: Arc::new(AtomicU32::new(0)),
        }
        
    }

    pub fn add(&mut self) -> u32 {
        self.id.fetch_add(1, Ordering::SeqCst);
        self.id.load(Ordering::SeqCst)
    }
    
}
