use dioxus::prelude::*;

const GALLERY_CSS: Asset = asset!("/assets/styling/gallery.css");

#[component]
pub fn Gallery(columns: usize, imgs: Vec<String>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: GALLERY_CSS }
        for j in (0..columns) {
            div { class: "gallery-column",
                for i in (j..imgs.len()).step_by(columns) {
                    div { class: "img-container",
                        img {
                            class: "photo",
                            key: i + "/" + j,
                            alt: "{imgs[i]}",
                            src: "{imgs[i]}",
                        }
                    }
                }
            }
        }
    }
}