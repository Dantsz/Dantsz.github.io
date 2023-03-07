mod app;
mod components;
mod router;
mod pages;
use app::Main;

fn main() {
    yew::Renderer::<Main>::new().render();
}
