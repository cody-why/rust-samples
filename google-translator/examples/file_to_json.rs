/*
 * @Author: anger
 * @Date: 2023-11-22 10:38:28
 * @LastEditTime: 2023-11-30 16:09:45
 */



fn main(){
    
}

#[cfg(test)]
mod tests {
    use std::{fs::{File, self}, collections::BTreeMap};
    use indexmap::IndexMap;

    #[test]
    fn tes_file_to_json() {
        // 1. 遍历目录,把文件名作为json中的key
        let file_name = "/Users/anger/Downloads/很杂/翻译/translator/lang.json";
        let dir = "/Users/anger/Downloads/很杂/fifa_zh_cn";
        let mut btmap = BTreeMap::new();
        btmap.insert("ZH", "");

        let mut new_map = IndexMap::new();

        // 遍历目录
        let mut entries = fs::read_dir(dir).unwrap().map(|x| x.unwrap().path()).collect::<Vec<_>>();
        // 按名称排序
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        
        for path in entries{
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if !file_name.ends_with(".png") && !file_name.ends_with(".jpg") {
                    continue;
                }
                let new_file_name = file_name.replace(".png", "").replace(".jpg", "");
                println!("{}", new_file_name);
                new_map.insert(new_file_name, btmap.clone());
            }
        }
        
        let file = File::create(file_name.to_owned()+".json").unwrap();
        serde_json::to_writer_pretty(file, &new_map).unwrap();
        
    }

    #[test]
    fn tes_file_to_json_change() {
        // 1. 读取json文件,原来的key是数字,现在需要把key改为文件名
        // 2. 遍历目录,把文件名替换json中的key
        let file_name = "/Users/anger/Downloads/很杂/翻译/translator/完成的/3kings-ar-pt-fr.t.json";
        let dir = "/Users/anger/Downloads/很杂/CN AR PT FR";
        let file = File::open(file_name).unwrap();
        let index_map: IndexMap<String, BTreeMap<String, String>> = serde_json::from_reader(file).unwrap();
    
        let mut map_iter = index_map.iter();
        let mut new_map = IndexMap::new();

        // 遍历目录
        let mut entries = fs::read_dir(dir).unwrap().map(|x| x.unwrap().path()).collect::<Vec<_>>();
        // 按名称排序
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        
        for path in entries{
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if !file_name.ends_with(".png") && !file_name.ends_with(".jpg") {
                    continue;
                }
                let new_file_name = file_name.replace(".png", "").replace(".jpg", "");
                println!("{}", new_file_name);
                let index = map_iter.next().unwrap();
                new_map.insert(new_file_name, index.1);
            }
        }
        
        let file = File::create(file_name.to_owned()+".json").unwrap();
        serde_json::to_writer_pretty(file, &new_map).unwrap();
    
    }
}