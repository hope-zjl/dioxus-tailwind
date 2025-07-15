use dioxus::prelude::*;
use crate::view::conf::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "p-2 h-10 bg-sky-100",
            Link { to: Route::Home, class: "pr-4 cursor-pointer not-italic", "首页" }
            Link { to: Route::Top1, class: "pr-4 cursor-pointer not-italic", "Top1" }
            Link { to: Route::Top2, class: "pr-4 cursor-pointer not-italic", "Top2" }
            Link { to: Route::Top3, class: "pr-4 cursor-pointer not-italic", "Top3" }
        }
        Outlet::<Route> {}
    }
}