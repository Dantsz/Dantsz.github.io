mod app;
mod article;

use app::Main;

fn main() {
    yew::Renderer::<Main>::new().render();
}
