use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Top2() -> Element {
    info!(">>> 进入 top2 >>>>>");
    rsx! {
        div { "TOP2" }
    }
}