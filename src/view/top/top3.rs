use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Top3() -> Element {
    info!(">>> 进入 top3 >>>>>");
    rsx! {
        div { "TOP3" }
    }
}