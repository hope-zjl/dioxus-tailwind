use crate::{server::get_time, view::conf::Route};
use dioxus::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[component]
pub fn Home() -> Element {
    rsx! {
        SuspenseBoundary {
            // 加载状态显示
            fallback: |_| rsx! {
                div {
                    class: "text-center",
                    span { class: "loading loading-dots loading-lg" }
                }
            },
            HomeHello {}
        }
    }
}

#[component]
pub fn HomeHello() -> Element {
    let data = use_resource(|| async {
        sleep(Duration::from_secs(5)).await;
        "hello  world"
    })
    .suspend()?;
    let server_get_time = use_server_future(get_time)?.unwrap();

    rsx! {
        div {
            span { "{data.clone()}" }
            span { "{server_get_time.clone().unwrap()}" }
        }
        Outlet::<Route> {}
    }
}
