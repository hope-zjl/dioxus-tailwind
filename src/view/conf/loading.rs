use dioxus::prelude::*;

#[component]
pub fn Loading(children: Element) -> Element {
    rsx! {
        SuspenseBoundary {
            // 加载状态显示
            fallback: |_| rsx! {
                div { class: "text-center",
                    span { class: "loading loading-dots loading-lg" }
                }
            },
            {children}
        }
    }
}
