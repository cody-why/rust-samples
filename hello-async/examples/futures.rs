/*
 * @Author: plucky
 * @Date: 2023-02-26 20:30:38
 * @LastEditTime: 2023-07-07 09:25:28
 * @Description: 
 */

//1顺序执行,2同时执行,3选择执行
 use std::{time::Duration, sync::{Arc, Mutex}};

 use futures::{executor::block_on, join, select, FutureExt};
 use futures_timer::Delay;
 
 fn main() {
     block_on(async_main())
 }
 
 /// 共享的状态
 struct AppState {
     counter: u64,
 }
 
 type Data = Arc<Mutex<AppState>>;
 impl AppState {
     fn new() -> Data {
         let state: AppState = AppState { counter: 0 };
         Arc::new(Mutex::new(state))
     }
     
 }
     
 async fn async_main() {
     // 顺序执行两个异步函数
     // async_fn1().await;
     // async_fn2().await;
 
     let state = AppState::new();
 
     // 同时执行两个异步函数,2个结果都会返回
     let (r1, r2)= join!(async_fn1(&state), async_fn2(&state));
     println!("result #1={r1}, result #2={r2}");
 
     // 哪个先完成,只要有一个Future返回，就会返回
     let r = select!{
         r1 = async_fn1(&state).fuse() => r1,
         r2 = async_fn2(&state).fuse() => r2,
     };
 
     println!("result={r}");
     
 }
 
 async fn async_fn1(state: &Data) -> i32{
     {
         state.lock().unwrap().counter += 1;
     }
 
     // 随机延时
     let delay = rand::random::<u64>() % 3;
     Delay::new(Duration::from_secs(delay)).await;
    
     println!("Hello, world! delay={delay}s");
     
     1
 }
 
 async fn async_fn2(state: &Data) -> i32{
     {
         if let Ok(mut state) = state.lock() {
             state.counter += 1;
         } 
     }
 
 
     let delay = rand::random::<u64>() % 3;
     Delay::new(Duration::from_secs(delay)).await;
     println!("Hello again, world! delay={delay}s");
     2
 }