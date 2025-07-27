use dioxus::prelude::*;
use crate::view::{
    conf::nav::NavBar,
    home::Home,
    top::{Top1, Top2, Ping, Local},
};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home,
    #[route("/top1")]
    Top1,
    #[route("/top2")]
    Top2,
    #[route("/ping")]
    Ping,
    #[route("/local")]
    Local,
}
