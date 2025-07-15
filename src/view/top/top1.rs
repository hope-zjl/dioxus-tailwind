use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Top1() -> Element {
    info!(">>> 进入 top1 >>>>>");
    rsx! {
        div { class: "absolute top-10 left-20 right-20 bottom-0 bg-blue-100 p-4", "Hello Top1" }
    }
}
