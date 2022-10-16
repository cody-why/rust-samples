#![allow(non_snake_case)]

use dioxus::prelude::{*};
use dioxus_heroicons::{ solid::Shape, Icon};

use crate::chain::Chain;

pub fn app(cx: Scope) -> Element {
    // 设置上下文
    use_context_provider(&cx, || {Chain::new()});
    //取出上下文
    let chain = use_context::<Chain>(&cx).unwrap();
    let val = use_state(&cx, || "".to_string());

    cx.render(rsx!{
        div{
            // link{href:"https://fonts.googleapis.com/css?family=Material+Icons",rel:"stylesheet" }
            style { [include_str! ("../output.css")] }
            
            header{
                // i{class:"material-icons inventory","inventory"}
                // BlockIcon{}
                i{
                    RightIcon{}
                }
                h1{[format!("Blocks: {}", chain.read().get_totals())]}
                
            }
        }
        main{
            section{
                input{
                    // class:"",
                    placeholder:"Enter a new block",
                    autofocus:"true",
                    value:"{val}",
                    oninput:move |e| {
                        val.set(e.value.clone());
                    },

                    onkeydown: move |e| {
                        if e.key == "Enter" {
                            chain.write().add_block(val.get().clone());
                            val.set("".to_string());
                        }
                        
                    }
                }
                Test_get{}
            }
            section{
                br{}
                ul{
                    // class:"",
                    chain.read().get_blocks().iter().map(|block|{rsx!{
                        BlockItem{key:"{block.index}", id: block.index}
                    }
                    })
                }
            }
        }
    })
}


#[derive(PartialEq, Props)]
pub struct BlockItemProps{
    id: usize,
}


pub fn BlockItem(cx: Scope<BlockItemProps>) -> Element {
    let chain = use_context::<Chain>(&cx).unwrap();
    let block = chain.read().get_block(cx.props.id).unwrap();
    let hash = block.get_hash();
    // 输出GTM+8时间
    let curtime = block.get_current_time().with_timezone(&chrono::Local);
    cx.render(rsx!{
        li{
            // class:"",
            strong{
                "{block.data}"
            }
            p{
                "{hash}"
            }
            em{
                "previous: {block.previous_hash}"
            }
            p{
                "{curtime}"
            }
        }
    })
}

// pub fn BlockIcon(cx: Scope) -> Element {
//     cx.render(rsx!{
//         //https://heroicons.com
//         // svg{
//         //     class:"h-6 w-6",
//         //     view_box: "0 0 24 24",
//         //     fill:"none",
//         //     stroke:"currentColor",
//         //     stroke_width: "1.5",
                // stroke_linecap: "round",
                // stroke_linejoin: "round",
//         //     path{
//         //         d:"M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"
//         //     }
            
//         // }
//         IconButton {
//             // onclick: move||{},
//             class: "btn",
//             title: "Delete it",
//             disabled: false,
//             size: 30,
//             icon: Shape::BookOpen,
//         }
//     })
// }

pub fn RightIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        Icon {
            size: 30,
            icon: Shape::BookOpen,
            // fill: "blue",
        }
    })
}

fn Test_get(cx: Scope) -> Element {
    // let content = use_future(&cx, (), |_| async move {
    //     reqwest::get("https://dog.ceo/api/breeds/list")
    //         .await
    //         .unwrap()
    //         // .json::<ListBreeds>()
    //         .text()
    //         .await
    // });
    
    // let status = match content.value() {
    //     Some(Ok(_val)) => "success",
    //     Some(Err(_e)) => "Error",
    //     None => "loading!",
    // };
    let status = "success";
    cx.render(rsx! { h1 { "{status}" } })
}
