/*
 * @Date: 2024-03-21 20:20:27
 * @LastEditTime: 2024-03-21 20:30:05
 */

#![allow(unused_imports)]
use rbatis::py_sql;
use serde::{Deserialize, Serialize};
// use rbatis::rbdc::types::*;
use rust_decimal::Decimal;
use fastdate::DateTime;

/// sys_permission
/// 资源权限表
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SysPermission {
	pub id: u32,
	pub name: Option<String>,
	pub parent_id: Option<i32>,
	pub permission: Option<String>,
	pub path: Option<String>,
	/// Create Time   
	pub create_time: Option<DateTime>,
}

impl SysPermission {
    pub fn table_name() -> &'static str {
        "sys_permission"
    }
}

rbatis::crud!(SysPermission{});

// ***************************************以下是自定义代码区域******************************************
impl SysPermission {
	/// get resource id by url
	#[py_sql("select id from sys_permission where path = #{path}")]
	pub async fn get_id_by_path(rb: &rbatis::RBatis, path: &str) -> u32{
		impled!()
	}
	
}
// *************************************************************************************************