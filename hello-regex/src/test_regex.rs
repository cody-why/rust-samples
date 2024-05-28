/*
 * @Author: plucky
 * @Date: 2023-03-28 22:20:35
 * @LastEditTime: 2023-03-29 15:58:31
 * @Description:
 */

#[allow(dead_code)]
//把所有在><之间的文本,加上~,例如"<a>文本</a><b>文本2</b>",返回"<a>~文本~</a><b>~文本2~</b>"
pub fn add_tilde(text:&str)->String{
    //使用正则表达式匹配所有在><之间的文本
    let re = regex::Regex::new(r">[^>]+<").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        // format!("{}", &caps[0].replace(">",">~").replace("<", "~<"))
        let len = caps[0].len();
        format!(">~{}~<", &caps[0][1..len-1])
    }).to_string()
    

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        println!("{}", add_tilde("<a> 文本</a>"));
        println!("{}", add_tilde("<a>文本</a><b>文本2</b>"));
        // assert_eq!(add_tilde("<a>文本</a>"), "<a>~文本~</a>");
        // assert_eq!(add_tilde("<a>文本</a><b>文本2</b>"), "<a>~文本~</a><b>~文本2~</b>");
    }
}
