use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Top3() -> Element {
    info!(">>> 进入 top3 >>>>>");
    rsx! {
        div { class: "absolute top-10 left-20 right-20 bottom-0 p-4 bg-blue-100", "TOP3" }
    }
}