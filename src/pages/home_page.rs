use yew::prelude::*;

use crate::components::{article_list::ArticleList};


#[function_component(HomePage)]
pub fn home_page() -> Html
{
    let _yorvs_load_script = r#"
    import init from '/resources/apps/yorvs_client/yorvs_loader.js';
    init('/resources/apps/yorvs_client/yorvs.wasm');
    "#;
    html!{
        <div class = "h-screen w-screen bg-emerald-200 dark:bg-gray-900 dark:text-indigo-200">
            <div class="grid gap-1 grid-cols-12 grid-rows-1">
                <div class = "col-span-2">
                    <div class = "w-full grid grid-cols-1 grid-rows-2">
                        <img class = "w-full" src="resources/images/coa.png" alt="rust image"/>
                        <h1 class ="w-full text-lg text-slate-500">{"Home"}</h1>
                    </div>
                </div>
                <div class = "col-span-8">
                      <ArticleList></ArticleList>
                      /*<canvas id="notan_canvas"></canvas>
                      <script type="module">{yorvs_load_script}</script>*/
                </div>
            </div>
        </div>
    }
}