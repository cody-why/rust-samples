//许多运算符都可以被重载，事实上，运算符仅仅是特征方法调用的语法糖。
//例如 a + b 中的 + 是 std::ops::Add 特征的 add 方法调用，因此我们可以为自定义类型实现该特征来支持该类型的加法运算。
use std::ops;

struct Foo;
struct Bar;

#[derive(PartialEq, Debug)]
struct FooBar;

#[derive(PartialEq, Debug)]
struct BarFoo;

// 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

fn main() {
    // 不要修改下面代码
    // 你需要为 FooBar 派生一些特征来让代码工作
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!")
}
