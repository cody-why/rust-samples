/*
 * @Author: anger
 * @Date: 2022-06-02 15:37:45
 * @LastEditTime: 2022-06-23 23:34:08
 * @Description: 
 */
/// 演示结构和枚举
// 这个宏实现结构打印,可以用println!("{:?}")输出
#[derive(Debug)]
pub struct ClassName {
    field: i32,
}

/// 实现结构方法
impl ClassName {
    //类方法
    pub fn new(value: i32) -> Self {
        Self { field: value }
    }

    //公开方法
    pub fn public_method(&self) {
        println!("from public method {}", self.field);
        self.private_method();
    }

    //私用方法
    fn private_method(&self) {
        println!("from private method {}", self.field);
    }
}

//enum 也可以有方法
#[allow(dead_code)]
pub enum EnumName {
    A,
    B,
}

impl EnumName {
    #[allow(dead_code)]
    pub fn some_method(&self) {}
}
