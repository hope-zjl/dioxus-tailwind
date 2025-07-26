#![windows_subsystem = "windows"]
mod server;
mod view;

use dioxus::prelude::*;
use view::conf::Route;

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // dioxus::launch(app);
    #[cfg(feature = "desktop")]
    fn launch_app() {
        use dioxus::desktop::tao;
        let window = tao::window::WindowBuilder::new().with_resizable(true);
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::new()
                    .with_window(window)
                    .with_menu(None),
            )
            .launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    fn launch_app() {
        dioxus::launch(App);
    }

    launch_app();
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
