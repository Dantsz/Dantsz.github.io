use yew::prelude::*;

use crate::components::post::MarkdownPost;

#[derive(Properties, PartialEq)]
pub struct MWPostPageProps {
    pub article_id: String,
}

#[function_component(MarkdownPostPage)]
pub fn post_page(props: &MWPostPageProps) -> Html {
    html!(
        <div class ="w-full h-full bg-emerald-200 dark:bg-gray-900 dark:text-indigo-200 grid grid-cols-8">
            <div class = "col-span-2"> </div>
            <div class = "col-span-6">
            <MarkdownPost src = {format!("/resources/articles/{}",props.article_id)}></MarkdownPost>
            </div>
        </div>
    )
}
