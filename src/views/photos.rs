use dioxus::prelude::*;
use crate::components::Gallery;

#[component]
pub fn Photos() -> Element {
    let imgs: Vec<Asset> = vec![
        asset!("/assets/photos/omir_conf_25/1.jpg"),
        asset!("/assets/photos/omir_conf_25/2.jpg"),
        asset!("/assets/photos/omir_conf_25/3.jpg"),
        asset!("/assets/photos/omir_conf_25/4.jpg"),
        asset!("/assets/photos/omir_conf_25/5.jpg"),
        asset!("/assets/photos/omir_conf_25/6.jpg"),
        asset!("/assets/photos/omir_conf_25/7.jpg"),
        asset!("/assets/photos/omir_fest_24/1.jpg"),
        asset!("/assets/photos/omir_fest_24/2.jpg"),
        asset!("/assets/photos/img_1.jpg"),
        asset!("/assets/photos/img_2.jpg"),
        asset!("/assets/photos/img_3.jpg"),
        asset!("/assets/photos/img_4.jpg"),
    ];
    rsx! {
        Gallery{ imgs: imgs }
    }
}
