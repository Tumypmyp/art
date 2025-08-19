use dioxus::prelude::*;
use crate::Image;

#[component]
pub fn ImagePage(id: i32) -> Element {
    rsx! {

        div { id: "image",
            // img {                
            //     class: "photo",
            //     padding: "0.5vw",
            //     key: i + "/" + j,
            //     alt: "{image.description:?}",
            //     src: "{image.asset}",
            // }
        }
    }
}

