/*
 * @Author: plucky
 * @Date: 2023-07-01 17:27:54
 * @LastEditTime: 2023-07-01 22:42:08
 */

use std::{collections::hash_map::DefaultHasher, hash::Hasher};

/// 生成Hash唯一ID
pub fn get_unique_id<T>(val: &T) -> u64 
    where T: std::hash::Hash
{
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}