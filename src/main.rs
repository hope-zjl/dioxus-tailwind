#![windows_subsystem = "windows"]
mod view;
mod server;

use dioxus::prelude::*;
use view::conf::Route;

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
