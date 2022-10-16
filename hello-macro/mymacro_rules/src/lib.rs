/*** 
 * @Author: plucky
 * @Date: 2022-06-09 17:09:35
 * @LastEditTime: 2022-07-11 16:42:06
 * @Description: 
 */

/// 这是一个声明宏,创建一个Vec,可传入可变参数
#[macro_export]
macro_rules! my_vec { //my_vec! 模仿vec！
    //表达式,匹配0个或多个参数
    ($($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
