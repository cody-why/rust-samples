use std::collections::HashMap;

fn main() {}

#[test]
fn test() {
    test1();
    test2();
}

#[allow(unused)]
fn test1() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 用空格分割字符串
    for word in text.split_whitespace() {
        //如果不存在key则插入0,存在则返回val
        let count = map.entry(word).or_insert(0);
        *count += 1; // 再把val加1
    }

    println!("{:?}", map);
}

#[allow(dead_code)]
fn test2() {
    let color = vec!["Blue", "Yellow"];
    let valus = vec![100, 200];
    //color.iter().zip(valus.iter())2个vec组成组合元组
    //collect() 组成集合
    let map: HashMap<_, _> = color.iter().zip(valus.iter()).collect();
    //println!("{:?}", color.iter().zip(valus.iter()));
    println!("{:?}", map);

    let mut map = HashMap::new();
    map.insert("k", 10);
    map.insert("s", 20);

    let v = map.get("s");
    if let Some(a) = v {
        println!("1.Get value:{} ", a);
    }
    match v {
        Some(a) => println!("2.Get value:{} ", a),
        None => println!("None value "),
    }

    // map 的遍历
    // 要继续使用,不要move,要引用&map
    for (k, v) in &map {
        println!("3.Get key: {} value:{} ", k, v);
    }
    // 如果key不存在则插入30,存在则返回val
    map.entry("u").or_insert(30);
}
