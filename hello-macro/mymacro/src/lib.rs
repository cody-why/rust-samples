/*** 
 * @Author: plucky
 * @Date: 2022-06-06 23:15:34
 * @LastEditTime: 2022-07-13 22:25:54
 * @Description: 
 */

//use syn::{parse_macro_input, AttributeArgs, Item};
use proc_macro::TokenStream;

///定义一个属性宏,属性在参数attr,函数定义在item
#[proc_macro_attribute]
pub fn my_first_attribute_proc_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //用eprintln!输出TokenStream
    eprintln!("------attr------");
    eprintln!("{:#?}", _attr);
    eprintln!("------item------");
    eprintln!("{:#?}", item);
    //什么也不做,直接返回
    item
}

use quote::quote;

/// 此属性宏将注册并运行测试3次，只有3次都失败时才会出错。
#[proc_macro_attribute]
pub fn flaky_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    //syn 把fn转换为itemfn,得到Ident名字,就是fn名字
    let input_fn = syn::parse_macro_input!(input as syn::ItemFn);
    let name = input_fn.sig.ident.clone();
    //输出看看
    //eprintln!("{:#?}",input_fn);
  
    // quote宏 把语法树转换为 proc_macro2:TokenStream
    let quote = quote! {
      #[test]//test宏
      fn #name() {//函数名
        #input_fn//函数体

        for i in 0..3 {//循环3次
          println!("flaky_test retry {}", i);
          //调用闭包,不会panic.成功返回ok,panic返回err
          let r = std::panic::catch_unwind(|| {
            #name();
          });
          if r.is_ok() {
            return;//如果有成功就返回
          }
          //最后返回错误
          if i == 2 {
            std::panic::resume_unwind(r.unwrap_err());
          }
        }
      }
    };
    //返回tokenstream
    TokenStream::from(quote)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
