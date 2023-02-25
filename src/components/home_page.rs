use yew::prelude::*;
use stylist::yew::styled_component;

#[function_component(HomePage)]
pub fn home_page() -> Html
{
    html!{
        <div class="grid gap-4 grid-cols-5 grid-rows-1">
            <div>
                <div class = "grid grid-cols-1 grid-rows-2">
                    <img src="resources/images/coa.png" alt="rust image"/>
                    <h1 class ="w-full text-lg text-slate-500">{"Home"}</h1>
                </div>
            </div>
            <div class = "col-span-4">
                <h2 class = {classes!("w-full","text-red-600", "first-letter:uppercase" ,"first-letter:text-7xl")}> {"About text"}</h2>
            </div>
        </div>
    }
}