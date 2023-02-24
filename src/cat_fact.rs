use std::collections::HashMap;

use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
#[derive(PartialEq)]
struct CatFactState
{
    fact : String
}

enum CatFactAction
{
    Set(String),
    Refresh
}

impl Reducible for CatFactState
{
    type Action = CatFactAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let fact = match action
        {
            CatFactAction::Set(the_fact) => {
                the_fact
            },
            CatFactAction::Refresh => self.fact.clone(),

        };
        Self{fact}.into()
    }
}


#[function_component(CatFact)]
pub fn cat_fact() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    let response_text = use_reducer(|| CatFactState {fact : String::new()} );

    let refresh_callback =
    {
        let fetched = response_text.clone();
        Callback::from(move |_|{
            let fetched = fetched.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let request: HashMap<String,serde_json::Value> = Request::get("https://catfact.ninja/fact")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                fetched.dispatch(CatFactAction::Set(format!("{:?}",request)));
            })})
    };
    html! {
        <div>
            <h1>{ "Cat fact example" }</h1>
            <button {onclick}>{ "Go Home" }</button>
            <p> {"Here is some text: "} </p>
            <p> {String::from(&*response_text.fact)}</p>
            <button onclick={refresh_callback}>{ "New fact" }</button>
        </div>
    }
}