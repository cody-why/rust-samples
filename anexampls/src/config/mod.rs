/*
 * @Author: plucky
 * @Date: 2023-06-25 18:31:45
 * @LastEditTime: 2023-07-20 12:32:26
 */

// 不检查与其同名的模块,父模块
#![allow(clippy::module_inception)]

use tracing_subscriber::{ EnvFilter, fmt::time};

pub fn init_log() {
    // tracing_subscriber::fmt::init();
    // let time_format = time::format_description::parse("[hour]:[minute]:[second]").unwrap();
    // 初始化日志,设置时区为GMT+8
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info,anserver=debug"))
        .with_target(false)
        // .with_ansi(false)
        .with_file(true).with_line_number(true)
        .with_timer(time::LocalTime::rfc_3339())
        .init();

}