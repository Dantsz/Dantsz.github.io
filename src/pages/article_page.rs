use yew::prelude::*;

use crate::components::article::MarkdownArticle;


#[derive (Properties,PartialEq)]
pub struct MWArticlePageProps
{
    pub article_id : String
}

#[function_component(MarkdownArticlePage)]
pub fn article_page(props: &MWArticlePageProps)  -> Html
{
    html!(
        <div class ="w-full h-full bg-emerald-200 dark:bg-gray-900 dark:text-indigo-200 grid grid-cols-8">
            <div class = "col-span-2"> </div>
            <div class = "col-span-6">
            <MarkdownArticle src = {format!("/resources/articles/{}",props.article_id)}></MarkdownArticle>
            </div>
        </div>
    )
}