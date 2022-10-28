/*
 * @Author: plucky
 * @Date: 2022-10-28 11:16:55
 * @LastEditTime: 2022-10-28 14:41:52
 * @Description: 
 */

use std::path::PathBuf;

// ./build.rs
fn main() {
    // dubbo_build::prost::configure()
    //     .compile(&["proto/greeter.proto"], &["proto/"])
    //     .unwrap();

    let path = PathBuf::from("./src/protos");
    println!("path: {:?}", path);
    dubbo_build::prost::configure()
        // .output_dir(path)
        .compile(&["protos/greeter.proto","protos/echo.proto"], &["protos/"])
        .unwrap();
}