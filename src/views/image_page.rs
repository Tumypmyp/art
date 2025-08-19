use dioxus::prelude::*;
use crate::ImageContainer;
use crate::PHOTOS;
use crate::GALLERY_CSS;


#[component]
pub fn ImagePage(id: usize) -> Element {
    let image = &PHOTOS[id];
    rsx! {
        document::Link { rel: "stylesheet", href: GALLERY_CSS }
        div { id: "image",
            ImageContainer { image: image.clone(), id: id }
        }
        div { id: "description",
            if let Some(desc) = image.description {
                p {
                    "{desc}"
                }
            }
        }
    }
}

