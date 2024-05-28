/*
 * @Author: plucky
 * @Date: 2023-02-18 09:24:47
 * @LastEditTime: 2023-03-06 08:56:34
 * @Description: 
 */


use hello_log::config::{setup_logging, LogConfig};

fn main() {
    let mut config = LogConfig::default();
    config.tofile = true;
    setup_logging(&config);
    tracing::info!(target:"my_crate","Hello, tracing!");
    log::info!(target:"my_crate","Hello, world!");
    // std::thread::sleep(std::time::Duration::from_secs(1));
}