/*
 * @Author: plucky
 * @Date: 2022-10-09 12:25:01
 * @LastEditTime: 2022-10-15 16:24:17
 * @Description: 
 */

use yew::prelude::*;

mod component;

#[function_component(App)]
pub fn app() -> Html {
    let val = use_state(||0);
    let onclick = {
        let val = val.clone();
        Callback::from(move |_| val.set(*val + 1))
    
    };
    let onclick2 = {
        let val = val.clone();
        Callback::from(move |_| val.set(*val - 1))
    
    };

    html! {
        <>
        <div>  
        <h1>{ "Hello World"}</h1>
        <h1>{ *val}</h1>
        <button onclick={onclick2} class="btn btn-primary">{ "减1" }</button>
        <button onclick = { onclick } class="btn btn-secondary">{ "加1" }</button>
        </div>
        
        <component::hello::Model />
        <component::counter::Counter count={*val} />
        
        
        </>
    }
}
