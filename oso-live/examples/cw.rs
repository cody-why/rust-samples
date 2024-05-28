/*
 * @Date: 2024-03-17 23:48:28
 * @LastEditTime: 2024-03-18 11:49:15
 */
#![allow(dead_code)]
use oso::{Oso, PolarClass};
use std::collections::HashMap;

#[derive(Clone, PolarClass)]
struct User {
    name: String,
    #[polar(attribute)]
    roles: Vec<String>,
}

#[derive(Clone, PolarClass)]
struct Repo {
    name: String,
    #[polar(attribute)]
    permissions: HashMap<String, Vec<String>>,
}

fn main() -> oso::Result<()> {
    let mut oso = Oso::new();

    // 注册 Rust 结构体
    oso.register_class(User::get_polar_class_builder().build())?;
    oso.register_class(Repo::get_polar_class())?;

    // 定义策略
    oso.load_str(
        r#"
        allow(user: User, action: String, repo: Repo) if
            role in user.roles and
            action in repo.permissions.(role);
        "#,
    )?;

    // 创建用户和资源
    let user = User {
        name: "Alice".to_string(),
        roles: vec!["财务总监".to_string()],
    };

    let mut permissions = HashMap::new();
    permissions.insert(
        "财务总监".to_string(),
        vec!["财务记账".to_string(), "查看报表".to_string(), "财务审核".to_string()],
    );
    permissions.insert("财务".to_string(), vec!["财务记账".to_string(), "查看报表".to_string()]);
    permissions.insert(
        "老板".to_string(),
        vec!["财务记账".to_string(), "查看报表".to_string(), "财务审核".to_string(),"财务审批".to_string()],
    );

    let resource = Repo {
        name: "财务系统".to_string(),
        permissions,
    };

    // 检查权限
    assert!(oso.is_allowed(user.clone(), "财务记账".to_string(), resource.clone())?);
    assert!(oso.is_allowed(user.clone(), "财务审核".to_string(), resource.clone())?);
    assert!(!oso.is_allowed(user, "财务审批".to_string(), resource)?);

    Ok(())
}
