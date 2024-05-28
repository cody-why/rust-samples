/*
 * @Author: plucky
 * @Date: 2023-07-28 11:44:57
 * @LastEditTime: 2023-08-19 12:04:30
 */
#![feature(async_fn_in_trait)]

// use async_trait::async_trait;
use core::future::Future;

async fn tester(string: &String) -> String {
    string.into()
}

// static dyn_tester: &dyn App = &tester;

#[tokio::main]
async fn main() {
    // let string = String::from("233");
    // let a = dyn_tester.call(&string).await;
    // println!{"{a}"}
}

// #[async_trait]
trait App: Sync {
    async fn call<'a>(&self, value: &'a String) -> String;
}

trait Wrapper<'a>: Send + Sync {
    type Res: Future<Output = String> + Send + Sync;
    fn wrapped_call(&self, s: &'a String) -> Self::Res;
}

impl<'a, F, Fut> Wrapper<'a> for F
where
    F: Send + Sync + Fn(&'a String) -> Fut,
    Fut: Send + Sync + Future<Output = String>,
{
    type Res = Fut;
    fn wrapped_call(&self, s: &'a String) -> Self::Res {
        self(s)
    }
}

// #[async_trait]
impl<T> App for T
where
    T: for<'a> Wrapper<'a>,
{
    async fn call<'a>(&self, value: &'a String) -> String {
        self.wrapped_call(value).await
    }
}

