use dioxus::prelude::*;
use crate::view::conf::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "absolute top-10 left-20 right-20 bottom-0 bg-blue-100 p-4", "首页内容" }
        Outlet::<Route> {}
    }
}