use comrak::{ComrakOptions, markdown_to_html};
use gloo_console::log;
use gloo_net::http::Request;
use yew::prelude::*;

#[derive (Properties,PartialEq)]
pub struct MWArticleProps{
    pub src: String,
    #[prop_or("prose-zinc".to_owned())] // default theme
    pub prose_theme : String
}

#[function_component(MarkdownArticle)]
pub fn markdown_article(props: &MWArticleProps) -> Html
{
    let markdown_string = use_state(|| String::new());
    {
        let state = markdown_string.clone();
        let url = props.src.clone();
        log!(&url);
        use_effect_with_deps(|_|{
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
        }, ());
    }
    log!(&*markdown_string);
    let markdown =  Html::from_html_unchecked(AttrValue::from(markdown_to_html(&*markdown_string,&ComrakOptions::default())));
    html!
    {
        <article class= {classes!("prose", "dark:prose-invert" ,&props.prose_theme)}>
            {markdown}
        </article>
    }
}
