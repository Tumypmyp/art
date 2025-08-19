use dioxus::prelude::*;
use crate::{components::*, Route};

use dioxus_elements::geometry::euclid::Size2D;


pub const GALLERY_CSS: Asset = asset!("/assets/styling/gallery.css");

#[derive(Props, PartialEq, Clone)]
pub struct GalleryProps {
    pub images: Vec<Image>,
    pub image_click: bool,
}

#[component]
pub fn Gallery(props: GalleryProps) -> Element {
    let images = props.images.clone();
    let images_len = images.len();

    let mut dimensions = use_signal(Size2D::zero);

    let n_columns = use_memo(move || 
        if images_len < 4 { 
            images_len 
        } else {
            if dimensions().width >= 600.0 { 3 } else { 2 }
        });
    
    rsx! {
        document::Link { rel: "stylesheet", href: GALLERY_CSS }
        div { onresize: move |evt| dimensions.set(evt.data().get_content_box_size().unwrap()),
            GalleryWithType { columns: n_columns(), props }
        }
    }
}


#[component]
fn GalleryWithType(columns: usize, props: GalleryProps) -> Element {
    rsx! {
        div { class: "gallery", padding: "0.5vw",
            for j in (0..columns) {
                div {
                    class: "gallery-column",
                    width: "{100.0/(columns as f64):?}%",
                    for i in (j..props.images.len()).step_by(columns) {
                        ImageContainer {
                            image: props.images[i].clone(),
                            id: i,
                            image_click: props.image_click,
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ImageContainer(image: Image, id: usize, image_click: bool) -> Element {
    // let mut image_loaded = use_signal(|| false);
    let nav = navigator();
    rsx! {
        div { class: "img-container",
            // if !*image_loaded.read() {
            //     // Loading rectangle
            //     // div {
            //     //     class: "absolute inset-0 bg-gray-300 animate-pulse",
            //     //     style: "width: 250px; hight: 300px; aspect-ratio: 3 / 4;",
            //     //     "Loading..."
            //     }
            // }
            img {
                class: "photo",
                padding: "0.5vw",
                // opacity: image_loaded,
                alt: "{image.description:?}",
                src: "{image.asset}",
                // onload: move |_| {
                //     // When the image finishes loading, set the state to true
                //     image_loaded.set(true);
                // },
                onclick: move |_| {
                    if image_click {
                        nav.push(Route::ImagePage { id: id });
                    }
                },
            }
        }
    }
}
