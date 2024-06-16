/*
 * @Date: 2023-03-02 17:12:50
 * @LastEditTime: 2024-06-16 21:21:36
 * @Description: cargo build --release --target x86_64-pc-windows-gnu 
 */

 use std::{io::Write, fs::File, collections::{BTreeMap, HashMap}, sync::{Arc, Mutex}};

 use indexmap::IndexMap;
 use rayon::prelude::*;
 use crate::{translator::translate, Config};
 
 // 读取json文件,转成map
 // 把英文翻译为中文,更新到map中
 // 写入json文件
 pub fn process(config: Config) {
 
     let http_proxy = config.https_proxy.unwrap_or_default();
     if !http_proxy.is_empty() {
         std::env::set_var("https_proxy", &http_proxy);
     }
     
    // 解析json文件,转成有序的map,文件内容是{"hello": {"EN": "English"}}
     // 使用IndexMap,保证顺序
     let file_name = &config.file;
     let index_map= read_json(file_name).unwrap();
      // 把IndexMap转成HashMap,方便并行处理
      let hash_map: HashMap<String, BTreeMap<String, String>> = index_map.clone().into_iter().collect();
     // 复制一个map,用于插入新的键值对
     let index_map = Arc::new(Mutex::new(index_map));
     let file_name = file_name.replace(".json", "")+".t.json";
     // 已经翻译过的语言
     let old_map = read_json(&file_name).unwrap_or_default();
    
     // 使用rayon把map转成并行的map,速度提升
     hash_map.par_iter().for_each(|(key, value)|{
         println!("key: {}, value: {:?}", key, value);
         config.translate.par_iter().for_each(|t|{
             let v = match value.get(&config.json_key) {
                 Some(v)=>v,
                 _ => return,
             }; 
             // 如果已经翻译过了,则跳过
             if let Some(v) = old_map.get(key) {
                 if let Some(v) = v.get(&t.json_key) {
                     if !v.is_empty() {
                        let mut map2 = index_map.lock().unwrap();
                        let val = map2.get_mut(key).unwrap();
                        // 更新map2
                        val.insert(t.json_key.clone(), v.clone());
                        return;
                     }
                 }

             }

             match translate(&config.from, &t.to, v) {
                 Ok(translated) => {
                     let mut map2 = index_map.lock().unwrap();
                     let val = map2.get_mut(key).unwrap();
                     // 更新map2
                     val.insert(t.json_key.clone(), translated);
                     
                 }
                 Err(e) => println!("Something wrong... {:?}", e)
             }
         });
     });
 
     // 写入json文件
     let json = serde_json::to_string_pretty(&*index_map).unwrap();
     write_all(&file_name, json);
     
 }

pub fn read_json(file_name: impl AsRef<str>) -> Result<IndexMap<String, BTreeMap<String, String>>, Box<dyn std::error::Error>> {
    let file = File::open(file_name.as_ref())?;
    Ok(serde_json::from_reader(file)?)
}

fn write_all(file_name: impl AsRef<str>, json: String) {
    let mut file = File::create(file_name.as_ref()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
