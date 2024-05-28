/*
 * @Author: plucky
 * @Date: 2023-06-20 22:29:44
 * @LastEditTime: 2023-06-21 00:28:32
 * @Description: 
 */


// 使用inventory, 用于注册全局变量

use std::{sync::{Mutex}, cell::{RefCell}};


fn main (){
    for flag in inventory::iter::<Flag> {
        println!("-{}, --{}", flag.short, flag.name);
        
    }

    let flag = inventory::iter::<Flag>.into_iter().next().unwrap();
    println!("first flag: -{}, --{}", flag.short, flag.name);
    
    // let flag = inventory::iter::<Flag>;
    // inventory::replace!(Flag::new('v', "verbose"));

    for flag in inventory::iter::<Flag> {
        println!("-{}, --{}", flag.short, flag.name);
        
    }
    
    
    // flag.short='a';
    
}

pub struct Flag {
    short: char,
    name: &'static str,
    // mutex: Mutex<RefCell<i32>>,
    /* ... */
}

impl Flag {
    pub const fn new(short: char, name: &'static str) -> Self {
        Flag { short, name }
    }
}

inventory::collect!(Flag);

inventory::submit! {
    Flag::new('v', "verbose")
}
