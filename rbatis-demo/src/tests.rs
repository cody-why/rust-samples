/*
 * @Author: plucky
 * @Date: 2022-10-18 20:08:56
 * @LastEditTime: 2022-10-19 15:54:15
 * @Description: 
 */

 #[cfg(test)]
mod tests{
    use crate::{init_db,model::{BizActivity, self}};

    #[tokio::test]
    async fn test_raw_sql(){
        let rb = init_db().await;
        let table: Option<BizActivity> = rb
            .fetch_decode("select * from biz_activity limit ?", vec![rbs::to_value!(1)])
            .await
            .unwrap();
        let count: u64 = rb
            .fetch_decode("select count(1) as count from biz_activity", vec![])
            .await
            .unwrap();
        dbg!( table);
        dbg!( count);
    }

   
    #[allow(unused)]
    #[tokio::test]
    async fn test_macro(){
        let rb = init_db().await;
        let table = model::macro_select(&rb, "活动1").await;
        // dbg!( table);
        

        // let table = model::py_select(&rb, "活动1").await;
        dbg!(">>>>>>", table);
    }
}