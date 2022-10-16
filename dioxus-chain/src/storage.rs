/*
 * @Author: plucky
 * @Date: 2022-10-09 22:47:42
 * @LastEditTime: 2022-10-10 01:21:24
 * @Description: 
 */

use std::io::{Write, Read};

 /// 存储trait
pub trait Storage {
    fn save(&self, data: String);
    fn load(&self) -> String;
}

// 存储在文件中
pub struct FileStorage {
    path: String,
}

impl Default for FileStorage {
    fn default() -> Self {
        let path = if cfg!(debug_assertions) {
            std::env::current_dir().unwrap()
        } else {
            std::env::current_exe().unwrap()
        };

        println!("path: {:?}", path);
        FileStorage {
            path: path.join("blockchain.json").to_string_lossy().to_string(),
        }
    }
}

impl FileStorage {
    pub fn new(path: String) -> FileStorage {
        FileStorage { path }
    }
}

impl Storage for FileStorage {
    fn save(&self, data: String) {
        let mut file = std::fs::File::create(&self.path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    fn load(&self) -> String {
        let contents =  match std::fs::File::open(&self.path) {
            Ok(mut file) => {
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap_or_default();
                data
            }
            Err(_) => "".to_string(),
        };
        contents
    }
}
