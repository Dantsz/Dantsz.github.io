use yew::prelude::*;


#[function_component(HomePage)]
pub fn home_page() -> Html
{
    let yorvs_load_script = r#"
    console.log("loading client");
    import init from '/resources/apps/yorvs_client/yorvs-871e8f5c219d5786.js';
    init('/resources/apps/yorvs_client/yorvs-871e8f5c219d5786_bg.wasm');
    "#;
    html!{
        <div class="grid gap-4 grid-cols-5 grid-rows-1">
            <div>
                <div class = "grid grid-cols-1 grid-rows-2">
                    <img src="resources/images/coa.png" alt="rust image"/>
                    <h1 class ="w-full text-lg text-slate-500">{"Home"}</h1>
                </div>
            </div>
            <div class = "col-span-4">
                <h2 class = {classes!("w-full","text-red-600", "first-letter:uppercase" ,"first-letter:text-7xl")}> {"About text"}</h2>
                <div class = "w-32">
                    <canvas id="notan_canvas"></canvas>
                    <script type="module">{yorvs_load_script}</script>
                </div>
            </div>
        </div>
    }
}