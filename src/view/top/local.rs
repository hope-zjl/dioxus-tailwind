use dioxus::prelude::*;
use sysinfo::{Networks, Disks, System, RefreshKind, CpuRefreshKind};

#[component]
pub fn Local() -> Element {
    let sys_name = sys_info::hostname().unwrap();
    let os_type = sys_info::os_type().unwrap();
    let networks = Networks::new_with_refreshed_list();
    let system_version = System::os_version().unwrap();

let s = System::new_with_specifics(
    RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
);
let cpu_name = s.cpus().get(0).unwrap();


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
                        td { class: "px-6 py-4", "{os_type} {system_version}" }
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
                        tr { class: "bg-white dark:bg-gray-800",
                            th {
                                scope: "row",
                                class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                                "可用空间"
                            }
                            td { class: "px-6 py-4", "{disk_space()}" }
                        }
                    }
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "CPU"
                        }
                        td { class: "px-6 py-4", "{cpu_name.brand()}" }
                    }
                }
            }
        }
    }
}

fn disk_space() -> String {
    let mut disk_size_available = 0;
    let mut disk_size_total = 0;
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        disk_size_available += disk.available_space();
        disk_size_total += disk.total_space();
    }
    format!("{:.2}%({})", (disk_size_available as f64 / disk_size_total as f64) * 100.0, format_bytes(disk_size_available))
}

fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit_index = 0;

    // 找到最合适的单位（直到值小于1024或达到最大单位）
    while value >= 1024.0 && unit_index < units.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    // 保留两位小数并拼接单位
    format!("{:.2} {}", value, units[unit_index])
}