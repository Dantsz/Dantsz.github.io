use yew::prelude::*;

use crate::{components::post::MarkdownPost, theme::BLOG_STYLE};

#[derive(Properties, PartialEq)]
pub struct MWPostPageProps {
    pub article_id: String,
}
///Page for a mardown post
#[function_component(MarkdownPostPage)]
pub fn post_page(props: &MWPostPageProps) -> Html {
    html!(
        <div class = {classes!("w-full", "h-full", "grid", "grid-cols-8",BLOG_STYLE)}>
            <div class = "col-span-2"> </div>
            <div class = "col-span-6">
            <MarkdownPost prose_theme= "prose-zinc" src = {format!("/resources/posts/{}",props.article_id)}></MarkdownPost>
            </div>
        </div>
    )
}
