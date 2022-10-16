/*
 * @Author: anger
 * @Date: 2022-06-23 10:04:34
 * @LastEditTime: 2022-06-23 16:56:05
 * @Description: 第一个线程例程
 */

#[cfg(test)]
mod tests {
    use std::{
        sync::{mpsc::channel, Arc, Mutex},
        thread,
        time::Duration,
    };

    #[test]
    fn test_trhead() {
        fn 一个函数(from: &str, n: u32) {
            for i in 1..=n {
                println!("{} print: {}", from, i);
                thread::sleep(Duration::from_secs(1));
            }
        }

        let h = thread::spawn(|| {
            一个函数("thread", 10);
        });

        一个函数("main", 5);

        // 使用join等待线程执行完毕
        h.join().unwrap_or(());
    }

    #[test]
    fn test_thread_move() {
        let v = vec![12, 3, 5];
        let h = thread::spawn(move || {
            println!("参数捕获:{:?}", v);
            let secs = Duration::from_secs(1);
            thread::sleep(secs);
        });

        h.join().unwrap();
    }

    ///使用channel 发送数据
    #[test]
    fn test_channel() {
        //mpsc:表示multiple producer, single consumer（多个生产者、一个消费者）
        let (sender, reciver) = channel::<&str>();

        thread::spawn(move || {
            sender.send("hello").unwrap();
        });

        let recv = &reciver.recv().unwrap();

        println!("Get: {:?}", recv);
    }

    #[test]
    fn test_memory_sharing() {
        //互斥锁保护数据
        let 互斥数据 = Mutex::new(0);
        //用计数指针包起
        let counting = Arc::new(互斥数据);

        let (tx, rx) = channel();

        //let handles = vec![];

        for _ in 0..10 {
            //需要clone一下,增加计数
            let counting = Arc::clone(&counting);
            let tx = tx.clone();
            thread::spawn(move || {
                //取得锁
                let mut mutex_guard = counting.lock().unwrap();
                //由于实现了Deref,所以可以用*指向
                *mutex_guard += 1;
                if *mutex_guard == 10 {
                    //发送一个空数据表示所有线程都已经完成
                    tx.send(()).unwrap();
                }
            });
            //handles.push(h);
        }

        // for h in handles {
        //     h.join();
        // }

        //等待线程完成
        rx.recv().unwrap();

        let n = counting.lock().unwrap();
        println!("计数结果是: {:?}", n)
    }
}
