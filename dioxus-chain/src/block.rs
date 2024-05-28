/*
 * @Author: plucky
 * @Date: 2022-10-09 19:56:45
 * @LastEditTime: 2023-11-01 13:05:10
 * @Description: 
 */

// #![allow(dead_code)]
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

/// 区块
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Block {
    pub index: usize,
    hash: String,
    current_time: DateTime<Utc>,
    pub data:String,
    pub previous_hash: String,
}

fn get_next_hash(index: usize, data: String, previous_hash: String)->(String, DateTime<Utc>){
    let mut hasher = Sha256::new();
    let current_time = Utc::now();
    let data = format!("{}{}{}{}", index, data, previous_hash, current_time);
    hasher.update(data);
    let hash = hasher.finalize();
    let hash = format!("{:x}", hash);
    // let hash = base16ct::lower::encode_string(&hash);
    (hash, current_time)
}

impl Block {
    pub fn new(index: usize, data: String, previous_hash: String) -> Block {
        let (hash, current_time) = get_next_hash(index, data.clone(), previous_hash.clone());
        Block {
            index,
            hash,
            current_time,
            data,
            previous_hash,
        }
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_current_time(&self) -> DateTime<Utc> {
        self.current_time
    }

    
}

