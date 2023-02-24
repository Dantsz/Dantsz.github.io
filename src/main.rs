mod app;
mod article;
mod cat_fact;
mod router;
use app::Main;

fn main() {
    yew::Renderer::<Main>::new().render();
}
