/*
 * @Author: plucky
 * @Date: 2022-10-09 19:51:18
 * @LastEditTime: 2022-10-10 23:00:24
 * @Description: 
 */

use dioxus_chain::app::app;



fn main() {
    console_error_panic_hook::set_once();
    
    dioxus::desktop::launch_cfg(app, |c| 
        c.with_window(|c| 
        c.with_title("Chain")
            .with_always_on_top(true)
    ));
}

