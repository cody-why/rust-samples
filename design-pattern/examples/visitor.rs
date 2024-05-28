/*
 * @Author: plucky
 * @Date: 2023-05-26 23:21:11
 * @LastEditTime: 2023-05-26 23:40:14
 * @Description: 
 */
// 访问者模式

#![allow(unused)]

use std::any::Any;

trait ShapeVisitor {
    fn visit_circle(&self, circle: &Circle);
    fn visit_rectangle(&self, rectangle: &Rectangle);
    fn visit_triangle(&self, triangle: &Triangle);
  }
   struct ShapeCollection(Vec<Box<dyn Shape>>);
   impl ShapeCollection {
    fn accept(&self, visitor: &impl ShapeVisitor) {
      for shape in &self.0 {
        if let Some(circle) = shape.as_any().downcast_ref::<Circle>() {
          visitor.visit_circle(&circle);
        } else if let Some(rectangle) = shape.as_any().downcast_ref::<Rectangle>() {
          visitor.visit_rectangle(&rectangle);
        } else if let Some(triangle) = shape.as_any().downcast_ref::<Triangle>() {
          visitor.visit_triangle(&triangle);
        }
      }
    }
  }
   struct EdgeCountVisitor {}
   impl ShapeVisitor for EdgeCountVisitor {
    fn visit_circle(&self, circle: &Circle) {
      println!("Circle has 1 edge.");
    }
     fn visit_rectangle(&self, rectangle: &Rectangle) {
      println!("Rectangle has 4 edges.");
    }
     fn visit_triangle(&self, triangle: &Triangle) {
      println!("Triangle has 3 edges.");
    }
  }
   fn main() {
    let mut shape_collection = ShapeCollection(vec![
      Box::new(Circle {}),
      Box::new(Rectangle {}),
      Box::new(Triangle {}),
    ]);
     let edge_count_visitor = EdgeCountVisitor {};
     shape_collection.accept(&edge_count_visitor);
  }
   trait Shape {
    fn as_any(&self) -> &dyn Any;
  }
   struct Circle {}
   impl Shape for Circle {
    fn as_any(&self) -> &dyn Any {
      self
    }
  }
   struct Rectangle {}
   impl Shape for Rectangle {
    fn as_any(&self) -> &dyn Any {
      self
    }
  }
   struct Triangle {}
   impl Shape for Triangle {
    fn as_any(&self) -> &dyn Any {
      self
    }
  }


// Rust中的访问者模式是一种行为模式，它允许您定义一些操作，这些操作可以在对象结构的元素上执行，而不必改变这些元素的类。该模式通过将操作封装为访问者对象来实现，该对象可以在不修改元素类的情况下访问并操作元素。 
// 在这个例子中，我们定义了三个图形对象:  Circle ,  Rectangle , 和  Triangle ，和一个访问者接口  ShapeVisitor 。我们还定义了一个  ShapeCollection  类，它存储了所有的图形对象，并实现了一个  accept  方法，该方法接受一个访问者对象并让每个图形对象调用访问

