/*
 * @Author: plucky
 * @Date: 2022-06-05 22:57:54
 * @LastEditTime: 2023-11-05 16:51:19
 * @Description: 
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/helloworld.proto");
    println!("cargo:rerun-if-changed=proto/echo.proto");
    
    // tonic_build::compile_protos("proto/helloworld.proto")?;
    // tonic_build::compile_protos("proto/echo.proto")?;
    tonic_build::configure().out_dir("src/proto").compile(&["proto/helloworld.proto", "proto/echo.proto"], &["proto"])?;
    
    Ok(())
}