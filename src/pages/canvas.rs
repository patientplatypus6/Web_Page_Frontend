use yew::prelude::*;
use crate::components::paintbar::Paintbar; 
use std::collections::HashMap;
use crate::css;


#[derive(Clone)]
pub struct Canvas {
    test_string: String,
    nr_of_clicks: u32,
    stl: HashMap<String,String>
}

pub enum Msg {
    ButtonClick,
}

impl Component for Canvas {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stl: css::canvas::styles(),
            test_string: "this is the test string".to_string(), 
            nr_of_clicks: 0 
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClick => {
                self.nr_of_clicks += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let clicks = self.nr_of_clicks;
        let test_string = self.test_string.clone();

        html! {
            <div style={self.stl["main"].clone()} >
                <div style={self.stl["flexcontainer"].clone()}>
                    <div style = {self.stl["flex1"].clone()}/>
                    <div style = {self.stl["flex1"].clone()}/>
                    <div style = {self.stl["paintbarcontainer"].clone()}>
                        <Paintbar {clicks} {test_string}/>
                    </div>
                </div>
            </div>
        }
    }
}

