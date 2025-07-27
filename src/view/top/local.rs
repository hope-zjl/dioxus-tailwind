use chrono::{DateTime, FixedOffset};
use dioxus::prelude::*;
use sysinfo::{CpuRefreshKind, Disks, Networks, RefreshKind, System};

#[component]
pub fn Local() -> Element {
    let networks = Networks::new_with_refreshed_list();
    let system_version = System::os_version().unwrap();
    let s =
        System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));
    let cpu_name = s.cpus().get(0).unwrap();

    rsx! {
        div { class: "relative overflow-x-auto",
            table { class: "w-2/5 text-sm text-center mx-auto rtl:text-right text-gray-500 dark:text-gray-400",
                tbody {
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "启动时间"
                        }
                        td { class: "px-6 py-4", "{get_boot_time()}" }
                    }
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "设备名称"
                        }
                        td { class: "px-6 py-4", "{System::host_name().unwrap()}" }
                    }
                    tr { class: "bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "系统名称"
                        }
                        td { class: "px-6 py-4", "{System::name().unwrap()} {system_version}" }
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
    format!(
        "{:.2}%({})",
        (disk_size_available as f64 / disk_size_total as f64) * 100.0,
        format_bytes(disk_size_available)
    )
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

fn time_utc_8() -> FixedOffset {
    FixedOffset::east_opt(8 * 60 * 60).unwrap()
}

fn get_boot_time() -> String {
    let boot_time = System::boot_time() as i64;
    let date_date_time = DateTime::from_timestamp(boot_time, 0).unwrap();
    date_date_time
        .with_timezone(&time_utc_8())
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
