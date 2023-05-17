use gloo_net::http::Request;

use yew::prelude::*;
///Component that lists the articles written in the resources directory
#[function_component(PostList)]
pub fn post_list() -> Html {
    let post_list = use_state(|| Vec::<String>::new());
    {
        let post = post_list.clone();
        use_effect(|| {
            wasm_bindgen_futures::spawn_local(async move {
                post.set(
                    Request::get("/resources/posts/posts.csv")
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap()
                        .split(",")
                        .map(|slice| slice.to_owned())
                        .collect::<Vec<String>>(),
                );
            });
        });
    }
    html! {
        <>
            <h1>{"Posts: "}</h1>
            <div class="w-full grid grid-cols-1 justify-items-start gap-y-1">
                {post_list.iter().
                                map(|name| html!(
                                    <div class = " w-full bg-gradient-to-r from-gray-800 to-gray-900 px-2">
                                        <p>{name}</p>
                                        <a href={format!("#/post/{}",name)}
                                           class = "hover:underline text-cyan-500">
                                           { "Read more" }
                                        </a>
                                    </div>
                                ))
                                .collect::<Html>()}
            </div>
        </>
    }
}
