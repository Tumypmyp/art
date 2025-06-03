use dioxus::prelude::*;
use dioxus_elements::geometry::euclid::Size2D;

const GALLERY_CSS: Asset = asset!("/assets/styling/gallery.css");

#[component]
pub fn Gallery(imgs: Vec<String>) -> Element {
    
    let mut dimensions = use_signal(Size2D::zero);
    

    let mobile = use_memo(move || if dimensions().width < 600.0 {"flex"} else {"none"});
    let desktop = use_memo(move || if dimensions().width >= 600.0 {"flex"} else {"none"});
    rsx! {        
        document::Link { rel: "stylesheet", href: GALLERY_CSS }
        div { 
            onresize: move |evt| dimensions.set(evt.data().get_content_box_size().unwrap()),
            // "This element is {dimensions():?}, and mobile - {mobile():?}, desktop - {desktop():?}",            
            // div {"mobile"}

            div { class: "mobile-gallery",
                display: mobile(),
                GalleryWithType{ columns: 2, imgs: imgs.clone() }
            }
            // div {"desktop"}
            div { class: "desktop-gallery",
                display: desktop(),
                GalleryWithType{ columns: 3, imgs: imgs.clone() }
            }
        }
    }
}

#[component]
fn GalleryWithType(columns: usize, imgs: Vec<String>) -> Element {
    rsx! {
        for j in (0..columns) {
            div { class: "gallery-column",
                // width: 33.333%;
                // width: 50%;
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