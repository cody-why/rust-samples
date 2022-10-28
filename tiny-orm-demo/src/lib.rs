/*
 * @Author: plucky
 * @Date: 2022-10-18 00:05:46
 * @LastEditTime: 2022-10-18 00:42:56
 * @Description: 
 */
pub mod model;

mod tests;

use sqlx::mysql::MySqlPoolOptions;
use tiny_orm_core::prelude::*;
use sqlx::Result;

pub async fn get_pool() -> Result<TinyOrmDbPool> {
    let ip = "192.168.1.199";
    let port = 3306;
    let user_name = "root";
    let password = "newpassword";
    let db_name = "hello";
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&format!(
            "mysql://{}:{}@{}:{}/{}",
            user_name, password, ip, port, db_name
        ))
        .await?;
    Ok(pool)
}