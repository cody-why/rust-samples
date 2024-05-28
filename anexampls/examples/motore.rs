/*
 * @Author: plucky
 * @Date: 2023-07-29 23:12:10
 * @LastEditTime: 2023-08-07 22:49:41
 */

#![feature(impl_trait_in_assoc_type)]
use std::{sync::Arc, fmt::Debug};

 use futures::Future;
use motore::Service;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let s = motore::builder::ServiceBuilder::new()
        // .layer(motore::timeout::TimeoutLayer::new(Some(Duration::from_millis(100))))
        .service_fn(|_cx: &mut u32, req: String| {
            *_cx += 1;
            async move {
                // println!("req: {:?}", req);
                Ok::<_,String>(req.len())
            }
           
        });

    // let r =s.call(&mut 1_usize, "hello".to_string()).await;
    

    // let s =MyService;
    // let r =call_service(&s,&mut 1_usize,"hello2".to_string()).await;
    // println!("result: {:?}", r);

    // let server = MyServer::new(s);
    // server.run().await;

    // 测试一下时间
    let t = std::time::Instant::now();
    let mut sum = 0;
    for _ in 0..10000000{
        let _r = s.call(&mut sum, "MyServer run".to_string()).await;
    }

    println!("sum: {:?}, motore time: {:?}", sum, t.elapsed());

    test_tower().await;

    let t = std::time::Instant::now();
    let mut sum = 0;
    // 直接call异步函数为啥这么慢:1.341718273s,和tower一样
    for _ in 0..10000000{
        let _r = test_call(&mut sum, "MyServer run".to_string()).await;
    }

    println!("sum: {:?}, call time: {:?}", sum, t.elapsed());

    let t = std::time::Instant::now();
    let mut sum = 0;
    // 直接call同步函数为啥这么慢:1.341718273s,和tower一样
    for _ in 0..10000000{
        let _r = test_call_no(&mut sum, "MyServer run".to_string());
    }

    println!("sum: {:?}, no time: {:?}", sum, t.elapsed());

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}



struct MyService;

impl<Cx> Service<Cx, String> for MyService 
where
   Cx: 'static + Send,
{
    type Response = usize;
    type Error = String;
    type Future<'cx> = impl Future<Output = Result<Self::Response, Self::Error>> + 'cx;

    fn call<'cx, 's>(&'s self, _cx: &'cx mut Cx, _req: String) -> Self::Future<'cx>
    where
         's: 'cx,
    {
        async move {
            println!("req: {:?}", _req);
            // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            Ok::<_,String>(_req.len())
        }
    }
}

struct MyServer<S>{
    s: Arc<S>,
}

impl<S> MyServer<S> 
where 
    // Self: 'static + Send,
    S: Send + Service<usize, String> +Send+ Sync + 'static,
    S::Response: Debug,
    S::Error: Debug,
{
    pub fn new(s: S) -> Self {
        Self {
            s: Arc::new(s),
        }
    }

    async fn run(& self) {
        let s =self.s.clone();
        let mut sum = 0;
        tokio::spawn(async move {
            let r = s.call(&mut sum, "MyServer run".to_string()).await;
            println!("result: {:?}", r);
        });
        
    }
}

async fn test_tower(){
    let mut s = tower::ServiceBuilder::new()
        .service_fn( |e: (&mut u32, String)| {
            *e.0 += 1;
            async move {
                // println!("e: {:?}", e);
                Ok::<_,String>(e.1.len())
            }
        });

     // 测试一下时间
    let t = std::time::Instant::now();
    let mut sum = 0;
    for _ in 0..10000000{
        let _r = tower::Service::call(&mut s, (&mut sum, "MyServer run".to_string())).await;
    }

    tokio::spawn(async move {
        let _r = tower::Service::call(&mut s, (&mut sum, "MyServer run".to_string())).await;
    });
 
    println!("sum: {:?}, tower time: {:?}", sum, t.elapsed());
}

async fn test_call(n:&mut u32, e:String)->Result<usize,String>{
    *n += 1;
    Ok::<_,String>(e.len())
}

fn test_call_no(n:&mut u32, e:String)->Result<usize,String>{
    *n += 1;
    Ok::<_,String>(e.len())
}