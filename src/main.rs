mod app;
mod components;
mod pages;
mod router;
use app::Main;
mod theme;
fn main() {
    yew::Renderer::<Main>::new().render();
}
