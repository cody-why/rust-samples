/*
 * @Author: anger
 * @Date: 2023-11-25 22:01:08
 * @LastEditTime: 2023-11-25 23:14:24
 */

use std::{thread, time::Duration};

use fast_log::{Config, consts::LogSize, plugin::{file_split::RollingType, packer::LogPacker}};

fn main() {
    // fast_log::init(Config::new().file("log/test.log").chan_len(Some(100000))).unwrap();
    fast_log::init(Config::new().console().chan_len(Some(100000))
    .file_split("logs/",
                    LogSize::MB(1),
                    RollingType::All,
                    LogPacker{}))
    .unwrap();
 
    // work add log feature
    tracing::info!("hello world tracing");
  
    log::info!("hello world log");
    // fast_log::flush();
    thread::sleep(Duration::from_secs(1));
}