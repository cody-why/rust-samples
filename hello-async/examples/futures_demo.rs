/*** 
 * @Author: plucky
 * @Date: 2022-06-26 11:10:41
 * @LastEditTime: 2022-06-26 12:56:00
 * @Description: futures
 */

// futures是一个异步的库，它提供了异步的基础设施，包括异步的任务，异步的执行器，异步的通道等等
use futures::executor;
use futures::executor::ThreadPool;

use futures::channel::mpsc;
use futures::StreamExt;

fn main() {

    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<i32>();

    //异步的块，他现在还不会执行
    let fut_values = async {
        // 又是一个异步的代码块，有父异步执行器执行
        let fut_tx_result = async move {
            (0..100).for_each(|v| {
                tx.unbounded_send(v).expect("Failed to send");
            })
        };

        // 使用线程池产生线程
        pool.spawn_ok(fut_tx_result);

        //接收线程结果,是异步的代码块里的
        let fut_values = rx
            .map(|v| v * 2)
            .collect();

        // 等待完成
        fut_values.await
    };

    //实际执行上面的future，会调用Future::poll和
    //随后链接适当的 Future::poll 和需要执行器的方法

    let values: Vec<i32> = executor::block_on(fut_values);

    println!("Values={:?}", values);

    let f =  hello_world();
    println!("还没有执行hello world");
    //这里才执行
    executor::block_on(f);

}


async fn hello_world(){
    println!("Hello world!")
}

