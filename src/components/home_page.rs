use comrak::{ComrakOptions, markdown_to_html};
use gloo_net::http::Request;
use yew::prelude::*;


#[function_component(HomePage)]
pub fn home_page() -> Html
{
    let yorvs_load_script = r#"
    console.log("loading client");
    import init from '/resources/apps/yorvs_client/yorvs-871e8f5c219d5786.js';
    init('/resources/apps/yorvs_client/yorvs-871e8f5c219d5786_bg.wasm');
    "#;
    
    let markdown_string = use_state(|| String::new());
    {
        let state = markdown_string.clone();
        use_effect_with_deps(|_|{
            wasm_bindgen_futures::spawn_local(async move {
                let request: String = Request::get("https://jaspervdj.be/lorem-markdownum/markdown.txt")
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
    let markdown =  Html::from_html_unchecked(AttrValue::from(markdown_to_html(&*markdown_string,&ComrakOptions::default())));
    html!{
        <div class="grid gap-4 grid-cols-5 grid-rows-1">
            <div>
                <div class = "grid grid-cols-1 grid-rows-2">
                    <img class = "w-full" src="resources/images/coa.png" alt="rust image"/>
                    <h1 class ="w-full text-lg text-slate-500">{"Home"}</h1>
                </div>
            </div>
            <div class = "col-span-4">
                <h2 class = {classes!("w-full","text-red-600", "first-letter:uppercase" ,"first-letter:text-7xl")}> {"About text"}</h2>
                <div className="prose prose-lg">
                    //<canvas id="notan_canvas"></canvas>
                    //<script type="module">{yorvs_load_script}</script>
                </div>
                <article class="prose prose-slate">
                    {markdown}
                </article>
            </div>
        </div>
    }
}