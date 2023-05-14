use crate::pages::home_page::HomePage;
use crate::pages::post_page::MarkdownPostPage;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::NotFound => {
            html! { <div> <h1>{ "404" }</h1> <img src="resources/images/coa.png"/> </div> }
        }
        Route::Post { id } => html! {
            <MarkdownPostPage article_id = {id}></MarkdownPostPage>
        },
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        //enforce dark mode
        <div class = "dark">
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        </div>
    }
}
