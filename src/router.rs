use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/assetgen")]
    AssetGen,
    #[not_found]
    #[at("/404")]
    NotFound,
}
