use yew::prelude::*;
use crate::css;
use crate::webgl_components::colorwheel::Colorwheel;
use std::collections::HashMap;

pub struct Paintbar{
    stl: HashMap<String,String>
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Paintbarprops {
    pub clicks: u32,
    pub test_string: String
}

impl Component for Paintbar {
    type Message = ();
    type Properties = Paintbarprops;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stl: css::paintbar::styles()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let msg = format!("My parent has been clicked {} times", ctx.props().clicks);
        let test_msg = ctx.props().test_string.clone();
        html! {
            <div>
                {r#"test 1"#}
                <div style = {self.stl["colorwheelcontainer"].clone()}>
                    {r#"testy test"#}
                    <Colorwheel/>
                </div>
            </div>
        }
    }
}

