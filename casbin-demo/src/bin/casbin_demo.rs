/*** 
 * @Author: plucky
 * @Date: 2022-07-04 13:45:05
 * @LastEditTime: 2022-07-06 16:24:47
 * @Description: casbin的测试,在线生成规则:
 * https://casbin.org/zh-CN/editor
 */

use std::collections::HashMap;

use sqlx_adapter::casbin::prelude::*;
use sqlx_adapter::casbin::Result; 
use sqlx_adapter::{casbin::{DefaultModel,Enforcer}, SqlxAdapter};

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}


#[tokio::main]
async fn  main()->Result<()> {
    dotenv().ok();
     // First access to `HASHMAP` initializes it
     println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    let m = DefaultModel::from_str(MODEL_AUTH).await?;
    let a = SqlxAdapter::new("mysql://root:newpassword@192.168.1.199:3306/casbin", 5).await?;

    println!("get_model:{:?}", m.get_model().keys());

    let mut e = Enforcer::new(m, a).await?;
    e.load_policy().await?;

    e.add_policy(to_owned(vec!["admin", "*", "*"])).await.unwrap_or_else(|e|{
        println!("add_policy error:{:?}", e);
        false
    });
    
    e.add_policy(to_owned(vec!["*", "/login", "*"])).await.unwrap_or(false);
    e.add_policy(to_owned(vec!["member", "/member","*"])).await.unwrap_or(false);
    
    println!("get_all_policy{:?}",e.get_all_policy());
    println!("get_all_roles{:?}",e.get_all_roles());
    println!("get_all_objects{:?}",e.get_all_objects());
    println!("get_all_subjects{:?}",e.get_all_subjects());


    let added = e.add_role_for_user("alice", "admin", None).await.unwrap_or_else(|e|{
        println!("add_role_for_user error:{:?}", e);
        false
    });
    println!("Is added? {:?}", added);
    let added = e.add_role_for_user("alice", "member", None).await.unwrap_or_else(|e|{
        println!("add_role_for_user error:{:?}", e);
        false
    });
    println!("Is added? {:?}", added);

    e.delete_role_for_user("alice", "admin", None).await?;
    //验证是否有admin角色
    let ok = e.has_role_for_user("alice", "admin", None);
    println!("Has role admin? {:?}", ok);
    

    let roles = e.get_roles_for_user("alice", None);
    println!("Roles for alice: {:?}", roles);

    // 测试alice是否有权限访问/member
    let ok = e.enforce(("alice", "/member", "GET")).unwrap_or(false);
    println!("Match alice for member is {:?}", ok);

    // 测试alice是否有权限访问/admin
    let ok = e.enforce(("alice", "/admin", "GET"));
    println!("Match alice for admin is {:?}", ok);

    // 测试anyone是否有权限访问/login
    let ok = e.enforce(("abcd", "/login", "*"));
    println!("Match anyone is {:?}", ok);
 
    // 测试anyone是否有权限访问/member
    let ok = e.enforce(("abcd", "/member", "*"));
    println!("Match anyone for member is {:?}", ok);
    
    

    Ok(())
}

pub fn to_owned(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(|x| x.to_owned()).collect()
}

const MODEL_AUTH: &str = r#"
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = (g(r.sub, p.sub) || p.sub=='*') && keyMatch(r.obj, p.obj) && (r.act==p.act || p.act=='*')
"#;

