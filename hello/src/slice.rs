/// 演示切片的用法
///
pub fn test_slice() {
    //文本切片
    let mut s = String::from("runoob");
    s.push_str(" push string");

    let slice1 = &s[0..3];
    let slice2 = &s[3..];
    //s.push_str("yes!"); // 错误,被切片引用的文本不能修改
    println!("slice1 = {}, slice2 = {}", slice1, slice2);

    //数组切片
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        print!("{} ", i);
    }
    println!("");
}
//vec 的排序
#[test]
fn test1() {
    //对整数排序
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    //对浮点数排序
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

#[test]
fn test2() {
    let mut vec = vec![1, 5, 10, 2, 15];
    //vec 的iter
    for i in &mut vec {
        *i += 100;
    }

    for i in vec {
        println!("{}", i);
    }
}
