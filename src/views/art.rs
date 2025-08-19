use dioxus::prelude::*;
use crate::image;
use crate::components::Gallery;

const ARTS: [crate::components::image::Image; 3] = [
    image!("/assets/arts/1.jpg"),
    image!("/assets/arts/2.jpg"),
    image!("/assets/arts/3.png"),
];

#[component]
pub fn Art() -> Element {
    rsx! {
        Gallery { images: ARTS.to_vec(), image_click: true }
    }
}