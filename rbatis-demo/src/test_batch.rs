/*
 * @Author: plucky
 * @Date: 2022-11-20 10:47:40
 * @LastEditTime: 2022-11-20 17:09:09
 * @Description: 
 */


#[cfg(test)]
mod tests{

    use crate::{*, user::*};

    // 使用insert_batch
    // 百万数据,2000/次,插入时间47秒
    // 用tokio::spawn并发插入,插入时间37秒
    // 1000/次,27秒,500/次,20秒,200/次,16秒,100/次,25秒
    #[tokio::test]
    async fn test_insert_batch(){
        let mut rb = init_db().await;
        let del = delete_all(&mut &rb).await.unwrap();
        println!("del: {:?}", del);
        
        let time = std::time::Instant::now();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        for i in 0..5000{
            let total = 200;
            
            let tx = tx.clone();
            let mut rb = rb.clone();
            tokio::spawn(async move {
                let mut users = Vec::with_capacity(total);
                for j in 0..total{
                    users.push(User{
                        id: None,
                        name: format!("rabits{}",i*total+j),
                        age: 18,
                        password: None,
                    });
                }
                let _ = User::insert_batch(&mut rb, &users, total as u64).await;
                tx.send(1).await.unwrap();
            });
        }

        drop(tx);
        let mut count = 0;
        while let Some(_) = rx.recv().await {
            count += 1;
            
        }
        println!("job count: {}", count);
        
        println!("insert_batch = {:?}", time.elapsed());

        let res = select_count(&mut rb).await;
        println!("select_all = {:?}", res);
        
    }

    // 使用sql
    // 百万数据,5000/次,7.1秒,2000/次,插入时间7.3秒,1000/次,8.0秒
    // 改变poolsize更慢
    #[tokio::test]
    async fn test_insert_batch_sql(){
        let mut rb = init_db().await;
        let del = user::delete_all(&mut &rb).await.unwrap();
        println!("del: {:?}", del);
        
        let time = std::time::Instant::now();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        for i in 0..500{
            let total = 2000;
            
            let tx = tx.clone();
            let rb = rb.clone();
            tokio::spawn(async move {
                let mut sql = String::from("insert into users(name, age) values");
                for j in 0..total{
                    sql.push_str(&format!("('{}', {}),",format!("rabits{}",i*total+j), 18));
                }
                sql.pop();
                // fetch 比exec 慢
                // rb.fetch(&sql, vec![]).await.unwrap();
                rb.exec(&sql, vec![]).await.unwrap();
                tx.send(1).await.unwrap();
            });
        }

        drop(tx);
        let mut count = 0;
        while let Some(_) = rx.recv().await {
            count += 1;
            
        }
        println!("job count: {}", count);
        
        println!("insert_batch = {:?}", time.elapsed());

        let res = user::select_count(&mut rb).await;
        println!("select_all = {:?}", res);
        
    }

}