use yew::prelude::*;
use stylist::yew::styled_component;

#[styled_component]
pub fn HomePage() -> Html
{
    html!{
        <>
        <h1 class ={css!(r#"background-color: red;"#)}>{"Home"}</h1>
        <h2 class ={classes!("1")}> </h2>
        </>
    }
}