/*
 * @Author: plucky
 * @Date: 2023-03-19 00:43:23
 * @LastEditTime: 2023-03-19 00:53:04
 * @Description: Semaphore的用途: 限制并发的任务数
 */

use std::time::Duration;
use log_error::*;
use tokio::{sync::Semaphore, time};

#[tokio::main]
async fn main() {
    let semaphore = Semaphore::new(2); // 创建一个初始值为 2 的信号量

    let permit1 = semaphore.acquire().await.unwrap(); // 获取一个许可证
    let _permit2 = semaphore.acquire().await.unwrap(); // 获取另一个许可证
    // 2 个许可证都被使用，因此信号量中没有剩余的许可证
    println!("acquired two permits");
    let _= time::timeout(Duration::from_secs(2), async {
        // 此时，信号量中没有剩余的许可证，因此下面的调用将被阻塞
        let _permit3 = semaphore.acquire().await.unwrap();
    }).await.log_warn("timeout");
    println!("timeout");

    // 释放一个许可证
    drop(permit1);

    // 现在，信号量中有一个剩余的许可证，因此下面的调用将立即返回
    let _permit4 = semaphore.acquire().await.unwrap();
    println!("acquired another permit");
}