use dioxus::prelude::*;
use crate::{server::get_time, view::conf::{Route}};
use std::time::Duration;
use tokio::time::sleep;

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


#[component]
pub fn Home() -> Element {
    let data = use_resource(|| async {
        sleep(Duration::from_secs(5)).await;
        "hello  world"
    })
    .suspend()?;
    let server_get_time = use_server_future(get_time)?.unwrap();

    rsx! {
        //Loading {
        div {
            span { "{data.clone()}" }
            span { "{server_get_time.clone().unwrap()}" }
        }
        Outlet::<Route> {}
    }
}