use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::article::Article;
use crate::cat_fact::CatFact;
use crate::router::Route;
use gloo_net::http::Request;



fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::CatFact => html! {
            <CatFact />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
