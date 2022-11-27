use yew::prelude::*;

/*
 * @Author: plucky
 * @Date: 2022-10-09 17:33:45
 * @LastEditTime: 2022-11-27 10:23:06
 * @Description:
 */
pub enum Msg {
    AddOne,
    SubOne,
}

pub struct Model {
   pub value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::SubOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <span>{"按钮组件"}</span>
            <p>{ self.value }</p>
            <div class="btn-group">
                <button onclick={ctx.link().callback(|_| Msg::AddOne)} class="btn btn-success">{ "+1" }</button>
                <button onclick={ctx.link().callback(|_| Msg::SubOne)} class="btn btn-success">{ "-1" }</button>
                <button class="btn btn-success">{"Click Me"}</button>
            </div>
            </>

        }
    }

    

    
}
