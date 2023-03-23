use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home_page::HomePage;
use crate::pages::article_page::MarkdownArticlePage;
use crate::router::Route;




fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::NotFound => html! { <div> <h1>{ "404" }</h1> <img src="resources/images/coa.png"/> </div> },
        Route::Post { id } => html!{
            <MarkdownArticlePage article_id = {id}></MarkdownArticlePage>
        },
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <div  class = "dark">
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}
