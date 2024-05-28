/*
 * @Author: plucky
 * @Date: 2023-07-18 12:13:55
 * @LastEditTime: 2023-07-18 21:45:47
 */
#![allow(unused)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

mod util;

#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = quote! {
        // ...
    };

    TokenStream::from(expanded)
}

/// Generates:
/// - service trait
/// - serve fn
/// - Request and Response enums
/// - ResponseFut Future
#[proc_macro_attribute]
pub fn service(attr: TokenStream, input: TokenStream) -> TokenStream {
    let expanded = quote! {
        // ...
    };

    TokenStream::from(expanded)
    
}