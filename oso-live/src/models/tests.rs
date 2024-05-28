/*
 * @Date: 2024-03-19 00:31:22
 * @LastEditTime: 2024-03-30 01:19:15
 */


#[cfg(test)]
mod tests_models {
    use rbatis::{PageRequest, RBatis};
    use std::sync::OnceLock;

    use crate::models::sys_user::SysUser;
    
    fn get_pool() -> &'static RBatis {
        static RB: OnceLock<RBatis> = OnceLock::new();
        RB.get_or_init(|| {
            tracing_subscriber::fmt()
                .with_env_filter("debug")
                .with_target(false)
                .init();
            let rb = RBatis::new();
            rb.init(rbdc_mysql::Driver {}, "mysql://root:789789@192.168.1.199:3306/oso")
                .expect("mysql init error");
            // rb.get_intercept::<LogInterceptor>().unwrap().set_level_filter(log::LevelFilter::Debug);
            rb
        })
    }

    #[tokio::test]
    async fn test_name() -> anyhow::Result<()> {
        let rb = get_pool();
        let u = SysUser::select_by_column(rb, "name", "fjxl").await?;
        println!("{:?}", u);
        if let Some(u) = u.first().cloned() {
            // SysUser::update_by_column(rb, &u, "name" ).await?;
            println!("{}", u.create_time.unwrap().format("YYYY-MM-DD hh:mm:ss"));
        }
        let u = SysUser::select_by_name(rb, "fjxlo").await;
        println!("{:?}", u);

        let u = SysUser::select_page(rb, &PageRequest::new(1, 1), 1).await;
        println!("select_page = {:?}", u);

        // 更新用户
        let user = SysUser {
            name: "fjxlo".to_string(),
            password: Some("123456a".to_string()),
            ..Default::default()
        };
        SysUser::update_by_column(rb, &user, "name").await?;

		// 插入用户
        let user = SysUser {
            name: "abcd".to_string(),
            password: Some("123456".to_string()),
            ..Default::default()
        };
        SysUser::insert(rb, &user).await?;

        let users= vec![SysUser{
            name: "abcd".to_string(),
            password: Some("123456".to_string()),
            ..Default::default()
        }];
        SysUser::insert_batch(rb, &users, 1).await?;
        
        
            
        Ok(())
    }
}

