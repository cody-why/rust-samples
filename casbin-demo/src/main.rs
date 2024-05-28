/***
 * @Author: plucky
 * @Date: 2022-07-04 13:45:05
 * @LastEditTime: 2022-07-06 16:24:47
 * @Description: casbin的测试,在线生成规则:
 * https://casbin.org/zh-CN/editor
 */

use std::env;

use sqlx_adapter::casbin::prelude::*;
use sqlx_adapter::casbin::Result;
use sqlx_adapter::{
    casbin::{DefaultModel, Enforcer},
    SqlxAdapter,
};

const MODEL_AUTH: &str = include_str!("auth_model.conf");

#[tokio::main]
async fn main() -> Result<()> {
    // 解析.env文件
    dotenv::dotenv().ok();

    let m = DefaultModel::from_str(MODEL_AUTH).await?;
    let a = SqlxAdapter::new(env::var("DATABASE_URL").unwrap(), 5).await?;

    println!("get_model:{:?}", m.get_model().keys()); //["r", "e", "m", "p", "g"]

    let mut e = Enforcer::new(m, a).await?;
    // 自定义函数
    e.add_function("isAdmin", |a, b|{
        println!("isAdmin: {:?}, {:?}", a, b);
        if a == "admin" {
            return true
        }
        false
    });
    e.load_policy().await?;

    // e.remove_policy(to_owned(vec!["*", "/login", "*"])).await.unwrap_or_default();
    // 添加角色admin,member
    e.add_policy(to_owned(vec!["admin", "*", "*"])).await.unwrap_or_default();
    e.add_policy(to_owned(vec!["member", "/member", "*"])).await.unwrap_or_default();
    
    println!("get_all_policy: {:?}", e.get_all_policy());
    //[["p", "p", "*", "/login", "*"], ["p", "p", "admin", "*", "*"], ["p", "p", "member", "/member", "*"]
    println!("get_all_roles: {:?}", e.get_all_roles());
    // ["member"]
    println!("get_all_objects: {:?}", e.get_all_objects());
    // ["/login", "*", "/member"]
    println!("get_all_subjects: {:?}", e.get_all_subjects());
    // ["*", "admin", "member"]
    
    // 添加g2,domain是租户
    // e.add_named_grouping_policy("g", to_owned(vec!["alice", "admin"])).await.unwrap_or_default();
    let added = e.add_role_for_user("alice", "admin", None).await.unwrap_or_else(|e| {
        println!("add_role_for_user error:{:?}", e);
        false
    });
    println!("Is added? {:?}", added);
    let added = e.add_role_for_user("alice", "member", None).await.unwrap_or_else(|e| {
        println!("add_role_for_user error:{:?}", e);
        false
    });
    println!("Is added? {:?}", added);
    // 删除用户的admin角色
    //  e.delete_role_for_user("alice", "admin", None).await?;

    //验证用户是否有admin角色
    let ok = e.has_role_for_user("alice", "admin", None);
    println!("Has role admin? {:?}", ok);

    // 获取alice的角色
    let roles = e.get_roles_for_user("alice", None);
    println!("Roles for alice: {:?}", roles); // ["admin", "member"]

    // 验证alice是否有权限访问/member
    let ok = e.enforce(("alice", "/member", "GET"));
    println!("Match alice for member is {:?}", ok);

    // 验证alice是否有权限访问/admin
    let ok = e.enforce(("alice", "/admin", "GET"));
    println!("Match alice for admin is {:?}", ok);


    // 验证anyone是否有权限访问/member
    let ok = e.enforce(("abcd", "/member", "POST"));
    println!("Match anyone for member is {:?}", ok); // false

    // 获取角色member的权限
    let menu = e.get_permissions_for_user("member", None);
    println!("Menu for member: {:?}", menu);
    //[["member", "/member", "*"]]

    Ok(())
}

pub fn to_owned(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(|x| x.to_owned()).collect()
}

#[cfg(test)]
mod tests {
    

    // 匹配方式代码
    #[test]
    fn test_key_match() {
        use sqlx_adapter::casbin::function_map::*;
        assert!(key_match("/foo/bar", "/foo/*"));
        assert!(key_match("/bar", "/ba*"));

        assert!(key_match2("/foo/baz", "/foo/:bar"));
        assert!(key_match3("/foo/baz", "/foo/{bar}"));

        // 以上都匹配"/foo/bar", "/foo/*"

        assert!(regex_match("foobar", "^foo*"));
        assert!(!regex_match("barfoo", "^foo*"));

        // assert!(!regex_match("GET", "*"));//错误
        assert!(regex_match("GET", "GET|POST"));
    }

    // cached 缓存,参数相同只会执行一次
    #[cached::proc_macro::cached(time = 3)]
    fn cached_slow_fn(n: u32) -> String {
        use std::thread::sleep;
        use std::time::Duration;
        println!("cached_slow_fn:{n}");
        sleep(Duration::new(1, 0));
        format!("{n}")
    }

    // once 不管参数是否相同,只会执行一次
    #[cached::proc_macro::once(time = 2)]
    fn once_slow_fn(n: u32) -> String {
        use std::time::Duration;
        println!("once_slow_fn:{n}");
        std::thread::sleep(Duration::new(1, 0));
        format!("{n}")
    }

    #[test]
    fn test_once() {
        use std::time::Duration;
        // 不会执行,不管参数是什么,直接返回1
        once_slow_fn(1);
        let a = once_slow_fn(1);
        let b = once_slow_fn(2);
        println!("a:{a}, b:{b}");
        std::thread::sleep(Duration::new(2, 0));
        let b = once_slow_fn(2);
        println!("b:{b}");

        cached_slow_fn(1);
        // 不会执行,直接返回1
        cached_slow_fn(1);
        let b = cached_slow_fn(2);
        println!("a:{a}, b:{b}");

        std::thread::sleep(Duration::new(3, 0));
        // 3秒过期后,再次执行
        let a = cached_slow_fn(1);
        println!("a:{a}");
    }
}