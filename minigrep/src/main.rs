use std::{process, env};

mod run;
use run::config;

/// 在文件里查找文本
/// cargo run searchstring abc.txt
/// 
fn main() {
    println!("Hello, world!");
  
    // 出错打印错误,退出程序
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        println!("解析配置时出错:{}",err);
        process::exit(1)
        //失败返回一个结果,程序都退出了,就不用返回了
    });

    println!("{:?}",config);
   
    //let content = fs::read_to_string(config.filename).expect("Read file error");
    if let Err(e) = run::search::run(config) {
        println!("Run error: {}",e);

    }
    
}

