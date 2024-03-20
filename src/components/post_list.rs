use gloo_net::http::Request;

use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct PostListProps {
    #[prop_or_default]
    pub post_displayed_limits: Option<usize>,
}

///Component that lists the articles written in the resources directory
#[function_component(PostList)]
pub fn post_list(props: &PostListProps) -> Html {
    let post_displayed_limits = props.post_displayed_limits;
    let post_list = use_state(|| Vec::<String>::new());
    {
        let post = post_list.clone();
        use_effect(move || {
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
                        .take(post_displayed_limits.unwrap_or(usize::MAX))
                        .collect::<Vec<String>>(),
                );
            });
        });
    }
    html! {
        <div class="flex flex-col gap-5">
            <div class="text-4xl">{"Posts: "}</div>
            <div class="w-full grid grid-cols-1 justify-items-start gap-y-5">
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
        </div>
    }
}
