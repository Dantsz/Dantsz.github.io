use yew::prelude::*;

#[function_component(AssetGen)]
pub fn asset_gen() -> Html {
    html! {
    <section class="min-h-screen p-6">
            <h1 class="text-3xl font-bold mb-6">{ "Asset Generator Preview" }</h1>

            // 1 column on mobile, 2 columns on md+
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 h-[80vh]">
            <div>
                <h1>{"Category: Chairs"}</h1>
                <iframe
                    title="Chairs"
                    src="/resources/generated_assets/chairs.html"
                    class="w-full h-full border rounded bg-white"
                />
            </div>
            <div>
                <h1>{"Category: Plants"}</h1>
                <iframe
                    title="Plants"
                    src="/resources/generated_assets/plants.html"
                    class="w-full h-full border rounded bg-white"
                />
            </div>
            </div>
        </section>
    }
}
