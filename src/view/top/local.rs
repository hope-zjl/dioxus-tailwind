use dioxus::prelude::*;
use sysinfo::Networks;

#[component]
pub fn Local() -> Element {
    let sys_name = sys_info::hostname().unwrap();
    let os_type = sys_info::os_type().unwrap();
    let os_release = sys_info::os_release().unwrap();
    let networks = Networks::new_with_refreshed_list();
    rsx! {
        div { class: "relative overflow-x-auto",
            table { class: "w-2/5 text-sm text-center mx-auto rtl:text-right text-gray-500 dark:text-gray-400",
                tbody {
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "系统名称"
                        }
                        td { class: "px-6 py-4", "{sys_name}" }
                    }
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "系统类型"
                        }
                        td { class: "px-6 py-4", "{os_type}" }
                    }
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "系统版本"
                        }
                        td { class: "px-6 py-4", "{os_release}" }
                    }
                    for (_interface_name , network) in &networks {
                        tr { class: "bg-white dark:bg-gray-800",
                            th {
                                scope: "row",
                                class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                                "MAC地址"
                            }
                            td { class: "px-6 py-4", "{network.mac_address()}" }
                        }
                        for net in network.ip_networks() {
                            if net.addr.is_ipv4() {
                                tr { class: "bg-white dark:bg-gray-800",
                                    th {
                                        scope: "row",
                                        class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                                        "IPV4地址"
                                    }
                                    td { class: "px-6 py-4", "{net.addr}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}