/*
 * @Date: 2024-03-19 09:33:11
 * @LastEditTime: 2024-03-26 12:49:38
 */

use crate::models::*;


pub struct AuthService {
}

impl AuthService {

    pub async fn is_allowed(&self, user_id: u64, action: &str) -> bool {
        let pool = get_pool();
        
        // 获取用户角色
        let role = match SysUserRole::select_by_column(pool, "user_id", user_id).await {
            Ok(r) => r,
            Err(_) => return false
        };

        // get 资源id
        let res_id = match SysPermission::get_id_by_path(pool, action).await {
            Ok(id) => id,
            Err(_) => return false
        };

        // 获取角色权限
        for r in role.iter() {
            let role_res = match SysRolePermission::select_by_column(pool, "role_id", r.role_id).await {
                Ok(r) => r,
                Err(_) => return false
            };

            if role_res.iter().any(|x| x.permission_id == res_id) {
                return true;
            }
        }

        
        false
    }
    
    pub async fn get_permissions(&self, user_id: u64) -> Vec<SysPermission> {
        let pool = get_pool();
        // 获取用户角色
        let roles = SysUserRole::select_by_column(pool,  "user_id", user_id).await.unwrap_or_default();

        let mut permissions = Vec::new();
        // 获取角色权限
        for r in roles.iter() {
            let role_res = SysRolePermission::select_by_column(pool,"role_id", r.role_id).await.unwrap_or_default();
           
            for r in role_res.iter() {
                let res = SysPermission::select_by_column(pool, "id", r.permission_id).await;
                if let Ok(r) = res {
                    permissions.extend(r);
                    
                }
                
            }
            
        }
        
        permissions
    }
}