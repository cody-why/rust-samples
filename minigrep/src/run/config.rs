use std::env::Args;

// 重构一下
#[derive(Debug)]
pub struct Config {
    /// 搜索文本
    pub query: String,
    /// 文件名
    pub filename: String,
    /// 区分大小写
    pub casesensitive: bool,
}

impl Config {
    /// 从运行参数解析配置,成功返回config,失败返回错误信息
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        //let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("运行参数不够2个,示例: 要搜索的内容 搜索的文件名");
        }
        //使用闭包函数,他能获得外部参数的所有权,所有性能好,避免clone
        //跳过1个
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少搜索的内容参数"),
        };

        //let query = args.next().unwrap_or("".into());
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少搜索的文件名参数"),
        };

        // 环境变量
        let case = option_env!("CASE").unwrap_or("0");

        Ok(Self {
            query,
            filename,
            casesensitive: case.eq("1"),
        })
    }
}
