#[test]
fn test1() {
    {
        //'b
        let _a = 1;
    }
    //'a
    //println!("{}",a);

    let b = "hello".to_string();

    let l = &largest("abc", &b);
    println!("Largest str is: {}", l)
}

#[test]
fn test2() {
    // 'a
    let _b = "hello".to_string();
    let l = "";
    { // 'b
         // l 的生命周期是 'a, one参数的生命周期是'b,不够长,所以报错
         //l = &largest("abc", &_b);
    }

    println!("Largest str is: {}", l);
}

/// 'a 参数和返回值和函数必须拥有相同的生命周期,'a 实际是参数1,参数2中的生命周期最短的那个
/// 泛型生命周期声明在函数名后面加<'a>,小写字母,通常是a
#[allow(dead_code)]
fn largest<'a>(one: &'a str, two: &'a str) -> &'a str {
    if one.len() > two.len() {
        one
    } else {
        two
    }
}
