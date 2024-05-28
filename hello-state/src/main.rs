/*
 * @Author: plucky
 * @Date: 2022-12-14 22:28:13
 * @LastEditTime: 2023-06-21 11:03:56
 * @Description:
 */

use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

use state::TypeMap;
use state::InitCell;

// 多类型存储
pub static APP_CONTEXT: TypeMap![Send + Sync] = <TypeMap![Send + Sync]>::new();
// Read/Write Singleton
static GLOBAL_MAP: InitCell<Mutex<HashMap<String, String>>> = InitCell::new();

// #[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub cell: AtomicI32,
}

impl Config {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            cell: AtomicI32::new(0),
        }
    }
}

fn main() {
    // 测试存储不同类型的全局变量
    APP_CONTEXT.set(Config::new(8080)); // true
    APP_CONTEXT.set(Config::new(8081)); // false
    let config = APP_CONTEXT.get::<Config>();
    println!("{:?}", config);
    config.cell.store(99, Ordering::Release);
    let config = APP_CONTEXT.get::<Config>();
    println!("{:?}", config);
    
    // 存储线程移动的可变类型
    let value =  Mutex::new(Cell::new(10));
    APP_CONTEXT.set(value);
    APP_CONTEXT.get::<Mutex<Cell<i32>>>().lock().unwrap().set(99);
    println!("{:?}", APP_CONTEXT.get::<Mutex<Cell<i32>>>().lock().unwrap().get());
    
    // 存储String
    APP_CONTEXT.set::<String>("hello".to_string());
    let str = APP_CONTEXT.get::<String>();
    println!("{:?}", str);

    // 测试指定类型的全局变量
    let mut initial_map = HashMap::new();
    initial_map.insert("key".into(), "value".into());
    GLOBAL_MAP.set(Mutex::new(initial_map));

    let mut map = GLOBAL_MAP.get().lock().unwrap();
    map.insert("k".into(), "v".into());
    println!("{:?}", map.get("key"));
    println!("{:?}", map.get("k"));
}