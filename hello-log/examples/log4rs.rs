/*
 * @Author: plucky
 * @Date: 2023-03-06 08:52:42
 * @LastEditTime: 2023-03-06 08:53:11
 * @Description: 
 */


fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    log::info!("Hello, world!");
    // log::info!(target:"hello", "Hello, world!");
    log::info!(target:"hello`", "Hello, world!");

}