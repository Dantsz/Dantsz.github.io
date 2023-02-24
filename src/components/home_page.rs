use yew::prelude::*;
use stylist::yew::styled_component;

#[function_component(HomePage)]
pub fn home_page() -> Html
{
    html!{
        <>
        <h1 class ="text-lg font-semibold text-slate-500">{"Home"}</h1>
        <h2 class ="text-red-600"> {"About text"}</h2>
        </>
    }
}