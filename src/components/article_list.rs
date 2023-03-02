use gloo_net::http::Request;
//components that lists the articles written in the resources directory
use yew::prelude::*;

#[function_component(ArticleList)]
pub fn articles_list() -> Html
{
    let article_list = use_state(|| Vec::<String>::new());
    {
        let articles = article_list.clone();
        use_effect_with_deps(|_|{
            wasm_bindgen_futures::spawn_local(async move {
                articles.set(Request::get("/resources/articles/articles.csv")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap().split(",").map(|slice| slice.to_owned()).collect::<Vec<String>>()
                );
            });
        }, ());
    }
    html!{
        <>
            <h1>{"Recent posts"}</h1>
            <ol class="list-disc">
                {article_list.iter().map(|name| html!(<li>{name}</li>)).collect::<Html>()}
            </ol>
        </>
    }
}

