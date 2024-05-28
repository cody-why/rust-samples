/*
 * @Author: plucky
 * @Date: 2023-07-23 11:42:18
 * @LastEditTime: 2023-07-23 21:16:37
 */

// #![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

#[tokio::main]
async fn main() {
    let async_struct = AsyncStruct;
    let r = async_struct.run("我的abc".to_string()).await;
    println!("result: {}", r);
    let r = async_struct.deff().await;
    println!("result: {}", r);
}

trait AsyncTrait {
    type Future<'l>: core::future::Future<Output = usize>+'l where Self: 'l;
    fn run(&self, st: String) -> Self::Future<'_>;
    // fn deff(&self) -> impl core::future::Future<Output = usize> + '_ {
    //     async move { 1 }
    // }
    async fn deff(&self) -> usize {
        1
    }

    async fn run2<T: Into<String>>(&self, st: T) -> usize;

    
}

struct AsyncStruct;

impl AsyncStruct {
    async fn sum(&self,st: String) -> usize{
        println!("sum");
        st.chars().count()
    }
    
}

impl AsyncTrait for AsyncStruct {
    type Future<'l> = impl core::future::Future<Output = usize>+'l;

    fn run(& self, st: String) -> Self::Future<'_> {
        async move {
            println!("AsyncStruct");
            self.sum(st).await
        }
    }

    async fn deff(&self) -> usize {
        self.sum("abc".to_string()).await
    }

    async fn run2<T: Into<String>>(&self, st: T) -> usize {
        self.sum(st.into()).await
    }
    
    // fn deff(&self) -> impl core::future::Future<Output = usize> + '_  {
    //     async move { self.sum("abc".to_string()).await }
    // }
}

