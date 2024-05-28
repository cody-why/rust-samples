/*
 * @Author: plucky
 * @Date: 2023-04-21 20:03:43
 * @LastEditTime: 2023-04-22 17:17:29
 * @Description: 
 */

// 抛弃面向对象思想.分离出一个第3方registry负责注册worker

use std::time::Duration;

struct Master {
    counter: i32,
}

impl Master {
    fn new() -> Master {
        Master {
            counter: 0,
        }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }
    
    fn register<F: Fn(&mut Self)>(self, f: F) -> Registry<F> {
        Registry {
            master: self,
            work: f,
        }
    }
}

struct Registry<F> {
    master: Master,
    work: F,
}

impl<F: Fn(&mut Master)> Registry<F> {
    fn run(&mut self) {
        let Self { ref mut master, ref mut work } = *self;
        work(master)
    }
}

fn simple_work (m: &mut Master) {
    loop {
        std::thread::sleep(Duration::from_secs(1));
        m.increment();
        println!("{}", m.counter);
    }
}

fn main() {
    let m = Master::new();
    let mut reg = m.register(simple_work);
    reg.run();
}