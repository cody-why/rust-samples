
// 展示各种函数

#[allow(dead_code)]
fn add_one(x:i32)->i32{
    x+1
}
//指针函数
//fn 函数指针
#[allow(dead_code)]
fn add_twice(f:fn(i32)->i32,v:i32)->i32{
    f(v)+f(v)
}
//闭包函数,只要实现了Fn(i32)->i32都可以
//Fn FnMut FnOne
#[allow(dead_code)]
fn wrapper_func<T>(t:T,v:i32)->i32
    where T:Fn(i32)->i32{
        t(v)
    
}
//直接返回Fn,编译器不知道尺寸,必须要dyn,所以报错
// fn return_fn()->Fn(i32)->i32{
//     |x|x+1
// }

/// 返回闭包
#[allow(dead_code)]
fn return_fn()->Box< dyn Fn(i32)->i32>{
    //用Box包装,只样编译器知道尺寸
    Box::new(|x|x+1)
}

#[test]
fn test1(){
    //指针函数
    let v = add_twice(add_one,5);
    println!("add_twice v={}",v);

    //闭包
    let v = wrapper_func(|x|x+1, 6);
    println!("wrapper_func v={}",v);
    //实现的函数指针
    let v = wrapper_func(add_one, 6);
    println!("add_one v={}",v);

    //返回闭包
    let c = return_fn();
    println!("return_fn v={}",c(6));

}
