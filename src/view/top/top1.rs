use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Top1() -> Element {
    info!(">>> 进入 top1 >>>>>");
    rsx! {
        div {
            span { "Hello,top1!" }
        }
    }
}
