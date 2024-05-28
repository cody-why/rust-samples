/*
 * @Author: plucky
 * @Date: 2023-07-15 10:33:27
 * @LastEditTime: 2023-07-15 11:55:15
 */

fn main(){
    let mut data: [usize; 4] = [0, 0, 0, 0];
    let mut str = String::new();
    //数组的数字0或1,有多少种组合
    for i in 0..2{
        for j in 0..2{
            for k in 0..2{
                for l in 0..2{
                    data[0] = i;
                    data[1] = j;
                    data[2] = k;
                    data[3] = l;
                    str.push_str(format!("{i}{j}{k}{l},").as_str());
                }
            }
        }
    }

    println!("{}", str);

    let a = Some("abc".to_string());
    //match 消耗值, unwrap() 消耗值, as_ref() 不消耗
    // match a {
    //     Some(s) => println!("{}", s),
    //     None => println!("None"),
        
    // }
    println!("{}", a.as_ref().unwrap());
    println!("{}", a.as_ref().unwrap());


}