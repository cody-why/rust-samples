/*
 * @Author: plucky
 * @Date: 2023-05-26 23:11:34
 * @LastEditTime: 2023-05-26 23:14:18
 * @Description: 
 */
//责任链模式

trait Handler {
    fn handle_request(&self, request: i32);
    fn set_next_handler(&mut self, next: Box<dyn Handler>);
}
// 具体处理者1
struct ConcreteHandler1 {
    next_handler: Option<Box<dyn Handler>>,
}
impl Handler for ConcreteHandler1 {
    fn handle_request(&self, request: i32) {
        if request >= 0 && request < 10 {
            println!("{} handled by ConcreteHandler1", request);
        } else {
            match &self.next_handler {
                Some(handler) => {
                    println!("ConcreteHandler1 can't handle {}, passing it to next handler", request);
                    handler.handle_request(request);
                }
                None => {
                    println!("No one can handle {}", request);
                }
            }
        }
    }
    fn set_next_handler(&mut self, next: Box<dyn Handler>) {
        self.next_handler = Some(next);
    }
}
// 具体处理者2
struct ConcreteHandler2 {
    next_handler: Option<Box<dyn Handler>>,
}
impl Handler for ConcreteHandler2 {
    fn handle_request(&self, request: i32) {
        if request >= 10 && request < 20 {
            println!("{} handled by ConcreteHandler2", request);
        } else {
            match &self.next_handler {
                Some(handler) => {
                    println!("ConcreteHandler2 can't handle {}, passing it to next handler", request);
                    handler.handle_request(request);
                }
                None => {
                    println!("No one can handle {}", request);
                }
            }
        }
    }
    fn set_next_handler(&mut self, next: Box<dyn Handler>) {
        self.next_handler = Some(next);
    }
}
// 具体处理者3
struct ConcreteHandler3 {
    next_handler: Option<Box<dyn Handler>>,
}
impl Handler for ConcreteHandler3 {
    fn handle_request(&self, request: i32) {
        if request >= 20 && request < 30 {
            println!("{} handled by ConcreteHandler3", request);
        } else {
            match &self.next_handler {
                Some(handler) => {
                    println!("ConcreteHandler3 can't handle {}, passing it to next handler", request);
                    handler.handle_request(request);
                }
                None => {
                    println!("No one can handle {}", request);
                }
            }
        }
    }
    fn set_next_handler(&mut self, next: Box<dyn Handler>) {
        self.next_handler = Some(next);
    }
}
// 客户端代码
fn main() {
    let mut handler1 = ConcreteHandler1 { next_handler: None };
    let mut handler2 = ConcreteHandler2 { next_handler: None };
    let handler3 = ConcreteHandler3 { next_handler: None };
    handler2.set_next_handler(Box::new(handler3));
    handler1.set_next_handler(Box::new(handler2));

    handler1.handle_request(5);
    handler1.handle_request(15);
    handler1.handle_request(25);
}

// 责任链模式（Chain of Responsibility Pattern）是一种行为设计模式。它允许将请求沿着处理链传递，直到有一个处理对象能够处理该请求。责任链模式将处理对象组织成一个链，每个对象都有一个指向下一个处理对象的引用。当一个对象无法处理请求时，它会将请求传递给链中的下一个处理对象，直到有一个处理对象能够处理该请求为止。 
// 责任链模式有以下几个角色组成： 
// 1. 抽象处理者（Handler）：定义处理请求的接口，并将自己的下一个处理者保存在其中。 
// 2. 具体处理者（Concrete Handler）：实现抽象处理者接口，并负责处理请求。如果它不能处理请求，则将请求传递给它的下一个处理者。 
// 3. 客户端（Client）：创建责任链，并向它的第一个处理者发送请求。 
// 责任链模式的优点在于它可以解耦请求的发送者和接收者，同时对于请求的处理和添加新的处理者都很灵活。责任链模式还可以避免请求的发送者和接收者之间的直接耦合，提高代码的可维护性、可扩展性和可重用性。 