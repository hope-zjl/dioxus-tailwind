use dioxus::prelude::*;
use crate::view::top::{Top1, Top2, Top3};
use crate::view::home::Home;
use super::nav::NavBar;

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