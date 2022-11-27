/*
 * @Author: plucky
 * @Date: 2022-11-25 21:32:41
 * @LastEditTime: 2022-11-25 22:16:42
 * @Description: 
 */

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use std::time::{Duration, Instant};

    use tokio::{join, time::sleep };


    #[tokio::test]
    async fn test_async() {
        dance_and_sing().await;
        println!("===================");
        dance_and_sing2().await;
    }

    // 跳舞需要2秒，唱歌需要1秒
    async fn dance(){
        sleep(Duration::from_secs(2)).await;
        println!("dance");
    }

    async fn sing() {
        println!("学习 sing ...");
        sleep(Duration::from_secs(1)).await;
        println!("sing");
        
    }

    // 串行执行, 3秒, await等待一个future完成,在等待过程中会调用其他任务
    // 结果是顺序的
    // dance
    // 学习 sing ...
    // sing
    async fn dance_and_sing() {
        let t = Instant::now();
        dance().await;
        sing().await;
        println!("dance_and_sing cost: {:?}", t.elapsed());
    }

    // 并行执行，2秒, 两个任务同时执行，等待两个任务都完成
    // 学习 sing ...
    // sing
    // dance
    async fn dance_and_sing2() {
        let t = Instant::now();
        // join! 在同一个线程中执行两个任务
        join!(dance(), sing());
        println!("dance_and_sing2 cost: {:?}", t.elapsed());
    }

}