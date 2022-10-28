/*
 * @Author: plucky
 * @Date: 2022-10-15 16:04:30
 * @LastEditTime: 2022-10-15 16:38:21
 * @Description: 
 */


use yew::prelude::*;


pub struct Counter {
    props: Props,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub count: i64,
}

impl Component for Counter {
    type Message=();

    type Properties=Props;

    // html 创建的时候调用
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }
    // fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    //     false
    // }

    // 通知更新属性
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = ctx.props().clone();
        true
    }

    // 用属性来更新组件
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <span>{"属性组件"}</span>
                <p>{ self.props.count }</p>
                <p>{ _ctx.props().count }</p>
            </div>
        }
    }

}
    
