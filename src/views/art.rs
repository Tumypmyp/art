use dioxus::prelude::*;
use crate::components::Gallery;

#[component]
pub fn Art() -> Element {    
    let imgs: Vec<Asset> = vec![
        asset!("/assets/arts/1.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/arts/2.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/arts/3.png", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
    ];
    rsx! {
        Gallery{ imgs: imgs }
    }
}