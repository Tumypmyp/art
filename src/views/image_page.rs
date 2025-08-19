use dioxus::prelude::*;
use crate::components::Gallery;
use crate::PHOTOS;


#[component]
pub fn ImagePage(id: usize) -> Element {
    let image = &PHOTOS[id];
    rsx! {
        Gallery { images: vec![image.clone()], image_click: false }
        div { id: "description",
            if let Some(desc) = image.description {
                p { "{desc}" }
            }
        }
    }
}

