/*
 * @Author: plucky
 * @Date: 2023-08-07 21:12:02
 * @LastEditTime: 2023-08-23 18:37:16
 */
#![allow(unused)]

use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use trc::SharedTrc;


fn main(){
    // debug 10000000, no_clone: 127.388773ms, arc: 344.332146ms, rc: 305.717137ms, map_time: 4.063436395s
    // release 10000000, no_clone: 51.693557ms, arc: 157.972679ms, rc: 62.875989ms
    // 10000000, no_clone: 144.311568ms, test_arc: 339.294294ms ,test_arc_trc: 385.468496ms
    // test_static: 143.204756ms, test_rc: 292.781045ms, test_arc_rclite: 377.686463ms
    let total = 10000000;
    // test_no(total);
    // test_rc(total);
    test_arc(total);
    // test_arc_trc(total);
    // test_arc_rclite(total);
    test_static(total);
    // test_map(total);

}

fn test_no(total: u32){
    let t = std::time::Instant::now();
    let  arc = AtomicUsize::new(0);
    for _ in 0..total{
        arc.fetch_add(1, Ordering::Relaxed);
    }
    println!("{}, no_clone: {:?}", arc.load(Ordering::SeqCst), t.elapsed());
}

fn test_rc(total: u32){
    let t = std::time::Instant::now();
    let  arc = Rc::new(AtomicUsize::new(0));
    for _ in 0..total{
        let arc2 = arc.clone();
        arc2.fetch_add(1, Ordering::Relaxed);
    }
    println!("{}, test_rc: {:?}", arc.load(Ordering::SeqCst), t.elapsed());
}

fn test_arc(total: u32){
    let t = std::time::Instant::now();
    let  arc = Arc::new(AtomicUsize::new(0));
    for _ in 0..total{
        let arc2 = arc.clone();
        arc2.fetch_add(1, Ordering::Relaxed);
    }
    println!("{}, test_arc: {:?}", arc.load(Ordering::SeqCst), t.elapsed());
}

fn test_arc_trc(total: u32){
    let t = std::time::Instant::now();
    let  arc = trc::Trc::new(AtomicUsize::new(0));
    let arc = SharedTrc::from_trc(&arc);
    for _ in 0..total{
        let arc2 = arc.clone();
        arc2.fetch_add(1, Ordering::Relaxed);
    }
 
    println!("{}, test_arc_trc: {:?}", arc.load(Ordering::SeqCst), t.elapsed());

    let arc2 =arc.clone();
    // let arc2 = SharedTrc::from_trc(&arc);
    
    std::thread::spawn( move || {
        arc2.load(Ordering::SeqCst);
    });
}





fn test_static(total: u32){
    let t = std::time::Instant::now();
    let  arc = AtomicUsize::new(0);
    let arc = Box::leak(Box::new(arc));
    let _b = unsafe{Box::from_raw(arc)};
    
    for _ in 0..total{
        arc.fetch_add(1, Ordering::Relaxed);
    }
 
    println!("{}, test_static: {:?}", arc.load(Ordering::SeqCst), t.elapsed());

    std::thread::spawn( move || {
        arc.load(Ordering::SeqCst);
    });

}


fn test_arc_rclite(total: u32){
    let t = std::time::Instant::now();
    let arc = rclite::Arc::new(AtomicUsize::new(0));
    
    for _ in 0..total{
        let arc2 = arc.clone();
        arc2.fetch_add(1, Ordering::Relaxed);
    }
   
    println!("{}, test_arc_rclite: {:?}", arc.load(Ordering::SeqCst), t.elapsed());

     // let arc2 =arc.clone();
    // std::thread::spawn( move || {
    //     arc2.load(Ordering::SeqCst);
    // });
}

#[allow(unused)]
// 如果是传递id,用map去取,比Arc慢好多,10000000, map time: 4.063436395s
fn test_map(total: u32){
    let mut map = std::collections::HashMap::new();
    for i in 0..total{
        map.insert(i, i);
    }

    let t = std::time::Instant::now();
    for i in 0..total{
        map.get(&i);
    }
    println!("map_time: {:?}", t.elapsed());
}

struct Connection{
    pub sum: u32
}

#[tokio::test]
async fn feature() {
    tokio::spawn(async move {
        let mut arg = Connection{sum: 0};
        feature_ref(&mut arg).await;
        println!("{}", arg.sum);
    });
}

async fn feature_ref(arg: &mut Connection) {
    arg.sum += 1;

}