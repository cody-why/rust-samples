/*
 * @Author: anger
 * @Date: 2022-06-23 22:33:21
 * @LastEditTime: 2022-06-24 09:43:01
 * @Description:
 */

#[allow(unused)]
// 导入其他语言ABI
extern "C" {
    fn abs(a:i32)->i32;
}
/// 一个导出ABI函数,#[no_mangle]表示编译不改变函数名
#[no_mangle]
pub extern "C" fn call_rust(){
    print!("Just called rust function!");
}


#[cfg(test)]
mod tests {
    use core::slice;
    use std::usize;

    #[allow(unused)]
    #[test]
    fn test_raw_pointer() {
        let mut m = 5;
        // 把引用变不可变的原始指针
        let p1 = &m as *const i32;
        // 把引用变可变的原始指针
        let p2 = &mut m as *mut i32;
        // 使用原始指针需要在unsafe里
        unsafe {
            //*p1 += 1;
            *p2 += 1;
            println!("{}", *p1);
            println!("{}", *p2);
        }

        unsafe {
            // 对一个不存在的内存地址操作
            let address = 0x12345usize;
            let _r = address as *const i32;
            //println!("{}",*r);
        }
    }

    #[test]
    fn test_raw_vec() {
        let mut v = vec![1, 2, 3, 4, 5];
        let (a, b) = v.split_at_mut(3);
        println!("{:?},{:?}", a, b);
    }

    /// v.split_at_mut的源码,示例指针的偏移
    #[allow(unused)]
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        assert!(mid > len);
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len-mid),
            )
        }
    }
    
   

}
