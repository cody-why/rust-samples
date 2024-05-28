/*
 * @Author: plucky
 * @Date: 2023-07-21 12:15:00
 * @LastEditTime: 2023-08-19 12:02:56
 */

#![allow(unused)]

use std::{rc::Rc, cell::{RefCell, Cell}, sync::Arc};

use anserver::server::Connection;
use futures::Future;
use tower::{Service, util::ServiceFn};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    test_tower().await;

}

#[derive(Debug)]
enum Message {
    Message(String),
    
}

async fn test_tower(){

    let mut s = tower::ServiceBuilder::new()
        .service_fn( |e: (&Arc<Connection>, Message)| {
            // *e.0 += 1;
            *e.0.get_state_mut::<i32>().unwrap_or_else(||{
                e.0.set_state_mut(0);
                e.0.get_state_mut::<i32>().unwrap()
            }).write()+=1;
            let n = e.0.get_state_mut::<i32>().unwrap().read();
            println!("e: {:?}", n);
            
            async move {
                // println!("e: {:?}", e.1);
                Ok::<_,String>(e.1)
            }
        });
        let mut sum = 0;
        let c = Connection::default();
        let c = Arc::new(c);
        tokio::spawn(async move {
            let _r = tower::Service::call(&mut s, (&c, Message::Message("run".to_string()))).await;
            
            println!("sum: {:?}", c.get_state_mut::<i32>());
        });

     // 测试一下时间
    let t = std::time::Instant::now();
    
    // let mut sum = fun_name(&mut s).await;
 
    let server = MyServer{};
    // server.run2(s).await;
    
    println!("sum: {:?}, time: {:?}", sum, t.elapsed());

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

}

async fn fun_name<F, S>(s: &mut ServiceFn<S>) -> u32 
    where S: FnMut((&mut u32, String)) -> F + Send + 'static,
    F: Future<Output = Result<usize, String>>
{
    let mut sum = 0;
    for _ in 0..1000000{
        let _r = Service::call(s, (&mut sum, "run".to_string())).await;
    }
    sum
}

struct MyServer{
    
}

impl MyServer{
    pub async fn run<S>(&self, s1: S)
        where S: for<'a> Service<(&'a mut u32, String), Response = usize, Error = String> + Send +'static+Clone,
    // for<'a> <S as Service<(&'a mut u32, String)>>::Future: Send,
    {
    
        let local = tokio::task::LocalSet::new();
        
        for i in 0..10{
            let mut s = s1.clone();
            let mut sum = i;
            
            local.spawn_local(async move {
                // tokio::task::spawn_local(async move {
                    let rand= rand::random::<u8>() as u64;
                    tokio::time::sleep(std::time::Duration::from_millis(rand)).await;
                    let _r = Service::call(&mut s, (&mut sum, "MyServer run".to_string())).await;
                    println!("sum: {:?}", sum);
                // }).await;
            });
           
        }
        local.await;
        
    }

    pub async fn run2<S, F>(&self, s1: S)
        where S: for<'a> Service<(&'a mut u32, String),  Future = F> + Send +'static+Clone,
        F: Future<Output = Result<usize, String>>+Send,
        
    {
        for i in 0..10{
            let mut s = s1.clone();
            let mut sum = i;
            
            tokio::spawn(async move {
                    let rand= rand::random::<u8>() as u64;
                    tokio::time::sleep(std::time::Duration::from_millis(rand)).await;
                    let _r = Service::call(&mut s, (&mut sum, "Server run".to_string())).await;
                    println!("sum: {:?}", sum);
            });
           
        }
        
    }

    
    
}