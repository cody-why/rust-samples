#[cfg(test)]
mod memory {
    /*
    s: 0x70000b76d6e8
    s = hello
    m: 0x70000b76d788
    m = hello
    s: 0x70000b76d6e8
    s = world
    */
    #[test]
    fn 理解变量内存() {
        println!("{}", std::mem::size_of::<f64>());
        // s是一个栈中的位置，保存了一个胖指针指向堆中的数据
        let mut s = "hello".to_string();
        // &s指针，该位置中保存指向堆位置的地址值
        println!("s: {:p}", &s);
        // 值是hello
        println!("s = {}", s);
        //将s中的胖指针移给了m，m保存胖指针指向堆中数据，s变回未初始化状态
        let m = s;

        println!("m: {:p}", &m);
        println!("m = {}", m);

        //s无法打印,因为已经移给了m
        //println!("s = {}",s);

        // 重新初始变量
        s = "world".to_string();
        //地址还是那个栈地址,值已经变了
        println!("s: {:p}", &s);
        println!("s = {}", s);

        {
            // x 在这里无效的，它尚未声明
            let x: String = "helLo".to_string();
            println!("x = {}", x);
        } // 此作用域已结束，x不再有效
          // 会自动调用Drop trait的drop函数来销毁该变量绑定在内存中的数据.
    }

    ///#[derive (Debug,Copy,Clone)]实现了Copy特性,依赖Clone
    #[derive(Debug, Copy, Clone)]
    #[allow(dead_code)]
    struct A {
        //所有成员都是copy特性类型,但struct不自动实现copy
        //如果成员中还有move类型,struct不能实现Copy
        a: i32,
        b: i32,
    }
    ///元组,枚举,如果成员是Copy则实现Copy Trait
    ///
    /// 是否实现了 Copy Trait
    /// 所有权机制：保证內存安全和性能
    /// 所有权转移．每个值都有一个所有者.
    #[test]
    fn 理解copy_move() {
        // Box是一个堆内存
        // Box 没有实现Copy trait, 所以是move特性
        let a = Box::new(5);
        // a的所有权转移b
        let b = a;
        println!("{:?}", b);
        //println!("{:?}", a);//borrow of moved value

        //struct 手动实现了 Copy, 复制成功
        let a = A { a: 1, b: 2 };
        let b = a;
        println!("{:?}", a);
        println!("{:?}", b);

        //元组,枚举,如果成员是Copy则实现Copy Trait
        let c = (1, 2, 3);
        let _d = c;
        println!("{:?}", c);

        let mut c = [7, 8, 9];
        // 可变借用,必须是mut
        foo(&mut c);
        assert_eq!([3, 8, 9], c);
    }

    fn foo(v: &mut [i32]) {
        v[0] = 3;
    }

    /// 不能存在input和output借用同一个绑定的情况
    /// 解引用会发发生所有权转移
    #[allow(dead_code)]
    fn compute(input: &i32, output: &mut i32) {
        let a = *input;
        //因为input是一个不可变借用，不会改变．寄存器中（提升性能）
        //*为解引用
        if a > 20 {
            *output = 1;
        } else if a < 20 {
            *output = 2;
        }
    }
}
