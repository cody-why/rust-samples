/*
 * @Author: plucky
 * @Date: 2022-12-14 22:28:13
 * @LastEditTime: 2022-12-19 18:19:22
 * @Description:
 */

use state::Container;

/// 全局上下文
pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

// #[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new(port: u16) -> Self {
        Self {
            port,
        }
    }
}

fn main() {
    APPLICATION_CONTEXT.set::<Config>(Config::new(8080));
    let config = APPLICATION_CONTEXT.get::<Config>();
    println!("{:?}", config);
    
}