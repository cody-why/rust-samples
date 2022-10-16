mod class1;
mod display;
mod yes;

mod slice;

/// 第一个rust例子
///
fn main() {
    println!("Hello, world!");
    test1();
    let object = class1::ClassName::new(1024);
    object.public_method();
    println!("{:?}", object);
    slice::test_slice();

    //调用display/mod.rs的方法
    display::display();
    //调用display/dis.rs里的方法()
    display::dis::display();
}

/// 学rust,演示输出,所有权,可变变量
fn test1() {
    let a = "Helo";
    println!("{{括号里面有内容{0}}}", a);

    //可变变量要加mut
    #[allow(unused)]
    let mut str1 = "world";
    str1 = "mut";
    assert_eq!("mut", str1);

    let b = 123;
    const C: i32 = 456;

    println!("{0},{1},{2}", str1, b, C);

    //克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();
    //let s2 = s1; //这样会把s1的堆赋值给s2,s1失效
    let s3 = &s1;

    println!("s1 = {}, s2 = {}, s3 = {} {}", s1, s2, s3, s1 == s2);

    //可以重新定义变量
    let mut s1 = String::from("hello");
    let s4 = &mut s1;
    // s4 是可变的引用
    s4.push_str(" from");
    println!("s4 = {}", s4);
}

#[allow(dead_code)]
#[test]
pub fn test2() {
    //1.59 版本引入：可以在赋值语句的左式中使用元组、切片或结构体进行匹配赋值
    let (x, y);
    (x, ..) = (3, 4); // 赋值x=3,省略4
    [.., y] = [1, 2]; // 赋值=2,省略1
                      // 填空，让代码工作
    assert_eq!([x, y], [3, 2]);

    let a: i32 = "123".parse().expect("not a number");
    assert_eq!(a, 123);

    //断言通过,精度比较0.1+0.2==0.3
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001); //true
                                                    //断言不通过,因为f64浮点数上不能测试相等性
                                                    //assert!(0.1 + 0.2 == 0.3);
    println!("{}", 0.1 + 0.2 == 0.3); //false

    //f32的精度可以通过相等性测试
    let abc: (f32, f32, f32) = (0.111, 0.251, 0.362);
    println!("{}", abc.0 + abc.1 == abc.2);
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());

    println!("{}", hello::type_of(&1)); //i32
    println!("{}", 'a' as u8); //97

    println!("{:?}", std::ops::Range { start: 1, end: 5 }); //1..5
    println!("{:?}", std::ops::RangeInclusive::new(1, 5)); //1..=5

    let c1 = 'a';
    assert_eq!(std::mem::size_of_val(&c1), 4); //char 占4位

    let c2 = '中';
    assert_eq!(std::mem::size_of_val(&c2), 4);
}

#[test]
fn test3() {
    let mut s = String::from("hello, ");

    let r1 = &mut s; //第一次可变引用
    r1.push_str("world");
    let r2 = &mut s; //再次可变引用,r1的所有权已经没有
    r2.push_str("!");

    //println!("{}",r1);//报错:不能多次可变引用,如果不用r1,就没有问题

    println!("{}", s);
}

// 比较浮点数的大小
#[test]
fn test_less(){
    let a= 0.1;
    let b = 0.2;
    let c = 0.3;

    // EPSILON是可以接受的误差范围,表示两个浮点数相等
    assert!(a+b - c < f64::EPSILON);
    //assert!(a+b==c);// 不能这样比较,因为浮点数不能精确表示

    let a = 0.1_f32;
    let b = 0.2_f32;
    let c = 0.3_f32;
    // 转为字节查看,f32的精度可以通过相等性测试
    println!("a+b={:x},c={:x}",(a+b).to_bits(),c.to_bits());
    assert!(a+b == c );

}