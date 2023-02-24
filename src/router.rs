use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/cat_fact")]
    CatFact,
    #[not_found]
    #[at("/404")]
    NotFound,
}