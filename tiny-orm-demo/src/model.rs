/*
 * @Author: plucky
 * @Date: 2022-10-17 23:46:37
 * @LastEditTime: 2022-10-18 01:09:50
 * @Description: 
 */

use tiny_orm_core::prelude::*;
use tiny_orm_macro_derive::{TinyOrm, TinyOrmQuery};
use anyhow::Result;
use anyhow::Context;


#[derive(TinyOrm, TinyOrmQuery, Debug)]
#[orm_table_name = "users"]
pub struct User{
    #[orm_pk(name = "id", auto = "true")]
    pub id: Option<u32>,
    // 生成orm_query_with_name方法
    #[orm_query]
    // 生成self.orm_update_name方法
    #[orm_update]
    pub name: String,
    // #[orm_field(name = "password")]
    #[orm_ignore]
    pub password: String,
    // 忽略字段
    #[orm_ignore]
    pub ignore_field: u32,
}

impl  User {
    
}

// 实现数据获取接口
impl TinyOrmData for User {
    // 将sql返回数据映射
    fn orm_row_map(row: TinyOrmSqlRow) -> Self {
        Self { 
            id: Some(row.get::<u32, _>("id")), 
            name: row.get("name"), 
            ignore_field: 0, 
            password: "".to_string(),
        }
    }
}