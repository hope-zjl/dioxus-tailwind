use dioxus::prelude::*;
use crate::view::{
    conf::nav::NavBar,
    home::Home,
    top::{Top1, Top2, Top3},
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
    #[route("/top3")]
    Top3,
}
