/*
 * @Author: plucky
 * @Date: 2023-05-11 23:09:32
 * @LastEditTime: 2023-05-11 23:11:04
 * @Description: 
 */

use std::time::Duration;
use tokio::time;

struct GameTimer {
    interval: Duration,
    f: Box<dyn Fn() -> Box<dyn std::future::Future<Output = ()> + Send + 'static> + Send + Sync>,
}

impl GameTimer {
    fn new(
        interval: u64,
        f: impl Fn() -> Box<dyn std::future::Future<Output = ()> + Send + 'static> + 'static + Send + Sync,
    ) -> Self {
        Self {
            interval: Duration::from_millis(interval),
            f: Box::new(f),
        }
    }

    async fn run(&self) {
        let mut interval = time::interval(self.interval);
        loop {
            interval.tick().await;
            let f = (self.f)();
            f.await;
        }
    }
}


use std::net::SocketAddr;
use tokio::sync::mpsc;

struct World {
    // other fields...
}

impl World {
    fn new() -> Self {
        Self {
            // ...
        }
    }

    async fn push_message(&self, conn: SocketAddr, msg_id: u16, data: &str) {
        // push message to connection here
        // ...
    }

    async fn broadcast(&self, msg_id: u16, data: &str) {
        // broadcast message here
        // ...
    }

    fn add_timer(&self, timer: GameTimer) {
        // add timer here
        // ...
    }
}