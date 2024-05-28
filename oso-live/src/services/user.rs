/*
 * @Date: 2024-03-19 09:59:50
 * @LastEditTime: 2024-03-22 08:34:43
 */

use crate::models::{get_pool,SysUser};

pub struct UserService {}

impl UserService {
    
    pub async fn get_user(name: impl AsRef<str>) -> Option<SysUser> {
        let pool = get_pool();
        
        SysUser::select_by_name(pool, name.as_ref()).await.unwrap_or_default()
    }

    
}