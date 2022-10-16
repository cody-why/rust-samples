use std::ops::Deref;

///模拟Box
struct MyBox<T>(T);

#[allow(dead_code)]
impl<T> MyBox<T> {
    fn new(arg: T) -> Self {
        Self(arg)
    }
}

// 实现deref,即可以使用*MyBox解引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//实现Drop, 释放内存,自动调用,或者用std::mem::drog调用
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop for MyBox");
    }
}

#[test]
fn name() {
    let my_box = MyBox::new(5);
    assert_eq!(*my_box, 5);
}
