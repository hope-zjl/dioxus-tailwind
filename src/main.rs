#![windows_subsystem = "windows"]
mod view;

use dioxus::prelude::*;
use view::conf::Route;

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
static DAISY_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: DAISY_CSS }
        Router::<Route> {}
    }
}
