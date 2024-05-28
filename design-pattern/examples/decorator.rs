/*
 * @Author: plucky
 * @Date: 2023-05-24 22:17:12
 * @LastEditTime: 2023-05-24 22:24:40
 * @Description: 
 */
// 装饰器模式
// 定义一个 trait 来标识装饰器
trait Shape {
    fn draw(&self) -> String;
}
// 创建实现了 Shape trait 的实体类
 struct Rectangle;
 impl Shape for Rectangle {
    fn draw(&self) -> String {
        String::from("Drawing a rectangle.")
    }
}
// 创建实现了 Shape trait 的实体类
 struct Circle;
 impl Shape for Circle {
    fn draw(&self) -> String {
        String::from("Drawing a circle.")
    }
}
// 创建实现了 Shape trait 的装饰器类
 struct ShapeDecorator<S: Shape> {
    shape: S,
}
 impl<S: Shape> Shape for ShapeDecorator<S> {
    fn draw(&self) -> String {
        self.shape.draw()
    }
}
// 创建扩展了 ShapeDecorator 类的实体装饰器类
 struct RedShapeDecorator<S: Shape> {
    shape: S,
}
 impl<S: Shape> Shape for RedShapeDecorator<S> {
    fn draw(&self) -> String {
        format!("{} {}", self.shape.draw(), "Filling the shape with red color.")
    }
}
 fn main() {
    // 创建一个简单的矩形和圆形
    let rectangle = Rectangle;
    let circle = Circle;
    // 创建装饰后的形状
    let red_rectangle = RedShapeDecorator { shape: rectangle };
    let red_circle = RedShapeDecorator { shape: circle };
    // 绘制形状
    // println!("{}", rectangle.draw());
    // println!("{}", circle.draw());
    println!("{}", red_rectangle.draw());
    println!("{}", red_circle.draw());

    let red_rectangle = ShapeDecorator { shape: Rectangle };
    let red_circle = ShapeDecorator { shape: Circle };
    println!("{}", red_rectangle.draw());
    println!("{}", red_circle.draw());
}

// 装饰器模式如何允许我们通过包装一个或多个实现相同接口的包装器对象来在运行时动态地向对象添加行为的方式。