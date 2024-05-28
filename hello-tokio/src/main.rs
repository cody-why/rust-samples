/*
 * @Author: plucky
 * @Date: 2022-06-10 09:10:55
 * @LastEditTime: 2023-03-16 19:08:16
 * @Description: 
 */



use std::future::Future;

use tokio::{fs, try_join};
use anyhow::Result;

/// 这是一个异步的例子


//anyhow::Result 代替 Result<(), Box<dyn Error>>

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    //方式1,await执行
    //读取文件,IO操作
    let content1 = fs::read_to_string("./Cargo.toml").await?;
    println!("{}", content1);
    
    //let f2:impl Future<Output = Result<String, Error>> = fs::read_to_string("./Cargo.Lock");
    // await相当于调用了 Future的poll
    /* match f2.poll(cx) {
        Poll::Ready (result) => return result,
        Poll::Pending => return Poll:: Pending
    } */
    

    //方式2:
    let f1 = fs::read_to_string("./Cargo.toml");
    let f2 = fs::read_to_string("./Cargo.Lock");
    //串行执行
    let (content1, _content2) = try_join!(f1, f2)?;
    //println!("{} {}",content1, content2);
    println!("-------1-------");

    let yaml1 = toml2yaml(&content1)?;
    //let _yaml2 = toml2yaml(content2.as_str())?;
    println!("{}", yaml1);
    
    //进程-＞多个线程-＞还有多个用户态协程(Future）依赖 executor（负责协程的调试）
    say_hello1("jack").await;
    say_hello2("leo").await;
    
    // fut: core::future::from_generator::GenFuture
    let fut = async { 42 };
    println!("type of fut is: {}", get_type_name (&fut));

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    // let value:Value = toml::from_str(&content)?;
    // Ok(serde_yaml::to_string(&value)?)
    Ok(content.to_string())
}

#[allow(dead_code)]
async fn say_hello1(name: &str)->usize{
    println!("1: Hello {}",name);
    33
}

//async fn 关键字相当于一个返回 impl Future<Output>的语法
fn say_hello2<'fut> (name: &'fut str) -> impl Future<Output = usize> + 'fut{
    async move {
        println!("2: Hello {}", name);
        42
    }
}

fn get_type_name<T>(_: &T)->&'static str {
    std::any::type_name::<T>()
}