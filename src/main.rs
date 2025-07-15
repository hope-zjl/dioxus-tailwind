#![windows_subsystem = "windows"]
mod view;

use dioxus::prelude::*;
use view::conf::Route;

static CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}
