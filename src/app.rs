use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::article::Article;
use gloo_net::http::Request;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    let response_text = use_state(|| String::new());
    {
        let fetched = response_text.clone();
        use_effect(move ||{
            let fetched = fetched.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let request: HashMap<String,serde_json::Value> = Request::get("https://catfact.ninja/fact")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                fetched.set(format!("{:?}",request));
            });

        });
    }

    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
            <p> {"Here is some text: "} </p>
            <p> {(*response_text).clone()}</p>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
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
// #[function_component(App)]
// pub fn app() -> Html {
//     let article_str = std::str::from_utf8(include_bytes!("assets/firstarticle.json")).expect("This should have been a compile time error");
//     let article : Article = serde_json::from_str(&article_str).unwrap();
//     html! {
//         <main>
//             <h1>{ article.name }</h1>
//             <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
//         </main>
//     }
// }
