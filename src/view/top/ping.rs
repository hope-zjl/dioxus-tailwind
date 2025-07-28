use dioxus::prelude::*;

#[component]
pub fn Ping() -> Element {
    rsx! {
        div { class: "relative overflow-x-auto",
            table { class: "w-2/5 text-sm text-center mx-auto rtl:text-right text-gray-500 dark:text-gray-400",
                tbody {
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "局域网"
                        }
                    }
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            input {
                                r#type: "text",
                                id: "floating-phone-number",
                                class: "w-35 py-2.0 ps-3 pe-0 text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none focus:outline-none focus:ring-0 focus:border-blue-600 dark:text-white dark:border-gray-600 dark:focus:border-blue-500 peer",
                                placeholder: " ",
                            }
                            button {
                                r#type: "button",
                                class: "ml-2 px-3 py-2 text-xs font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                                "测试"
                            }
                        }
                    }
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "互联网"
                        }
                    }
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            input {
                                r#type: "text",
                                id: "floating-phone-number",
                                class: "w-35 py-2.0 ps-3 pe-0 text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none focus:outline-none focus:ring-0 focus:border-blue-600 dark:text-white dark:border-gray-600 dark:focus:border-blue-500 peer",
                                placeholder: " ",
                            }
                            button {
                                r#type: "button",
                                class: "ml-2 px-3 py-2 text-xs font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                                "测试"
                            }
                        }
                    }
                }
            }
        }
    }
}