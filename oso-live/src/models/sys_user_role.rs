#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
// use rbatis::rbdc::types::*;
use rust_decimal::Decimal;
use fastdate::DateTime;

/// sys_user_role
/// 用户角色表
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SysUserRole {
	pub id: u32,
	/// 用户ID   
	pub user_id: u32,
	/// 角色ID   
	pub role_id: u32,
	pub create_time: Option<DateTime>,
	pub is_deleted: Option<i8>,
}

impl SysUserRole {
    pub fn table_name() -> &'static str {
        "sys_user_role"
    }
}

rbatis::crud!(SysUserRole{});

// ***************************************以下是自定义代码区域******************************************
/*
example: [
    {"skip_fields": ["updated_at", "created_at"], "filename": "table_name1"},
    {"contain_fields": ["updated_at", "created_at"], "filename": "table_name2"}
]
*/
// *************************************************************************************************