use dioxus::prelude::*;
use crate::components::Gallery;

#[component]
pub fn Art() -> Element {    
    let imgs: Vec<Asset> = vec![
        asset!("/assets/arts/1.jpg"),
        asset!("/assets/arts/2.jpg"),
        asset!("/assets/arts/3.png"),
    ];
    rsx! {
        Gallery{ imgs: imgs }
    }
}