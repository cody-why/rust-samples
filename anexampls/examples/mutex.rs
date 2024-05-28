/*
 * @Author: plucky
 * @Date: 2023-07-21 11:17:38
 * @LastEditTime: 2023-07-21 11:17:42
 */



fn main() {
    println!("Hello, world!");
}

#[tokio::test(flavor = "multi_thread")]
async fn test(){
    // use tokio::sync::Mutex;
    use parking_lot::Mutex;
    use std::sync::Arc;
    use std::time::Duration;
    let lock = Arc::new( Mutex::new(0));
    for _ in 0..100 {
        let lock = lock.clone();
        tokio::spawn(async move {
            // 同步锁不能等待异步释放,所以要在await的作用域前面释放锁
            {
                let mut guard = lock.lock();
                *guard += 1;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        });
    }
    
    tokio::time::sleep(Duration::from_secs(1)).await;
    let guard = lock.lock();
    assert_eq!(*guard, 100);
}