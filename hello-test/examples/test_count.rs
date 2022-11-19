/*
 * @Author: plucky
 * @Date: 2022-11-16 16:50:55
 * @LastEditTime: 2022-11-17 15:27:34
 * @Description: 
 */

fn main(){
    let count = 3;
    let mut n = 0;
    let mut v= vec![];
    for i in 15..37 {
        v.push(i);
        if v.len()>=count {
            let t = format!("{:?}", v).replace(" ", "");
            println!("{}",t);
            n += 1;
            v.clear();
        }
        
    }
    println!("{:?}",v);
    println!("n={}", n);
}