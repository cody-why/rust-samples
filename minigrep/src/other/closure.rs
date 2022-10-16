use std::{thread, time::Duration};

/// 一个缓存,不管读取多少次value,只执行一次闭包
struct Cacher<T>
where
    T: Fn(u32) -> u32, //约束T是一个Fn函数
{
    closure: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(closure: T) -> Self {
        Self {
            closure,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(num) => num,
            None => {
                // 怎么调用闭包呢,写错函数()有提示
                let v = (self.closure)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

/// 一个延时任务
#[allow(dead_code)]
fn generate_workout(intensity: u32) {
    // 一个闭包,延时2秒返回
    let expensive_closure = |num| {
        println!("calculating sLowly");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // 一个缓存,不管读取多少次value,只执行一次闭包
    let mut cacher = Cacher::new(expensive_closure);

    if intensity < 25 {
        //这里,闭包执行了2次
        //let num = expensive_closure(intensity);
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else if intensity < 30 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive_closure(intensity));
    }
}

#[test]
fn test1() {
    generate_workout(10);
}
