use yew::prelude::*;

pub struct Paintbar;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Paintbarprops {
    pub clicks: u32,
    pub test_string: String
}

impl Component for Paintbar {
    type Message = ();
    type Properties = Paintbarprops;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let msg = format!("My parent has been clicked {} times", ctx.props().clicks);
        let test_msg = ctx.props().test_string.clone();
        html! {
            <div class="child">
                <div>{msg}</div>
                <div>{r#"this is the test_string: "#}{test_msg}</div>
            </div>
        }
    }
}

