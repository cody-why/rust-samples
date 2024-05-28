/*
 * @Author: plucky
 * @Date: 2023-05-24 01:37:51
 * @LastEditTime: 2023-05-24 22:10:22
 * @Description: 
 */
// 构建者模式
// 定义一个 Builder trait 来标识 Builder
trait Builder {
    type Item;
    fn new() -> Self;
    fn build(&self) -> Self::Item;
}
// 定义将要构建的结构体
struct Product {
    name: String,
    price: f64,
    count: u32
}
// 建造者
struct ProductBuilder {
    name: Option<String>,
    price: Option<f64>,
    count: Option<u32>
}
// 实现 Builder trait
impl Builder for ProductBuilder {
    type Item = Product;
    fn new() -> ProductBuilder {
        ProductBuilder {
            name: None,
            price: None,
            count: None
        }
    }
    fn build(&self) -> Product {
        Product {
            name: self.name.clone().unwrap(),
            price: self.price.unwrap_or(0.0),
            count: self.count.unwrap_or(0)
        }
    }
}
// 建造者中的方法
impl ProductBuilder {
    fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }
    fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }
}
// 使用建造者构建对象
fn main() {
    let product = ProductBuilder::new()
                    .name("Product A")
                    .price(15.99)
                    .count(100)
                    .build();
    println!("{} has price {} and count {}.", product.name, product.price, product.count);
}

// 在上面的例子中，我们首先定义了一个  Builder  trait，它包含两个方法： new()  用于创建 Builder 对象， build()  用于构建将要生成的对象。
// 然后我们定义了一个将要生成的结构体  Product ，它包含三个字段：名称、价格、数量。接着，我们定义了一个  ProductBuilder  结构体，它包含了三个可选字段，用于构建  Product  对象。为了实现  Builder  trait，我们在  ProductBuilder  结构体中实现了  new()  和  build()  方法。
// 在  build()  方法中，我们使用  unwrap()  方法获取  Option  类型中的值，如果值不存在则使用默认值。此外，我们还在  ProductBuilder  结构体中实现了三个方法，用于设置可选字段的值，每个方法都返回  Self ，这使得我们可以使用方法链接来设置字段的值。
// 在  main()  函数中，我们通过创建一个  ProductBuilder  对象，并使用链式方法调用来设置字段的值，最后调用  build()  方法构建出一个  Product  对象，并打印它的信息。