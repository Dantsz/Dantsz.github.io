use comrak::{markdown_to_html, ComrakOptions};
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MWPostProps {
    pub src: String,
    pub prose_theme: String,
}
///Component that renders a markdown article
#[function_component(MarkdownPost)]
pub fn markdown_article(props: &MWPostProps) -> Html {
    let markdown_string = use_state(|| String::new());
    {
        let state = markdown_string.clone();
        let url = props.src.clone();
        // This needs to be wrapped in the wasm_bindgen_futures::spawn_local becuase it's an async function, the blocking api of reqwest is not available in wasm :(
        // Get the markdown file from the server when the component is loaded
        use_effect(|| {
            wasm_bindgen_futures::spawn_local(async move {
                let request: String = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                state.set(request);
            });
        });
    }
    let markdown = Html::from_html_unchecked(AttrValue::from(markdown_to_html(
        &*markdown_string,
        &ComrakOptions::default(),
    )));
    html! {
        //Tailwind prose plugin and inverse in dark
        <article class= {classes!("prose", "dark:prose-invert" ,&props.prose_theme)}>
            {markdown}
        </article>
    }
}
