use comrak::{ComrakOptions, markdown_to_html};
use gloo_net::http::Request;
use yew::prelude::*;

use crate::components::{article::MarkdownArticle, article_list::ArticleList};


#[function_component(HomePage)]
pub fn home_page() -> Html
{
    let yorvs_load_script = r#"
    import init from '/resources/apps/yorvs_client/yorvs_loader.js';
    init('/resources/apps/yorvs_client/yorvs.wasm');
    "#;
    html!{
        <div class = "bg-emerald-200 dark:bg-gray-800 dark:text-white">
            <div class="grid gap-1 grid-cols-4 grid-rows-1">
                <div>
                    <div class = " grid grid-cols-1 grid-rows-2">
                        <img class = "w-1/2" src="resources/images/coa.png" alt="rust image"/>
                        <h1 class ="w-full text-lg text-slate-500">{"Home"}</h1>
                    </div>
                </div>
                <div class = "col-span-2">
                      <MarkdownArticle src = "https://jaspervdj.be/lorem-markdownum/markdown.txt" />
                      /*<canvas id="notan_canvas"></canvas>
                      <script type="module">{yorvs_load_script}</script>*/
                </div>
                <div>
                    <ArticleList></ArticleList>
                </div>
            </div>
        </div>
    }
}