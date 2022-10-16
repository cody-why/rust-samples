use super::config::Config;
use crate::run::search;
use std::{fs, io::Error};

/// 在文件里查找文本
pub fn run(config: Config) -> Result<(), Error> {
    let content = fs::read_to_string(config.filename)?;

    println!("File text:\n{}", content);

    let search = if config.casesensitive {
        search::search(&config.query, &content)
    } else {
        search::search_case_insensitive(&config.query, &content)
    };

    println!("search result:");
    for line in search {
        println!("{}", line);
    }

    Ok(())
}

/// 搜索,区分大小写,有生命周期,参数contents的生命周期要和返回值的生命周期同样
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // result

    //使用迭代器,性能高,代码简洁
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// 搜索,不区分大小写
#[allow(dead_code)]
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //使用迭代器,性能高,代码简洁
    let query = &query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        let contents = "let's go!";
        let query = "go";
        assert_eq!(super::search(query, contents)[0], contents);
    }
}
