/*
 * @Author: plucky
 * @Date: 2022-10-15 00:32:59
 * @LastEditTime: 2022-10-27 16:57:48
 * @Description: 
 */

#![allow(non_snake_case)]

use dioxus::{prelude::*};
// use tracing::info;
use rsa::{PublicKey, RsaPublicKey, PaddingScheme, pkcs8:: DecodePublicKey};
struct Params{
    pub data: Vec<(String,String)>,
}

impl Params {
    pub fn new() -> Self {
        Self {
            data: vec![
                ("appId".to_string(),"".to_string()),
                ("gameId".to_string(),"".to_string()),
                ("at".to_string(),"".to_string()),
            ]
        }
    }

    pub fn add_params(&mut self, key: String, value: String) {
        self.data.push((key,value));
    }

    pub fn remove_params(&mut self, id: usize) {
        self.data.remove(id);
    }

    pub fn update_params(&mut self, id: usize, key: String, value: String) {
        self.data[id] = (key,value);
    }
    pub fn generate_sign(&self, public_key:&str) -> String {
        // 过滤空值
        let mut data = self.data.iter().filter(|(k,v)|{!k.is_empty() && !v.is_empty()}).collect::<Vec<_>>();
        // 排序
        data.sort_by(|a,b|{
            a.0.cmp(&b.0)
        });
        // 拼接
        let str = data.iter()
        .map(|(k,v)| format!("{}={}",k.trim(),v.trim()))
        .collect::<Vec<String>>().join("&");
        let md5 = md5::compute(str.as_bytes());
        let md5 = format!("{:X}", md5);

        // rsa pkcs8
        let public_key = RsaPublicKey::from_public_key_pem(public_key).unwrap();
        
        let mut rng = rand::rngs::OsRng;
        let rsau8 = public_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), md5.as_bytes()).unwrap();
        let sign = base64::encode(rsau8);
        // let json =  self.generate_json(&data, &sign);

        format!("params: {}\nmd5: {:}\nsign: {}\n",str, md5,sign)
    }

    #[allow(dead_code)]
    pub fn generate_json(&self,data: &Vec<&(String, String)>, sign: &str) -> String {
        let mut json = String::with_capacity(1024);
        json.push_str("{");
        for (k,v) in data {
            json.push_str(&format!("\"{}\":\"{}\",",k,v));
        }
        json.push_str(&format!("\"sign\":\"{}\"",sign));
        json.push_str("}");
        json
       
    }


}

pub fn view(cx: Scope)->Element{
    
    cx.render(rsx!{
        div {
            class: "flex justify-center items-center text-sm",
            // Form Elements
            Forms{}
            
        }
    })
}


fn Forms(cx: Scope)->Element{
    let params = use_ref(&cx, || Params::new());
    let result = use_state(&cx, ||String::new());
    let public_key = use_state(&cx, ||include_str!("../../public_key.pem").to_string());

    let check_params = move || {
        params.with(|p| {
            result.set(p.generate_sign(&public_key));
        });
       
    };

    cx.render(rsx!{
        div {
            class: "mt-6 max-w-2xl w-11/12  mx-auto",// 上边距6，最大宽度2xl，宽度11/12，水平居中
            
            div {
                class: " p-4 bg-white rounded-md shadow-md",
                h4 {
                    class: "text-gray-600",
                    "Params"
                }
                div {
                    class: "grid grid-cols-3 gap-1",
                    // 填充数据
                    params.read().data.iter().enumerate().map(|(id,(key, value))|{
                        let k = key.clone();
                        let v = value.clone();
                        
                    rsx!(
                        div {
                            class: "flex col-span-1",
                            input {
                                class: "form-input ",
                                r#type: "text",
                                value: "{key}",
                                // disabled: format_args!("{}", id < 3 ),
                                oninput: move |e| {
                                    params.with_mut(|p|{
                                        p.update_params(id, e.value.clone(), v.clone());
                                    });
                                },
                                
                            }
                            span {
                                class: "text-gray-600 self-center m-1",
                                ":"
                            }
                        }
                        
                        div{
                            class: "flex col-span-2",
                        
                        input {
                            // id: "{id}",
                            class: "form-input",
                            r#type: "text",
                            // placeholder: "Enter your appid",
                            value: "{value}",
                            oninput: move |e| {
                                // info!("input: {:?} {:?}", k.clone(), v.clone());
                                params.with_mut(|p|{
                                    p.update_params(id, k.clone(), e.value.clone());
                                });
                            },
                        }
                        button {
                            class: "px-2 ml-2 mt-1 rounded-md bg-gray-500  text-gray-200 hover:bg-red-600",
                            // hidden: format_args!("{}", id < 3 ),
                            onclick: move |_|{
                                params.with_mut(|p|{
                                    p.remove_params(id);
                                });
                                
                            },
                            "Del"
                        }
                        }
                    )
                    })
                    
                }
                div {
                    class: "flex justify-end mt-2",
                    button {
                        r#type: "button",
                        class: "form-button",
                        onclick: move |_|{
                            // info!("onclick: {:?}", 1);
                            params.with_mut(|p|{
                                p.add_params("".into(), "".into());
                            });
                            
                        },
                        "Add param"
                    }
                }
                h4 {
                    class: "text-gray-600 mt-2",
                    "Public key"
                }
                textarea {
                    class: "text-sm w-full h-52 md:h-36 border-1 border-gray-200 rounded-md",
                    value: "{public_key}",
                    oninput: move |e| {
                        public_key.set(e.value.clone());
                    },
                }
                
                div {
                    class: "flex justify-end mt-2",
                    button {
                        r#type: "button",
                        class: "form-button",
                        onclick: move |_|{
                            check_params();
                        },
                        "Sign"
                    }
                }
            
            }
            // 结果部位
            
            div {
                class: " mt-1 p-4 bg-white rounded-md shadow-md",
                h4 {
                    class: "text-gray-600",
                    "Result"
                }
                textarea {
                    class: "text-sm w-full h-52 md:h-44 border-1 border-gray-200 rounded-md",
                    readonly: "true",
                    rows: "5",
                    // cols: "100",
                    value: "{result}",
                    
                }
            }
        
        }
    })
}

mod icons{
    
    

}