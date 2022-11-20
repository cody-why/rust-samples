/*
 * @Author: plucky
 * @Date: 2022-10-18 21:23:43
 * @LastEditTime: 2022-11-20 16:42:39
 * @Description: 
 */
#![allow(dead_code)]
use rbatis::{Rbatis, rbdc::db::ExecResult};
use serde::{Serialize, Deserialize};


 #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
    #[serde(skip_serializing)] // 插入更新时忽略该字段
    pub id: Option<u64>,
    #[serde(rename = "name")] // 重命名该字段
    pub name: String,
    pub age: u8,
    #[serde(skip)]// 忽略该字段
    pub password:Option<String>,
}

crud!(User{}, "users");
impl_delete!(User{delete_by_name(name:&str) => "`where name = #{name}`"}, "users");
impl_delete!(User{delete_all() => "``"}, "users");

// test macro
#[sql("select * from users where name = ? ")]
pub async fn select_by_name(rb: &Rbatis, name: &str) -> Vec<User>{}

#[py_sql("`select * from users where name = #{user.name} `")]
pub async fn select_by_struct(rb: &Rbatis, user: &User) -> Vec<User>{}

#[sql("delete from users")]
pub async fn delete_all(rb: &Rbatis) -> ExecResult {}

#[sql("select count(*) from users")]
pub async fn select_count(rb: &Rbatis) -> u64 {}


#[cfg(test)]
mod tests{
    use crate::init_db;

    use super::*;

    //test
    #[tokio::test]
    async fn test(){
        let rb = init_db().await;
        // let _user = User{
        //     id: None,
        //     name: "rabits".to_string(),
        //     age:None,
        //     password: None,
        // };
        // let res = User::insert(&mut &rb,&_user).await;
        // println!("{:?}",res);

        // let res = User::select_all(&mut &rb).await;
        // println!("{:?}",res);
        let res = User::select_by_column(&mut &rb,"id", 1).await;
        println!("{:?}",res);

        let res = select_by_name(&mut &rb,"rabits").await;
        println!("{:?}",res);

        let res = select_by_struct(&mut &rb,&User{
            id: None,
            name: "rabits".to_string(),
            age:0,
            password: None,
        }).await;
        println!("{:?}",res);
        
    }
}