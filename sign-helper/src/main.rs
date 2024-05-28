/*
 * @Author: plucky
 * @Date: 2022-10-25 14:41:28
 * @LastEditTime: 2022-10-27 11:43:42
 * @Description:
 */
mod views;
mod tests;
use dioxus::prelude::*;
use views::*;


fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

pub fn app(cx: Scope) -> Element {
    cx.render(
        rsx! {
            forms::view {}
        }
    )
}