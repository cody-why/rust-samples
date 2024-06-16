/*** 
 * casbin在线生成规则:
 * https://casbin.org/zh/editor
 */


use std::env;
use sqlx_adapter::casbin::prelude::*;
use sqlx_adapter::casbin::Result; 
use sqlx_adapter::{casbin::{DefaultModel,Enforcer}, SqlxAdapter};
use casbin_demo::*;


#[tokio::main]
async fn  main()->Result<()> {
    dotenv::dotenv().ok();
    let model_auth = include_str!("../conf/rbac_model.conf");
    let m = DefaultModel::from_str(model_auth).await?;
    let a = SqlxAdapter::new(env::var("DATABASE_URL").unwrap(),8).await?;
    
    // 获取模型中的所有key ["r", "e", "m", "p", "g"]
    println!("get_model:{:?}", m.get_model().keys());

    let mut e = Enforcer::new(m, a).await?;
    e.load_policy().await?;

    // 删除member角色的所有权限
    // e.delete_permissions_for_user("member").await?;

    // 添加角色admin,member,anyone
    e.add_policy(vec_string!["admin", "*", "*"]).await.unwrap_or_default();
    e.add_policy(vec_string!["*", "/login", "*"]).await.unwrap_or_default();
    e.add_policy(vec_string!["member", "/member","*"]).await.unwrap_or_default();
    // 功能和add_policy一样
    e.add_permission_for_user("member", vec_string!["/book", "*"]).await.unwrap_or_default();
    
    

    let added = e.add_role_for_user("jack", "admin", None).await.unwrap_or_else(|e|{
        println!("add_role_for_user error:{:?}", e);
        false
    });
    println!("Is added? {:?}", added);
    let added = e.add_role_for_user("alice", "member", None).await.unwrap_or_else(|e|{
        println!("add_role_for_user error:{:?}", e);
        false
    });
    println!("Is added? {:?}", added);
    // 删除用户的admin角色
    // e.delete_role_for_user("alice", "admin", None).await?;

    // 获取所有策略(表中所有数据)
    println!("get_all_policy: {:?}",e.get_all_policy());
    //[["p", "p", "*", "/login", "*"], ["p", "p", "admin", "*", "*"], ["p", "p", "member", "/member", "*"]
    
    // 获取所有角色 ["*", "admin", "member"]
    println!("get_all_subjects: {:?}",e.get_all_subjects());
    // 获取所有权限菜单 ["/login", "*", "/member"]
    println!("get_all_objects: {:?}",e.get_all_objects());
    // 获取所有角色(只输出设置了用户的角色) ["member"]
    println!("get_all_roles: {:?}",e.get_all_roles());

    // 获取alice的角色 ["member"]
    let roles = e.get_roles_for_user("alice", None);
    println!("Roles for alice: {:?}", roles); 

    // 获取member角色的权限菜单 [["member", "/book", "*"], ["member", "/member", "*"]]
    let menu = e.get_permissions_for_user("member",None);
    println!("Menu for member: {:?}", menu); 
    
    // e.delete_permission_for_user("member", vec_string!["/book", " "]).await.unwrap_or_default();
    // let menu = e.get_permissions_for_user("member",None);
    // println!("Menu for member: {:?}", menu); 

    //验证用户是否有admin角色 Ok(false)
    let ok = e.has_role_for_user("alice", "admin", None);
    println!("Has role admin? {:?}", ok);

    // 验证alice是否有权限访问/member Ok(true)
    let ok = e.enforce(("alice", "/member", "GET"));
    println!("Match alice for member is {:?}", ok); 

    // 验证alice是否有权限访问/admin Ok(false)
    let ok = e.enforce(("alice", "/admin", "GET"));
    println!("Match alice for admin is {:?}", ok);
    
    // 验证anyone是否有权限访问/login Ok(true)
    let ok = e.enforce(("abcd", "/login", "*"));
    println!("Match anyone is {:?}", ok); 
 
    // 验证anyone是否有权限访问/member Ok(false)
    let ok = e.enforce(("abcd", "/member", "GET"));
    assert!(!ok.unwrap());
    
    
   
    
    Ok(())
}



