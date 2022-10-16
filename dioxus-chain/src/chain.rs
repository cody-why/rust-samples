/*
 * @Author: plucky
 * @Date: 2022-10-09 20:05:24
 * @LastEditTime: 2022-10-09 23:26:07
 * @Description: 
 */

use crate::{block::Block, storage::{FileStorage, Storage}};

/// 区块链
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Chain {
        // load from storage
        let storage = FileStorage::default();
        let data = storage.load();
        match serde_json::from_str(&data) {
            Ok(blocks) => Chain { blocks },
            Err(_) => {
                let genesis_block = Block::new(0, "Genesis Block".to_string(), "no previous".to_string());
                Chain {blocks: vec![genesis_block] }
            }
        }
        
    }

    pub fn add_block(&mut self, data: String) {
        match self.blocks.last() {
            Some(block) => {
                let new_block = Block::new(block.index + 1, data, block.get_hash());
                self.blocks.push(new_block);
            }
            None => {
                let genesis_block = Block::new(0, data, "no previous".to_string());
                self.blocks.push(genesis_block);
            }
        }
        self.save();
    }

    pub fn get_totals(&self) -> usize {
        self.blocks.len()
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn get_block(&self, index: usize) -> Option<Block> {
        self.blocks.get(index).cloned()
    }

    pub fn save(&self) {
        let data = serde_json::to_string(&self.blocks).unwrap();
        let storage = FileStorage::default();
        storage.save(data);

    }
}