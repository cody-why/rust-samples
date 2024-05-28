/*
 * @Author: plucky
 * @Date: 2023-11-06 10:30:41
 * @LastEditTime: 2023-11-10 16:08:09
 */

use etcd_client::*;
use hello_tonic::{EtcdDiscovery, EtcdRegister};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 服务发现
    let opt = ConnectOptions::new().with_user("tonic_user", "789789");
    let mut discover = EtcdDiscovery::connect(["127.0.0.1:2379"], Some(opt.to_owned())).await?;
    discover.service_discover("/hello").await?;
    
    tokio::spawn(async move {
        // 服务注册
        let mut registry = EtcdRegister::connect(["127.0.0.1:2379"], Some(opt)).await.unwrap();
        registry.lease_grant(30, 10).await.unwrap();
        
        registry.put("/hello/1", "http://world").await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        registry.put("/hello/2", "http://world").await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        registry.delete("/hello/1").await.unwrap();

    });

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let s = discover.get_service("/hello/1");
    println!("service: {:?}", s);
    let s = discover.get_all_channel();
    println!("chanel: {:?}", s);

    for _ in 0..5 {
        println!("services: {:?}", discover.get_service_map().read().unwrap());
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
   
    // tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use etcd_client::*;

    #[tokio::test]
    async fn get_all_keys() {
        let opt = ConnectOptions::new().with_user("root", "789789");
        let mut client = Client::connect(["127.0.0.1:2379"], Some(opt)).await.unwrap();
        // get all keys
        let resp = client.get("", Some(GetOptions::new().with_all_keys())).await.unwrap();
        for kv in resp.kvs() {
            println!("key: {}, value: {}", kv.key_str().unwrap(), kv.value_str().unwrap());
        }
        
    }


    #[tokio::test]
    async fn init_role_user() {
        use etcd_client::*;
        let opt = ConnectOptions::new().with_user("root", "r789789");
        let mut client = Client::connect(["127.0.0.1:2379"], Some(opt)).await.unwrap();
      
        client.user_list().await.iter().for_each(|u| println!("user list: {:?}", u));

        // client.user_change_password("root", "r789789").await.unwrap();
        // let opt = Some(RoleRevokePermissionOptions::new().with_all_keys());
        // client.role_revoke_permission("api_tonic_role", "/hello",opt).await.unwrap();

        add_user(&mut client,"root", "root", "r789789",None).await;

        let opt = Some(Permission::read_write("/hello").with_prefix());
        add_user(&mut client, "api_tonic_role", "tonic_user", "789789", opt).await;

        client.auth_enable().await.unwrap();
        
    }
        
    
    async fn add_user(client: &mut Client, role: &str, user: &str, password: &str, permission: Option<Permission>) {
        if let Ok(rsp) =  client.role_get(role).await{
            if let Some(p) = permission {
                if rsp.permissions().iter().find(|r| *r == &p).is_none(){
                    client.role_grant_permission(role, p).await.unwrap();
                }
            }
        }else {
            client.role_add(role).await.unwrap();
            if let Some(p) = permission {
                client.role_grant_permission(role, p).await.unwrap();
            }
        }
        if let Ok(rsp) = client.user_get(user).await{
            println!("user: {:?}", rsp);
            if rsp.roles().iter().find(|r| *r == role).is_none(){
                client.user_grant_role(user, role).await.unwrap();
            }
        }else {
            client.user_add(user, password, None).await.unwrap();
            client.user_grant_role(user, role).await.unwrap();
        }
        
    }
   
}
