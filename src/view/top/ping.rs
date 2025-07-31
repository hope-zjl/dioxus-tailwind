use std::{net::IpAddr,time::Duration};
use dioxus::prelude::*;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};

#[component]
pub fn Ping() -> Element {
    let mut ping_value = use_signal(|| "192.168.1.9".to_string());
    let mut ping_msg = use_resource(move || async move {
        set_ping(ping_value.read().clone()).await
    });
    rsx! {
        div { class: "relative overflow-x-auto",
            table { class: "border w-2/5 text-sm text-center mx-auto rtl:text-right text-gray-500 dark:text-gray-400",
                tbody {
                    tr { class: "border bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            "网络测试"
                        }
                    }
                    tr { class: "border bg-white dark:bg-gray-800",
                        th {
                            scope: "row",
                            class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                            span { class: "font-bold text-xs me-2 text-blue-600", "{ping_msg.cloned().unwrap_or_default():?}" }
                            input {
                                r#type: "text",
                                id: "floating-phone-number",
                                class: "w-35 py-2.0 ps-3 pe-0 text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none focus:outline-none focus:ring-0 focus:border-blue-600 dark:text-white dark:border-gray-600 dark:focus:border-blue-500 peer",
                                placeholder: " ",
                                value: "{ping_value}",
                                oninput: move |ev| ping_value.set(ev.value().clone()),
                            }
                            button {
                                r#type: "button",
                                class: "ml-2 px-3 py-2 text-xs font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                                onclick: move |_| async move {
                                    ping_msg.restart();
                                    _ = set_ping(ping_value.read().clone()).await;
                                },
                                "测试"
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn set_ping(val :String) -> Duration {
    let clinet = Client::new(&Config::default()).unwrap();
    let mut ping = clinet.pinger(IpAddr::V4(val.parse().unwrap()), PingIdentifier(10)).await;
    let res = ping.ping(PingSequence(2), &[2; 8]).await;
    let (_, rtt) = res.unwrap();
    rtt
}