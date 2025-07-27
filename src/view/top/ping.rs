use dioxus::prelude::*;

#[component]
pub fn Ping() -> Element {
    rsx! {
        div { class: "relative overflow-x-auto",
            form { class: "max-w-xs mx-auto",
                label {
                    "for": "default-search",
                    class: "mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white",
                    "Search"
                }
                div { class: "relative h-10", // 固定高度确保垂直对齐
                    // 前缀文本容器 - 与输入框内容基线对齐
                    div { class: "absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none",
                        div { class: "text-sm text-gray-600 leading-relaxed", "192.168.1." }
                    }
                    // 输入框 - 统一字体大小和行高
                    input {
                        r#type: "search",
                        id: "default-search",
                        class: "block w-full h-full p-2 pl-20 pr-24 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        placeholder: "局域网段",
                        required: true,
                    }
                    // 按钮 - 与输入框垂直居中对齐
                    button {
                        r#type: "submit",
                        class: "absolute end-2 top-1/2 -translate-y-1/2 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-3 py-1 text-white dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                        "Ping"
                    }
                }
            }
        }
    }
}