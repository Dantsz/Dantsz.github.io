use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::cat_fact::CatFact;
use crate::components::home_page::HomePage;
use crate::router::Route;




fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
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
