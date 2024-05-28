#![allow(dead_code)]
use std::{rc::Rc, sync::{Arc, Mutex}, fmt::Debug};

fn main (){
    let a = 10; // 在栈中的整数
    let b = Box::new(20); // 在堆中的整数,也叫装箱的整数
    let c = Rc::new(Box::new(30)); // 包装在一个引用计数的装箱的整数
    let d = Arc::new(Mutex::new(40)); // 包装在一个原子引用计数的整数,并由一个互斥锁保护

    println!("a = {a}, b = {b}, c = {c}, d = {d:?}");

    let w = W::new(a);
    println!("a == *w? {}", a==*w); // true
        
}

// 用泛型包装一个新类型T
struct W<T> (pub T);

impl <T> W<T> {
    fn new(t: T) -> W<T> {
        W(t)
    }
}

// 实现Deref trait,使得W<T>可以被解引用为T
impl <T> std::ops::Deref for W<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}


trait A {
    fn a(&self);
}
// 用3种方法传递trait参数

//fn print_a(a: impl A+Debug) {
fn print_a<T>(a: T) 
    where T: A+Debug
{
    println!("{:?}", a);
    a.a();
}

// 用into方法兼用string和&str
fn into_string<S: Into<String>>(a: S) -> String {
    a.into()
}


// 当需要vec存储不同类型时,用enum包装多种类型
// 或者用Box<Any>包装多种类型
enum E {
    A(i32),
    B(String),
    C(Box<i32>),
}