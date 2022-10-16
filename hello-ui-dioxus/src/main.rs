/*
 * @Author: plucky
 * @Date: 2022-10-06 17:28:51
 * @LastEditTime: 2022-10-10 22:58:38
 * @Description:
 */
#![allow(non_snake_case)]

use dioxus::{prelude::{*}};

// run desktop app
fn main() {
    console_error_panic_hook::set_once();
    start();
}

// trunk serve
#[cfg(feature = "web")]
fn start(){
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

#[cfg(feature = "desktop")]
fn start(){
    // dioxus::desktop::launch(app);
    // dioxus::desktop::launch_cfg(app, |c| c.with_window(|w| w.with_title("My App")));
    dioxus::desktop::launch_cfg(
        app,
        |w|w.with_window(|w|w.with_title("App"))
        .with_custom_index(
        r#"
        <!DOCTYPE html>
        <html>
          <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>My App</title>

          </head>
          <body>
            <div id="main"> </div>
            <script>
              window.onresize = function(){
                  var scale = window.innerWidth / 1100;
                  document.body.style.transform = 'scale(' + scale + ')';
                  document.body.style.transformOrigin = 'top';
              }
              window.onresize();
             
          </script>
          </body>
        </html>
        "#.into(),
        ),
    );

}


fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);
    let vec = vec![("Alice", 20), ("Bob", 21), ("Carol", 22)];
    let data = use_ref(&cx, || vec);

    #[cfg(feature = "desktop")]
    let window = dioxus::desktop::use_window(&cx);
    let level = use_state(&cx, || 1.0);

    #[cfg(feature = "desktop")]
    let style_str = rsx!(style {[include_str!("../assets/app.css")]});
    #[cfg(feature = "web")]
    let style_str = rsx!{""};
    
    cx.render(rsx!(
        style_str
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "加数" }
        button { onclick: move |_| count -= 1, "减少" }
        //*count.make_mut()+=1;
        
        input {
            r#type: "number",
            value: "{level}",
            oninput: |e| {
                let num = e.value.parse::<f64>().unwrap_or(1.0);
                level.set(num);
                // window.set_zoom_level(num);
                #[cfg(feature = "desktop")]
                window.set_title(num.to_string().as_str());
            }
        }
        
        button {
            onclick: move |_| {
                if count.get() > &9 {
                    count.set(0);
                    data.write().clear();
                    return
                }
                count += 1;
                 data.write().push(("Dora".into(), *count.current()));
            },
            // class: "circle",
            "按钮"
        }
        
        // // 表格数据
        // table {
        //     tr { th { "Name" } th { "Age" } }
        //     data.read().iter().map(|(name, age)| {
        //         rsx!{tr { 
        //             td {"{name}"}
        //             td {"{age}"} 
        //         }}
        //     })
            
        // }
        
        section{
           Addlist2{todos: data}
        }
        
    ))
    
}

#[derive(Props,PartialEq)]
pub struct AddlistProps<'a> {
    // name: &'a str,
    todos: &'a UseRef<ListData>,
}

// 首字母需要大写
pub fn Addlist<'a>(cx:Scope<'a,AddlistProps<'a>>)->Element{
    let todos = cx.props.todos.read();
    cx.render(rsx!{
        table {
            tr { th { "Name" } th { "Age" } }
            todos.iter().map(|(name, age)| {
                rsx!{tr { 
                    td {"{name}"}
                    td {"{age}"} 
                }}
            })
            
        }
    })
}
type ListData = Vec<(&'static str, i32)>;

// 方式二
 #[inline_props]
pub fn Addlist2<'a>(cx:Scope, todos:&'a UseRef<ListData>)->Element{
    let todos = todos.read();
    cx.render(rsx!{
        table {
            tr { th { "Name" } th { "Age" } }
            todos.iter().map(|(name, age)| {
                rsx!{tr { 
                    td {"{name}"}
                    td {"{age}"} 
                }}
            })
            
        }
    })
}