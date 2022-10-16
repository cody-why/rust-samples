/*** 
 * @Author: plucky
 * @Date: 2022-06-17 23:30:03
 * @LastEditTime: 2022-07-13 23:57:52
 * @Description: 
 */

fn main() {
    let mut s = String::from("hello world!");
    let word = first_word(&s);
    //s.clear();//如果用切片返回,则一个不变引用和一个可变引用冲突,改为用String返回,如果不需要清掉,返回&str获得性能

    println!("First word is: {}", word);
    s.clear();

    let s = String::from("您好!");
    //string是utf-8编码
    //遍历String,标量值用chars,字节用bytes
    for v in s.chars() {
        println!("{}", v)
    }
    for v in s.bytes() {
        println!("{}", v)
    }
    //切片,中文字占3个byte
    println!("{}", &s[0..3]);

    // 原始字符串
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果你要在原始字符串中写引号，请在两边加一对 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中需要写 "#，那就在定界符中使用更多的 #。
    // 可使用的 # 的数目没有限制。
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

}

/// 返回第一个单词
/// 参数切片
/// 返回值切片&str,改为用String返回
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    //迭代器,enumerate包装为元组
    for (i, v) in bytes.iter().enumerate() {
        if v == &b' ' {
            //空格字符
            return &s[..i];
        }
    }
    &s[..]
}
