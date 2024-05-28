/*
 * @Author: plucky
 * @Date: 2023-06-20 23:21:58
 * @LastEditTime: 2023-06-21 11:19:38
 * @Description: 
 */

use linkme::distributed_slice;

// 在这个例子中，我们将创建一个分布式切片，其中包含一些常用的链接。
// 这些链接可以在整个 Rust 代码库中使用。

// 首先，我们使用 `distributed_slice` 宏定义一个分布式切片。
#[distributed_slice]
pub static LINKS: [&str] = [..];

// 然后，我们可以在代码中添加链接。
#[distributed_slice(LINKS)]
static GOOGLE: &str = "https://www.google.com";
#[distributed_slice(LINKS)]
static RUST: &str = "https://www.rust-lang.org";
#[distributed_slice(LINKS)]
static GITHUB: &str = "https://github.com";



fn main() {
    // 我们可以在代码的任何地方使用 `LINKS` 切片。
    for link in LINKS {
        println!("{}", link);
    }

    
    println!("Google: {}", GOOGLE);
    
    
}