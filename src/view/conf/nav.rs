use dioxus::prelude::*;
use crate::view::conf::Route;

#[component]
pub fn NavBar() -> Element {
    let mut is_selected = use_signal(|| Route::Home);

    let default_class = "block py-2 px-3 text-gray-900 rounded-sm hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent";
    let selected_class = "block py-2 px-3 text-white bg-blue-700 rounded-sm md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500";

    rsx! {
        nav { class: "bg-white border-gray-200 dark:bg-gray-900",
            div { class: "max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4",
                div {
                    class: "hidden w-full md:block md:w-auto",
                    id: "navbar-default",
                    ul { class: "font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700",
                        li { class: if *is_selected.read() == Route::Home { selected_class } else { default_class },
                            Link {
                                to: Route::Home,
                                onclick: move |_| is_selected.set(Route::Home),
                                class: "font-bold",
                                "首页"
                            }
                        }
                        li { class: if *is_selected.read() == Route::Top1 { selected_class } else { default_class },
                            Link {
                                to: Route::Top1,
                                onclick: move |_| is_selected.set(Route::Top1),
                                class: "font-bold",
                                "Top1"
                            }
                        }
                        li { class: if *is_selected.read() == Route::Top2 { selected_class } else { default_class },
                            Link {
                                to: Route::Top2,
                                onclick: move |_| is_selected.set(Route::Top2),
                                class: "font-bold",
                                "Top2"
                            }
                        }
                        li { class: if *is_selected.read() == Route::Top3 { selected_class } else { default_class },
                            Link {
                                to: Route::Top3,
                                onclick: move |_| is_selected.set(Route::Top3),
                                class: "font-bold",
                                "Top3"
                            }
                        }
                    }
                }
                div { class: "flex items-center space-x-6 rtl:space-x-reverse", "关于" }
            }
        }
        Outlet::<Route> {}
    }
}