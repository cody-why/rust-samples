use  mymacro_rules::my_vec;

/// 使用属性宏,传进一个属性
#[mymacro::my_first_attribute_proc_macro("abciou")]
fn add(a:i32, b:i32) { println!("{} + {} = {}", a, b, a+b);}

// #[derive(macro_define_crate: : FirstDeriveProcMacro)]
// struct car;

fn main() {
    //调用声明宏
    let v= my_vec!(1,2,3);
    println!("{:?}",v);

    //调用函数,编译后就有属性宏传递的值
    add(1, 5);
    
}

#[mymacro::flaky_test]
fn my_test() {
  assert_eq!(1, 2);
}
