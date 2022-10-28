/*
 * @Author: plucky
 * @Date: 2022-10-18 00:08:47
 * @LastEditTime: 2022-10-21 16:52:17
 * @Description: 
 */
#[cfg(test)]
mod tests{
    // #![allow(unused_imports)]
    // use tiny_orm_core::prelude::*;
    use anyhow::Result;

    use crate::get_pool;
    use crate::model::User;
    
    #[tokio::test]
    async fn test()->Result<()>{
        let pool = get_pool().await?;
        // let data = User::orm_get_all(&pool).await?;
        // dbg!(data);
        
        let data=User::orm_query_with_name(&pool, "jack3").await?;
        dbg!(&data);

        let u=User::orm_get_with_pk(&pool, 1).await?;
        u.orm_update_all(&pool).await?;
        u.orm_update_name(&pool).await?;

        dbg!(u);
        
        Ok(())
    }
}
