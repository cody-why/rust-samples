use std::{collections::HashMap, fs::File, io::Write, sync::{Arc, Mutex}};

use indexmap::IndexMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{translate, Config};


/// 每个语言一个文件
pub fn process(config: Config) {

    let http_proxy = config.https_proxy.unwrap_or_default();
    if !http_proxy.is_empty() {
        std::env::set_var("https_proxy", &http_proxy);
    }
     // 使用IndexMap,保证顺序
    let file_name = &config.file;
    let index_map = read_json(file_name).unwrap();
    // 把IndexMap转成HashMap,方便并行处理
    let hash_map: HashMap<String, String> = index_map.clone().into_iter().collect();
    // 每个语言一个map
    config.translate.iter().for_each(|t|{
        let file_name = t.json_key.clone();
        // 复制index_map2,保持顺序
        let s_map =  Arc::new(Mutex::new(index_map.clone()));
        
        let old_map = read_json(file_name).unwrap_or_default();

        hash_map.par_iter().for_each(|(key, value)|{
            println!("key: {}, value: {:?}", key, value);

            // 如果已经翻译过了,则跳过
            if let Some(val) = old_map.get(key){
                if !val.is_empty() {
                    s_map.lock().unwrap().insert(key.clone(), val.clone());
                    return;
                }
            }
           
            match translate(&config.from, &t.to, value) {
                Ok(translated) => {
                    // 更新map2
                    s_map.lock().unwrap().insert(key.clone(), translated);
                    
                }
                Err(e) => println!("Something wrong... {:?}", e)
            }
        });
        // 写到文件
        let file_name = if t.json_key.is_empty() {
            format!("{}.json", t.to)
        }else {
            format!("{}.json", t.json_key)
        };
        let json = serde_json::to_string_pretty(&*s_map).unwrap();
        write_all(file_name, json);
    });
    
}


pub fn read_json(file_name: impl AsRef<str>) -> Result<IndexMap<String, String>, Box<dyn std::error::Error>> {
    let file = File::open(file_name.as_ref())?;
    Ok(serde_json::from_reader(file)?)
}

fn write_all(file_name: impl AsRef<str>, json: String) {
    let mut file = File::create(file_name.as_ref()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
