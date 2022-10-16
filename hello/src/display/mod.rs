/*** 
 * @Author: plucky
 * @Date: 2022-06-05 17:31:10
 * @LastEditTime: 2022-07-08 00:23:55
 * @Description: 
 */
/// 展示了文件分层的方法
/// main文件中,use display, 将去找display.rs 或display/mod.rs
/// `mod display` 将找到 `display.rs` 并在它们放到各自的模块中。
pub mod dis;
mod tests;
/// mod tests没有外部用的东西,是私用的

// display::display();
pub fn display() {
    println!("from mod.rs!")
}

pub use dis::*;