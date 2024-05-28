/*
 * @Author: plucky
 * @Date: 2023-04-21 23:51:44
 * @LastEditTime: 2023-04-22 17:11:27
 * @Description:
 */

use std::fs::DirEntry;

use crate::prelude::*;

// Generic wrapper for a new type T, 实现了TryFrom<W<&DirEntry>> for String
impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;

    fn try_from(val: W<&DirEntry>) -> Result<String> {
        val.0
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| Error::Generic(format!("Invalid path: {:?}", val.0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name()->Result<()> {
        for entry in std::fs::read_dir("./")?.filter_map(|e|e.ok()) {
            let name = String::try_from(W(&entry))?;
            println!("{}", name);
        }
        Ok(())
    }
}