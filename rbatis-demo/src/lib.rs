/*
 * @Author: plucky
 * @Date: 2022-10-18 16:50:11
 * @LastEditTime: 2022-11-20 17:24:46
 * @Description: 
 */

//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;
use std::env;

use rbatis::Rbatis;

pub mod model;
mod tests;
mod user;
mod test_batch;

use once_cell::sync::Lazy;

pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());

/// make a rbatis
pub async fn init_db() -> Rbatis {
    // fast_log::init(fast_log::Config::new().console().level(log::LevelFilter::Info)).expect("log init fail");
    dotenv::dotenv().ok();
    let url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let rb = Rbatis::new();
    
    // ------------choose database driver------------
    rb.init(rbdc_mysql::driver::MysqlDriver {}, url).unwrap();
    // rb.get_pool().unwrap().resize(10);
    
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:newpassword@192.168.1.199:3306/hello").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    // rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db", ).unwrap();

    return rb;
}

pub async fn init_db2() {
    RB.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:newpassword@192.168.1.199:3306/hello").unwrap();
}

/// sync tables
pub async fn sync_tables() -> Rbatis {
    let rb = init_db().await;
    // // ------------sync tables------------
    // use rbatis::rbdc::db::Driver;
    // use rbatis::table_sync::{RbatisTableSync, SqliteTableSync};
    // let mut s = RbatisTableSync::new();
    // let driver = SqliteDriver {};
    // s.insert(driver.name().to_string(), Box::new(SqliteTableSync {}));
    // let raw = fast_log::LOGGER.get_level().clone();
    // fast_log::LOGGER.set_level(LevelFilter::Off);
    // s.sync(
    //     driver.name(),
    //     rb.acquire().await.unwrap(),
    //     &BizActivity {
    //         id: None,
    //         name: None,
    //         pc_link: None,
    //         h5_link: None,
    //         pc_banner_img: None,
    //         h5_banner_img: None,
    //         sort: None,
    //         status: None,
    //         remark: None,
    //         create_time: None,
    //         version: None,
    //         delete_flag: None,
    //     },
    // )
    // .await
    // .unwrap();
    // fast_log::LOGGER.set_level(raw);
    // // ------------sync tables end------------

    // ------------create tables way 2------------
    let sql = std::fs::read_to_string("table_mysql.sql").unwrap();
    let raw = fast_log::LOGGER.get_level().clone();
    fast_log::LOGGER.set_level(log::LevelFilter::Off);
    let _ = rb.exec(&sql, vec![]).await;
    fast_log::LOGGER.set_level(raw);
    // ------------create tables way 2 end------------

    return rb;
}