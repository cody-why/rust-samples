/*
 * @Date: 2024-03-21 20:20:27
 * @LastEditTime: 2024-03-22 08:22:54
 */

#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
// use rbatis::rbdc::types::*;
use rust_decimal::Decimal;
use fastdate::DateTime;

/// sys_role_permission
/// 角色资源表
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SysRolePermission {
	pub id: u32,
	pub role_id: u32,
	pub permission_id: u32,
	pub create_time: Option<DateTime>,
}

impl SysRolePermission {
    pub fn table_name() -> &'static str {
        "sys_role_permission"
    }
}

rbatis::crud!(SysRolePermission{});

// ***************************************以下是自定义代码区域******************************************
/*
example: [
    {"skip_fields": ["updated_at", "created_at"], "filename": "table_name1"},
    {"contain_fields": ["updated_at", "created_at"], "filename": "table_name2"}
]
*/
// *************************************************************************************************