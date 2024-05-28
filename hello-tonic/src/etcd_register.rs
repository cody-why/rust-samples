/*
 * @Author: plucky
 * @Date: 2023-11-06 14:25:18
 * @LastEditTime: 2023-11-10 15:24:47
 */

use async_recursion::async_recursion;
pub use etcd_client::*;
use tracing::info;

/// etcd 服务注册
pub struct EtcdRegister {
    pub etcd_client: Client,
    lease_id: i64,
}

impl EtcdRegister {
    pub fn new(client: Client) -> Self {
        Self {
            etcd_client: client,
            lease_id: 0,
        }
    }

    /// 连接etcd
    pub async fn connect(etcd_addr: impl AsRef<[&str]>, options: Option<ConnectOptions>) -> Result<Self, Error> {
        let client = Client::connect(etcd_addr, options).await?;
        info!("etcd connect success");
        Ok(Self {
            etcd_client: client,
            lease_id:0,
        })
    }

    /// 创建一个租约,租约时间为 ttl_secs 秒,每隔 keep_alive_secs 秒进行一次续约 \
    /// 如果 keep_alive_secs = 0 则不进行续约
    pub async fn lease_grant(&mut self, ttl_secs: i64, keep_alive_secs: u64)->Result<i64, Error>{
        let resp = self.etcd_client.lease_grant(ttl_secs, None).await?;
        self.lease_id = resp.id();
        if keep_alive_secs > 0 {
            self.keep_alive(keep_alive_secs).await?;
        }

        Ok(resp.id())
    }

    /// 添加一个key-value, 绑定创建的租约
    pub async fn put(&mut self, key: impl Into<Vec<u8>>, value: impl Into<Vec<u8>>) -> Result<PutResponse, Error> {
        let opt = Some(PutOptions::new().with_lease(self.lease_id));
        self.etcd_client.put(key, value, opt).await

    }
    
    /// 获取一个key-value
    pub async fn get(&mut self, key: impl Into<Vec<u8>>) -> Result<GetResponse, Error> {
        self.etcd_client.get(key, None).await
    }

    /// 删除一个key-value
    pub async fn delete(&mut self, key: impl Into<Vec<u8>>) -> Result<DeleteResponse, Error> {
        self.etcd_client.delete(key, None).await
    }
    
    async fn keep_alive(&mut self, ttl: u64 ) -> Result<(), Error> {
        // let (mut keeper, mut stream) = self.etcd_client.lease_keep_alive(self.lease_id).await?;
        let client = self.etcd_client.clone();
        let lease_id = self.lease_id;
    
        tokio::spawn(Self::keep_alive_loop(client, lease_id, ttl));

        Ok(())
    }

    // 异步的递归
    #[async_recursion]
    async fn keep_alive_loop(mut client: Client, lease_id: i64,  ttl: u64 ) -> Result<(), Error> {
        info!("lease {:?} keep alive loop", lease_id);
        let (mut keeper, mut stream) =loop {
            let  keeper = client.lease_keep_alive(lease_id).await;
            if keeper.is_err(){
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                continue;
            }
            let (keeper, stream) = keeper.unwrap();
            break (keeper, stream);
        }; 
        
        info!("lease {:?} keep alive connect", lease_id);
        loop {
            let _ = keeper.keep_alive().await;
            let resp = stream.message().await;
            match resp {
                Ok(Some(_kl)) => {
                    if _kl.ttl() <= 0 {
                        info!("lease {:?} expired", _kl.id());
                        break;
                    }
                    tracing::debug!("lease {:?} keep alive, new ttl {:?}", _kl.id(), _kl.ttl());
                }
                Ok(None) => {
                    info!("lease keep alive stream closed");
                    break;
                }
                Err(e) => {
                    info!("lease keep alive error {:?}", e);
                    Self::keep_alive_loop(client, lease_id, ttl).await.unwrap();
                    break;
                }
                
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(ttl)).await;
        }
        Ok(())
    }
    /// 释放租约
    pub async fn lease_revoke(&mut self) -> Result<(), Error> {
        self.etcd_client.lease_revoke(self.lease_id).await?;
        Ok(())
    }
   


}

impl Drop for EtcdRegister {
    fn drop(&mut self) {
        if self.lease_id > 0 {
            let lease_id = self.lease_id;
            let mut client = self.etcd_client.lease_client();
            tokio::spawn(async move {
                info!("lease revoke");
                client.revoke(lease_id).await.unwrap();
            });
            
        }
    }
    
}


#[cfg(test)]
mod tests {
    use etcd_client::*;

    use crate::etcd_register::EtcdRegister;

    #[tokio::test]
    async fn test_registry() -> Result<(), etcd_client::Error> {
        let opt = ConnectOptions::new().with_user("root", "789789");
        let mut register = EtcdRegister::connect(["127.0.0.1:2379"], Some(opt)).await.unwrap();
        register.lease_grant(30, 10).await.unwrap();

        register.put("/hello/1", "world").await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        register.put("/hello/2", "world").await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        register.delete("/hello/1").await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        register.lease_revoke().await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        Ok(())
    
    }


}