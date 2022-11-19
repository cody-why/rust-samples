/*
 * @Author: plucky
 * @Date: 2022-10-18 16:46:35
 * @LastEditTime: 2022-11-19 18:00:38
 * @Description: 
 */

use rbatis::{rbdc::datetime::FastDateTime, Rbatis};
use serde::{Deserialize, Serialize};

/// example table
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

//crud!(BizActivity {},"biz_activity");//custom table name
//impl_select!(BizActivity{select_all_by_id(table_name:&str,id:&str) => "`where id = #{id}`"}); //custom table name
crud!(BizActivity {});
impl_select!(BizActivity{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
impl_select!(BizActivity{select_by_id(id:&str) -> Option => "`where id = #{id} limit 1`"});
impl_update!(BizActivity{update_by_name(name:&str) => "`where name = #{name}`"});
impl_delete!(BizActivity {delete_by_name(name:&str) => "`where name= #{name}`"});
impl_select_page!(BizActivity{select_page() =>r#"
     if !sql.contains('count'):
       `order by create_time desc`"#});
impl_select_page!(BizActivity{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});



// test macro
#[sql("select * from biz_activity where name = ?")]
pub async fn macro_select(rb: &Rbatis, name: &str) -> Vec<BizActivity>{}
//or： pub async fn select(rb: &Rbatis,name: &str) -> rbatis::Result<BizActivity> {}



#[py_sql("`select * from biz_activity where delete_flag = 0`
                if name != '':
                  ` and name=#{name}`"
)]
async fn py_select(rb: &Rbatis, name: &str) -> Vec<BizActivity> {
  // impled!()
}
