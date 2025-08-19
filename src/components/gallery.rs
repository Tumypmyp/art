use dioxus::prelude::*;
use crate::{components::*, Route};

use dioxus_elements::geometry::euclid::Size2D;

use std::rc::Rc;

// use dioxus::html::geometry::euclid::Rect;

const GALLERY_CSS: Asset = asset!("/assets/styling/gallery.css");


#[derive(Props, PartialEq, Clone)]
pub struct GalleryProps {
    pub images: Vec<Image>,
}

#[component]
pub fn Gallery(props: GalleryProps) -> Element {
    let images = props.images;
// should get size of a screen
    let mut dimensions = use_signal(Size2D::zero);

    let n_columns = use_memo(move || if dimensions().width >= 600.0 { 3 } else { 2 });
    
    let mut div_element = use_signal(|| None as Option<Rc<MountedData>>);
    // let mut dims = use_signal(Rect::zero);

    // div_element.unwrap().get_client_rect()
    let mut visibility_signal = use_signal(|| "hidden");

    // let window = use_window().webview;
    rsx! {        
        document::Link { rel: "stylesheet", href: GALLERY_CSS }
        div {
            onmounted: move |evt| {
                div_element.set(Some(evt.data()));
                visibility_signal.set("visible");
            },
            // width: "100vw",
            // onmounted: move |evt| async move { 
                // dims.set(evt.data().get_client_rect().await.unwrap())
            // },
            // visibility: "visible",
            // visibility: "{visibility_signal():?}",
            // opacity: "1",
            onresize: move |evt| dimensions.set(evt.data().get_content_box_size().unwrap()),
            // "This element is {dimensions():?}, dims,"
            // div{ "div {div_element.unwrap().get_client_rect():#?}"}
            if visibility_signal() == "visible" {
                GalleryWithType{ columns: n_columns(), imgs: images, vis: visibility_signal() }
            }
        }
    }
}

#[component]
fn GalleryWithType(columns: usize, imgs: Vec<Image>, vis: ReadOnlySignal<String>) -> Element {
    rsx! {
        div { class: "gallery",
            padding: "0.5vw",
            visibility: "{vis():?}",
            for j in (0..columns) {
                div { class: "gallery-column",
                    width: "{100.0/(columns as f64):?}%",
                    for i in (j..imgs.len()).step_by(columns) {
                        ImageContainer{image: imgs[i].clone(), vis: vis}
                    }
                }
            }
        }
    }
}

#[component]
fn ImageContainer(image: Image, vis: ReadOnlySignal<String>) -> Element {
    let mut image_loaded = use_signal(|| false);
    rsx! {
        div { class: "img-container",
            if !*image_loaded.read() {
                // Loading rectangle
                div {
                    class: "absolute inset-0 bg-gray-300 animate-pulse",
                    style: "width: 250px; hight: 300px; aspect-ratio: 3 / 4;",
                    "Loading..."
                }
            },
            img {                
                class: "photo",
                padding: "0.5vw",
                opacity: image_loaded,
                key: i + "/" + j,
                alt: "{image.description:?}",
                src: "{image.asset}",
                onload: move |_| {
                    // When the image finishes loading, set the state to true
                    image_loaded.set(true);
                },
            }
        }
    }
}
