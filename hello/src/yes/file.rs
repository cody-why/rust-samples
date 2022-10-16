/*
 * @Author: anger
 * @Date: 2022-06-19 23:39:53
 * @LastEditTime: 2022-06-23 23:32:25
 * @Description: 
 */

 ///文件读写


/// 构造函数用来验证参数的值是否合法
#[allow(dead_code)]
struct Some {
    value: u32,
}
#[allow(dead_code)]
impl Some {
    
   /// It's a constructor.
   ///  Example
   /// ```rust
   /// let s = Some::new(3);
   /// println!("{}", s.value());
   /// ```
    pub fn new(value: u32) -> Self {
        if value < 1 {
            panic!("值不能为0");
        }
        Self { value }
    }

    /// It's a getter
    pub fn value(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::OpenOptions,
        io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    };
    /// 读文件,返回Result,演示?返回
    #[allow(dead_code)]
    fn read_file_return_err() -> Result<String, std::io::Error> {
        let mut buf = String::new();
        let _ = OpenOptions::new()
            .read(true)
            .open("test.txt")?
            .read_to_string(&mut buf)?; //?如果有错误则返回错误
        Ok(buf)
    }

    #[test]
    fn test1() {
        //打开只读,创建只写
        /* let _f = match File::open("test.txt") {
            Ok(f) => f,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => File::create("test.txt").expect("创建文件失败"),
                other => panic!("Error open file: {}", other),
            },
        }; */
        //f.write("buf".as_bytes()).expect("写文件出错");
        //f.read_to_string(&mut buf).expect("读文件出错");

        //用OpenOptions可选读写创建打开文件
        let f = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open("test.txt")
            .expect("打开文件失败");

        let mut w = BufWriter::new(&f);
        w.write_all(b"test\n").expect("写文件出错");
        w.flush().unwrap();

        let mut r = BufReader::new(&f);
        r.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        r.read_to_string(&mut buf).expect("读文件出错");
        println!("{}", buf);
    }
}
