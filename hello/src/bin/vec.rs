fn main() {}

#[derive(Debug)]
#[allow(dead_code)]
enum SomeCell {
    Int(i32), // 枚举可以在项里存放数据
    Float(f32),
    Text(String),
}

#[test]
fn test3() {
    let row = vec![
        SomeCell::Int(3),
        SomeCell::Float(1.8),
        SomeCell::Text("hell0".into()),
    ];
    println!("{:?}", row);

    match row[0] {
        SomeCell::Int(i) => println!("{}", i),
        SomeCell::Float(_) | SomeCell::Text(_) => todo!(),
    }
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
