/*
 * @Date: 2024-03-21 20:20:27
 * @LastEditTime: 2024-03-22 08:24:06
 */

#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
// use rbatis::rbdc::types::*;
use rust_decimal::Decimal;
use fastdate::DateTime;

/// sys_user
/// 用户表
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SysUser {
	pub id: u64,
	pub name: String,
	pub password: Option<String>,
	pub email: Option<String>,
	pub amount: Option<Decimal>,
	pub create_time: Option<DateTime>,
	pub update_time: Option<DateTime>,
	pub is_deleted: Option<i8>,
}

impl SysUser {
    pub fn table_name() -> &'static str {
        "sys_user"
    }
}

rbatis::crud!(SysUser{}, SysUser::table_name());

// ***************************************以下是自定义代码区域******************************************
rbatis::impl_select!(SysUser{select_by_name(name: &str) -> Option => "`where name = #{name} limit 1`"});
rbatis::impl_select_page!(SysUser{select_page(id: u64) => "`where id != #{id}`"});

// *************************************************************************************************