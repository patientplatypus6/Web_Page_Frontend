use yew::prelude::*;
use yew_router::prelude::*;
use yew::functional::*;
use std::collections::HashMap;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::router::BrowserRouter;

use crate::pages::home::Home;
use crate::pages::canvas::Canvas;
use crate::pages::mdpages::Mdpages;
use gloo_net::http::Request;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/mdpages")]
    Mdpages,
    #[at("/canvas")]
    Canvas,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {
    
    html! {
        <BrowserRouter>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                { "Home" }
            </Link<Route>> 
            <Link<Route> classes={classes!("navbar-item")} to={Route::Canvas}>
                { "Canvas" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Mdpages}>
                { "Mdpages" }
            </Link<Route>>
            
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { < Home /> },
        Route::Canvas => html! { < Canvas />},
        Route::Mdpages => html! { <Mdpages /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

