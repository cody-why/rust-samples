/*
 * @Author: plucky
 * @Date: 2023-03-02 17:12:50
 * @LastEditTime: 2024-03-22 18:30:24
 * @Description: cargo build --release --target x86_64-pc-windows-gnu 
 */

use std::{io::Write, fs::File, collections::{BTreeMap, HashMap}, sync::{Arc, Mutex}};

use indexmap::IndexMap;
use rayon::prelude::*;
use serde::Deserialize;
use translator::translate;

mod lang;
mod translator;

#[derive(Debug, Deserialize, Default)]
pub struct Config{
    pub file: String,
    pub from: String,
    pub json_key: String,
    pub translate: Vec<Translate>,
    pub https_proxy: Option<String>,

}

#[derive(Debug, Deserialize)]
pub struct Translate{
    pub to: String,
    pub json_key: String,
}


// sock5代理需要设置环境变量, reqwest打开socks feature
// export https_proxy=socks5://xxx.xx.xx.xx:8080
fn main() {
    init_env();
    let mut file_name = "config.yaml".to_string();
    if std::env::args().len() >1 {
        file_name = std::env::args().nth(1).unwrap();
    }
    process(file_name);

    println!("Press enter key to continue...");
    // std::io::stdin().read_line(&mut String::new()).unwrap();
    
}



fn init_env() {
    std::env::set_var("RUST_LOG", "info");
    // env_logger::init();
    let path = std::env::current_exe().unwrap();
    println!("{}", path.display());
   
    // 如果是release模式,则切换到json文件所在目录
    if cfg!(not(debug_assertions)) {
        let path = path.parent().unwrap();
        std::env::set_current_dir(path).unwrap();
    }

}
// 读取json文件,转成map
// 把英文翻译为中文,更新到map中
// 写入json文件
pub fn process(file_name: String) {
    
    let config:Config = serde_yaml::from_reader(File::open(file_name).unwrap()).unwrap();
    println!("{:?}", config);

    let http_proxy = config.https_proxy.unwrap_or_default();
    if !http_proxy.is_empty() {
        std::env::set_var("https_proxy", &http_proxy);
    }
    
    let file_name = &config.file;
    let file = File::open(file_name).unwrap();
    
    // 解析json文件,转成有序的map,文件内容是{"hello": {"EN": "English"}}
    // 使用IndexMap,保证顺序
    let index_map: IndexMap<String, BTreeMap<String, String>> = serde_json::from_reader(file).unwrap();
    // 复制一个map,用于插入新的键值对
    let map2 = Arc::new(Mutex::new(index_map.clone()));
    
    // 把IndexMap转成HashMap,方便并行处理
    let hash_map: HashMap<String, BTreeMap<String, String>> = index_map.into_iter().collect();
    // 使用rayon把map转成并行的map,速度提升
    hash_map.par_iter().for_each(|(key, value)|{
        println!("key: {}, value: {:?}", key, value);
        config.translate.par_iter().for_each(|t|{
            let v = match value.get(&config.json_key) {
                Some(v)=>v,
                _ => return,
            }; 
            
            match translate(&config.from, &t.to, v) {
                Ok(translated) => {
                    let mut map2 = map2.lock().unwrap();
                    let val = map2.get_mut(key).unwrap();
                    // 更新map2
                    val.insert(t.json_key.clone(), translated);
                    
                }
                Err(e) => println!("Something wrong... {:?}", e)
            }
        });
    });

    // 写入json文件
    let json = serde_json::to_string_pretty(&*map2).unwrap();
    let mut file = File::create(file_name.to_string()+".t.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
    
}

#[test]
pub fn load_config(){
    
    let config:Config = serde_yaml::from_reader(File::open("config.yaml").unwrap()).unwrap();
    println!("{:?}", config);
}