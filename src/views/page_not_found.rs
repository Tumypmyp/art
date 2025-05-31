use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        // p { "We are terribly sorry, but the page you requested doesn't exist." }
        
        Link { to: Route::Home {}, "Go to Home" }
    }
}