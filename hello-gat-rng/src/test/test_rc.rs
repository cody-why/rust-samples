/*
 * @Author: plucky
 * @Date: 2022-12-04 21:18:42
 * @LastEditTime: 2022-12-04 22:09:39
 * @Description: https://course.rs/advance/smart-pointer/rc-arc.html
 */

#[cfg(test)]
mod tests {

    #[test]
    fn test_name() {
        use std::rc::Rc;
        let a = Rc::new(String::from("test ref counting"));
        // Rc::strong_count 可以获取当前引用计数的值
        println!("count after creating a = {}", Rc::strong_count(&a));//1
        let b =  Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&b));//2
        {
            let c =  Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&c));//3
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));//2
        // 不要被 clone 字样所迷惑，以为所有的 clone 都是深拷贝。
        // 这里的 clone 仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据，因此 a 和 b 是共享了底层的字符串 s，这种复制效率是非常高的。
    }

    #[test]
    fn test_owner() {
        use std::rc::Rc;
        struct Owner {
            name: String,
        }
        // 多个工具属于同一个主人
        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
        }
        
        // 创建一个基于引用计数的 `Owner`.
        let gadget_owner: Rc<Owner> = Rc::new(Owner {
            name: "Gadget Man".to_string(),
        });

        // 创建两个不同的工具，它们属于同一个主人
        let gadget1 = Gadget {
            id: 1,
            owner: Rc::clone(&gadget_owner),
        };
        let gadget2 = Gadget {
            id: 2,
            owner: Rc::clone(&gadget_owner),
        };

        // 释放掉第一个 `Rc<Owner>`
        drop(gadget_owner);

        // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
        // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
        // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
        // 引用指向底层的 owner 数据，引用计数尚未清零
        // 因此 owner 数据依然可以被使用
        println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
        println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

        // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
        // 数据也被清理释放
    }
    

    #[test]
    fn test_arc(){
        use std::sync::Arc;
        use std::thread;
        // Arc:Atomic Rc,它能保证我们的数据能够安全的在线程间共享
        let s = Arc::new(String::from("多线程漫游者"));
        for _ in 0..10 {
            let s = Arc::clone(&s);
            let _handle = thread::spawn(move || {
               println!("{}", s)
            });
        }
    }

    #[test]
    fn test_cell(){
        use std::cell::Cell;
        use std::cell::RefCell;
        // Cell<T> 和 RefCell<T> 都是用来包装可变类型的，它们的区别在于：
        // Cell<T> 只能包装实现了 Copy trait 的类型，而 RefCell<T> 可以包装任意类型。

        let c = Cell::new(5);
        let _five = c.get(); // 同时获取可变和不可变引用Rust是不允许的,但是Cell可以
        c.set(6); // 等于是可变借用
        assert_eq!(6, c.get());

        // Rust 规则 一个数据只有一个所有者, Rc/Arc让一个数据可以拥有多个所有者
        // Rust 规则 要么多个不可变借用，要么一个可变借用 RefCell实现编译期可变、不可变引用共存
        
        let c = RefCell::new(String::from("hello"));
        let _s = c.borrow(); // 通过 borrow 方法获取 Ref 类型的智能指针
        let _s2 = c.borrow_mut(); // 使用 RefCell 时，违背借用规则会导致运行期的 panic

        //Cell 没有额外的性能损耗，例如以下两段代码的性能其实是一致的：
        // code snipet 1
        let x = Cell::new(1);
        let y = &x;
        x.set(2);
        y.set(3);
        println!("{}", x.get());

        // code snipet 2 
        // 编译不通过
        // let mut x = 1;
        // let y = &mut x;
        // x = 2;
        // *y = 3;
        // println!("{}", x);

    }
    
    #[test]
    fn test_rc_cell() {
        use std::cell::RefCell;
        use std::rc::Rc;
        // Rc 和 RefCell 在一起使用，前者可以实现一个数据拥有多个所有者，后者可以实现数据的可变性：
        let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

        let s1 = s.clone();
        let s2 = s.clone();
        // let mut s2 = s.borrow_mut();
        s2.borrow_mut().push_str(", on yeah!");

        println!("{:?}\n{:?}\n{:?}", s, s1, s2);
        
    }
}

