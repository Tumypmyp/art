use dioxus::prelude::*;
use crate::image;
use crate::components::Gallery;

pub const PHOTOS: [crate::components::image::Image; 17] = [
    image!("/assets/photos/omir_conf_25/1.jpg", description: "The animal rights activists in Almaty talk."),
    image!("/assets/photos/omir_conf_25/2.jpg"),
    image!("/assets/photos/omir_conf_25/3.jpg", description: "A photo from the conference"),
    image!("/assets/photos/omir_conf_25/4.jpg"),
    image!("/assets/photos/omir_conf_25/5.jpg"),
    image!("/assets/photos/omir_conf_25/6.jpg"),
    image!("/assets/photos/omir_conf_25/7.jpg"),
    image!("/assets/photos/omir_conf_25/8.jpg"),
    image!("/assets/photos/omir_fest_24/1.jpg"),
    image!("/assets/photos/omir_fest_24/2.jpg"),
    image!("/assets/photos/omir_fest_24/3.jpg"),
    image!("/assets/photos/omir_fest_24/4.jpg"),
    image!("/assets/photos/1.jpg"),
    image!("/assets/photos/2.jpg"),
    image!("/assets/photos/3.jpg"),
    image!("/assets/photos/4.jpg"),
    image!("/assets/photos/5.jpg"),
];

#[component]
pub fn Photos() -> Element {
    rsx! {
        Gallery{  images: PHOTOS.to_vec() }
    }
}
