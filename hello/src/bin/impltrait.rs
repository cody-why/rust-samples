/***
 * @Author: plucky
 * @Date: 2022-09-09 22:08:41
 * @LastEditTime: 2022-09-09 22:23:50
 * @Description: https://play.rust-lang.org 输出汇编代码分析编译内存
 */

#![allow(unused)]
/// impl & dyn & trait object

trait Nice {
    fn nice(&self);
}

/// 静态调用,编译器会生成不同类型的函数
fn say_by_impl(n: impl Nice) {
    n.nice();
}

/// 动态调用,只生成一个函数
fn say_by_dyn(n: &dyn Nice) {
    n.nice();
}

fn say_by_trait_object(n: Box<dyn Nice>) {
    n.nice();
}

struct Hello{
    name: String,
}
impl Nice for Hello {
    fn nice(&self) {
        println!("hello {}", self.name);
    }
}


struct  Good{
    v: i32,
}

impl Nice  for Good {
    fn nice(&self) {
        println!("good {}", self.v);
    }
}
    


fn main(){

    let h = Hello{name: "plucky".to_string()};
    //say_by_impl(h);
    say_by_dyn(&h);
    say_by_trait_object(Box::new(h));

    let g = Good{v: 100};
    //say_by_impl(g);
    say_by_dyn(&g);
    say_by_trait_object(Box::new(g));
    
}