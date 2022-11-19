/*
 * @Author: plucky
 * @Date: 2022-10-18 21:23:43
 * @LastEditTime: 2022-11-19 20:08:01
 * @Description: 
 */
#![allow(dead_code)]
use rbatis::{Rbatis, rbdc::db::ExecResult};
use serde::{Serialize, Deserialize};


 #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
    #[serde(skip_serializing)] // 插入更新时忽略该字段
    id: Option<u64>,
    #[serde(rename = "name")] // 重命名该字段
    name: String,
    age: u8,
    #[serde(skip)]// 忽略该字段
    password:Option<String>,
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
    use super::*;
    use crate::*;

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


    // 使用insert_batch
    // 百万数据,2000/次,插入时间47秒
    // 用tokio::spawn并发插入,插入时间37秒
    // 1000/次,27秒,500/次,20秒,200/次,16秒,100/次,25秒
    #[tokio::test]
    async fn test_insert_batch(){
        let mut rb = init_db().await;
        let del = delete_all(&mut &rb).await.unwrap();
        println!("del: {:?}", del);
        
        let time = std::time::Instant::now();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        for i in 0..5000{
            let total = 200;
            let mut users = Vec::with_capacity(total);
            for j in 0..total{
                users.push(User{
                    id: None,
                    name: format!("rabits{}",i*total+j),
                    age: 18,
                    password: None,
                });
            }
            // let _data = User::insert_batch(&mut rb, &users, total as u64).await;
            // println!("insert_batch = {:?}", data);

            let tx = tx.clone();
            let mut rb = rb.clone();
            tokio::spawn(async move {
                let _ = User::insert_batch(&mut rb, &users, total as u64).await;
                tx.send(1).await.unwrap();
            });
        }

        drop(tx);
        let mut count = 0;
        while let Some(_) = rx.recv().await {
            count += 1;
            
        }
        println!("job count: {}", count);
        
        println!("insert_batch = {:?}", time.elapsed());

        let res = select_count(&mut rb).await;
        println!("select_all = {:?}", res);
        
    }

    // 使用sql
    // 百万数据,5000/次,7.1秒,2000/次,插入时间7.2秒,1000/次,8.0秒
    // 改变poolsize更慢
    #[tokio::test]
    async fn test_insert_batch_sql(){
        let mut rb = init_db().await;
        let del = delete_all(&mut &rb).await.unwrap();
        println!("del: {:?}", del);
        
        let time = std::time::Instant::now();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        for i in 0..500{
            let total = 2000;
            let mut sql = String::from("insert into users(name, age) values");
            for j in 0..total{
                sql.push_str(&format!("('{}', {}),",format!("rabits{}",i*total+j), 18));
            }
            sql.pop();

            // let _data = User::insert_batch(&mut rb, &users, total as u64).await;
            // println!("insert_batch = {:?}", data);

            let tx = tx.clone();
            let rb = rb.clone();
            tokio::spawn(async move {
                rb.exec(&sql, vec![]).await.unwrap();
                tx.send(1).await.unwrap();
            });
        }

        drop(tx);
        let mut count = 0;
        while let Some(_) = rx.recv().await {
            count += 1;
            
        }
        println!("job count: {}", count);
        
        println!("insert_batch = {:?}", time.elapsed());

        let res = select_count(&mut rb).await;
        println!("select_all = {:?}", res);
        
    }

}