use dioxus::prelude::*;
use crate::view::conf::Route;
use crate::server::get_time;

#[component]
pub fn Home() -> Element {
    let server_get_time = use_server_future(get_time)?.unwrap();
    rsx! {
        div {
            span { "{server_get_time.clone().unwrap()}" }
        }
        Outlet::<Route> {}
    }
}