mod app;
mod article;
mod components;
mod router;
use app::Main;

fn main() {
    yew::Renderer::<Main>::new().render();
}
