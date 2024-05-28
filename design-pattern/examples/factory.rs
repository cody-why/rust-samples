/*
 * @Author: plucky
 * @Date: 2023-05-24 01:33:17
 * @LastEditTime: 2023-05-24 01:37:12
 * @Description:
 */
// 工厂模式
// 定义一个 trait 来标识可创建的对象
trait Product {
    fn create() -> Self;
    fn info(&self);
}
// 实现具体的 Product 类型
struct ConcreteProductA {}
impl Product for ConcreteProductA {
    fn create() -> ConcreteProductA {
        ConcreteProductA {}
    }
    fn info(&self) {
        println!("This is ConcreteProductA");
    }
}
struct ConcreteProductB {}
impl Product for ConcreteProductB {
    fn create() -> ConcreteProductB {
        ConcreteProductB {}
    }
    fn info(&self) {
        println!("This is ConcreteProductB");
    }
}
// 实现工厂类
struct Factory {}
impl Factory {
    fn create_product<T: Product + 'static>(&self) -> T {
        T::create()
    }
}
fn main() {
    let factory = Factory {};
    let product_a = factory.create_product::<ConcreteProductA>();
    let product_b = factory.create_product::<ConcreteProductB>();
    product_a.info();
    product_b.info();
}
