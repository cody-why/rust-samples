/*
 * @Author: plucky
 * @Date: 2023-04-20 17:27:39
 * @LastEditTime: 2023-04-22 18:06:39
 * @Description: 
 */


pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;


// 用泛型包装一个新类型T
pub struct W<T> (pub T);