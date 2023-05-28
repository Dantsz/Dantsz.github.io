use yew::prelude::*;

use crate::{components::post_list::PostList, theme::BLOG_STYLE};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class = {format!("{} {}","h-screen w-screen", BLOG_STYLE)} >
            <div class="grid gap-1 grid-cols-12 grid-rows-1">
                <div class = "col-span-2">
                    <div class = "w-full grid grid-cols-1 grid-rows-2">
                        <img class = "w-full rounded-b-lg" src="resources/images/coa.png" alt="rust image"/>
                    </div>
                </div>
                <div class = "col-span-8">
                      <PostList post_displayed_limits = 6 ></PostList>
                </div>
            </div>
        </div>
    }
}
