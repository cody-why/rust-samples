#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
// use rbatis::rbdc::types::*;
use rust_decimal::Decimal;
use fastdate::DateTime;

/// sys_role
/// 角色表
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SysRole {
	pub id: u32,
	pub name: String,
	pub parent_id: Option<u32>,
	pub create_time: Option<DateTime>,
	pub description: Option<String>,
	pub is_deleted: Option<i8>,
}

impl SysRole {
    pub fn table_name() -> &'static str {
        "sys_role"
    }
}

rbatis::crud!(SysRole{});

// ***************************************以下是自定义代码区域******************************************
/*
example: [
    {"skip_fields": ["updated_at", "created_at"], "filename": "table_name1"},
    {"contain_fields": ["updated_at", "created_at"], "filename": "table_name2"}
]
*/
// *************************************************************************************************