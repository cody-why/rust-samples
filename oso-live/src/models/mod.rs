/*
 * @Date: 2024-03-18 16:31:40
 * @LastEditTime: 2024-03-22 08:33:28
 */

use std::sync::OnceLock;
use rbatis::RBatis;

pub mod sys_role;
pub mod sys_role_permission;
pub mod sys_user;
pub mod sys_user_role;
pub mod sys_permission;

pub use sys_role::*;
pub use sys_role_permission::*;
pub use sys_user::*;
pub use sys_user_role::*;
pub use sys_permission::*;


mod tests;

static RB: OnceLock<RBatis> = OnceLock::new();

pub fn get_pool() -> &'static RBatis {
    RB.get_or_init(|| {
        tracing_subscriber::fmt().with_env_filter("debug").init();
        let rb = RBatis::new();
        rb.init(rbdc_mysql::Driver{}, "mysql://root:789789@192.168.1.199:3306/oso").expect("mysql init error");
        rb
    })
    
}

// pub async fn init_db_pool(url: impl AsRef<str>, max_conns: u32)->Result<(), Box<dyn std::error::Error>> {
//     let rb = RBatis::new();
//     rb.init(rbdc_mysql::Driver{}, url.as_ref()).expect("mysql init error");
//     let pool = rb.get_pool()?;
//     pool.set_timeout(Some(Duration::from_secs(60))).await;
//     pool.set_max_open_conns(max_conns as u64).await;
//     // 获取一个连接,检查是否成功
//     pool.get_timeout(Duration::from_secs(10)).await?;
//     assert!(RB.set(rb).is_ok());

//     Ok(())
// }pub mod sys_permission;

