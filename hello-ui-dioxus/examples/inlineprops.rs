/*
 * @Author: plucky
 * @Date: 2022-10-07 20:40:35
 * @LastEditTime: 2022-10-08 10:13:56
 * @Description: 
 */

#![allow(non_snake_case)]
#![allow(dead_code)]

use dioxus::prelude::*;


fn main() {
    #[cfg(feature = "desktop")]
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let state = use_state(&cx, || 1);
    let data = use_ref(&cx, || 1);
    cx.render(rsx! {
        div {
            button {
                onclick:move |_| {
                    state.set(state.get() + 1);
                },
                "Click me"
            }
            Thing1 { _a: 1 },
            Thing2 { _a: 1 },
            Thing3 { _a: data },
            Thing4 { _a: state },
        }
    })
}


#[inline_props]
fn Thing1<T>(cx: Scope, _a: T) -> Element {
    cx.render(rsx! { "" })
}

#[inline_props]
fn Thing2(cx: Scope, _a: u32) -> Element<'a> {
    cx.render(rsx! {p{"{_a}"} })
}

#[inline_props]
fn Thing3<'a, T>(cx: Scope<'a>, _a: &'a T) -> Element<'a> {
    // let _b:&UseRef<i32>  = _a as &UseRef<i32>;
    
    cx.render(rsx! { p{}  })
}

#[inline_props]
fn Thing4<'a>(cx: Scope<'a>, _a: &'a u32) -> Element<'a> {
    cx.render(rsx! { p{"{_a}"}})
}

