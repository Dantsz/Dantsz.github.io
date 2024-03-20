use yew::prelude::*;

use crate::{components::post_list::PostList, theme::BLOG_STYLE};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class = {format!("{} {}","h-screen w-screen", BLOG_STYLE)} >
            <div class="grid grid-cols-12 grid-rows-1">
                <div class = "col-span-2 flex flex-col">
                    <div class = "w-full flex border-8 border-transparent flex-row justify-center">
                        <img class = "w-3/4 rounded-b-lg object-center" src="resources/images/coa.png" alt="rust image"/>
                    </div>
                </div>
                <div class = "col-span-8">
                      <PostList post_displayed_limits = 6 ></PostList>
                </div>
            </div>
        </div>
    }
}
