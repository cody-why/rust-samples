/*
 * @Author: plucky
 * @Date: 2022-11-25 12:06:31
 * @LastEditTime: 2022-11-25 22:02:04
 * @Description:
 */

// GAT 泛型关联类型

#![allow(dead_code)]

use std::{ops::Deref,rc::Rc,sync::Arc};

// 这是一个trait,它有一个关联类型,这个关联类型是一个泛型
pub trait PointerFamily {
    type Pointer<T>: Deref<Target = T>;
    fn new<T>(value: T) -> Self::Pointer<T>;
}

struct ArcFamily;

// 这是一个实现了PointerFamily trait的结构体
// 泛型关联类型,可以存放任意类型的值
impl PointerFamily for ArcFamily {
    type Pointer<T> = Arc<T>;
    fn new<T>(value: T) -> Self::Pointer<T> {
        Arc::new(value)
    }
}

struct RcFamily;

impl PointerFamily for RcFamily {
    type Pointer<T> = Rc<T>;
    fn new<T>(value: T) -> Self::Pointer<T> {
        Rc::new(value)
    }
}


// 他指定了类型为trait的关联类型
pub struct Foo<P:PointerFamily>{
    bar:P::Pointer<String>,
}

impl <P:PointerFamily>Foo<P>{
    pub fn new(value:String)->Self{
        Self{
            bar:P::new(value),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer() {
        let arc = ArcFamily::new("arc".to_string());
        let rc = RcFamily::new(String::from("rc"));

        dbg!(arc);
        dbg!(rc);

        let foo = Foo::<ArcFamily>{
            bar:ArcFamily::new("bar".to_string()),
        };

        let foo2 = Foo::<RcFamily>::new(String::from("foo2"));
        dbg!(foo.bar);
        dbg!(foo2.bar);
    }
}