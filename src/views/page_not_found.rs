use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        Link { to: Route::Home {}, "Go to Home" }
    }
}