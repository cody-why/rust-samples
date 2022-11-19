/*
 * @Author: plucky
 * @Date: 2022-11-11 14:07:19
 * @LastEditTime: 2022-11-16 18:14:35
 * @Description: 
 */

use std::{ sync::{Arc, Mutex, RwLock}};


fn main(){
    let s = Arc::new(Mutex::new(vec![0]));
    let b = s.clone();
    s.lock().unwrap().push(1);
    // 测试clone数据是否一样
    println!("{:?}, {:?}",s, b);


    let s = Arc::new(RwLock::new(vec![0]));
    let b = s.clone();
    s.write().unwrap().push(1);
    // 测试clone数据是否一样
    println!("{:?}, {:?}",s, b);


}
