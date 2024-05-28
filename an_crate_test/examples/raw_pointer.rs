/*
 * @Author: plucky
 * @Date: 2023-08-29 18:34:38
 * @LastEditTime: 2023-08-29 18:41:51
 */

 #[tokio::main]
async fn main() {
    let mut a = 1;
    let w = Wrapper(&mut a);
    
    tokio::spawn(async move {
        println!("{:?}", w);
        unsafe {
            *w.0 = 2;
            println!("{:?}", *w.0);
        }

    }).await.unwrap();
}

#[derive(Debug)]
struct Wrapper(*mut i32);

unsafe impl Send for Wrapper {}
unsafe impl Sync for Wrapper {}

