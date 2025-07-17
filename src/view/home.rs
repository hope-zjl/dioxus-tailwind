use dioxus::prelude::*;
use crate::view::conf::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { "Hello, 首页!" }
        Outlet::<Route> {}
    }
}