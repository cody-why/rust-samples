/*
 * @Author: plucky
 * @Date: 2023-09-22 17:37:46
 * @LastEditTime: 2023-09-22 18:53:27
 */

use std::{collections::HashMap, fs::File};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config{
    pub urls: Vec<String>,
    pub https_proxys: HashMap<String, String>,
 
}


pub fn load_config(file_name: &str) -> Config {
    init_env();
    serde_yaml::from_reader(File::open(file_name).unwrap()).unwrap()
}

fn init_env() {
    std::env::set_var("RUST_LOG", "info");
    // env_logger::init();
   
    // 如果是release模式,则切换到json文件所在目录
    if cfg!(not(debug_assertions)) {
        let path = std::env::current_exe().unwrap();
        println!("{}", path.display());
        let path = path.parent().unwrap();
        std::env::set_current_dir(path).unwrap();
    }

}

#[test]
fn test_load_config(){
    let config = load_config("app.yaml");
    println!("{:#?}", config);
}