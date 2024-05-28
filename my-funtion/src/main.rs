/*
 * @Date: 2024-05-15 19:37:14
 * @LastEditTime: 2024-05-15 20:28:02
 */

 // 类似axum的Router，将不同参数的处理器传递

use my_funtion::*;

fn main() {
    let context = Context::new("magic".into(), 33);
    // 类似axum的Router，将不同参数的处理器传递给trigger
    trigger(context.clone(), print_id);
    trigger(context.clone(), print_param);
    trigger(context.clone(), print_all);


}



// 传递不同参数的处理器, 要实现Handler 
pub fn trigger<T, H>(context: Context, handler: H)
where
    H: Handler<T>,
{
    handler.call(context);
}

// 方法1, 打印id
// print_id 是 Fn(Id) 类型，它实现了 Handler<Id>，当调用 handler.call 方法时，相当于执行如下代码：
// print_id(Id::from_context(&context));
fn print_id(id: Id) {
    println!("id is {}", id.0);
}

// 方法2, 打印param
fn print_param(Param(param): Param) {
    println!("param is {}", param);
}

// 方法3, 打印id和param
fn print_all(Param(param): Param, Id(id): Id) {
    println!("param is {param}, id is {id}");
}
