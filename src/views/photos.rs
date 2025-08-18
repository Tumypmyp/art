use dioxus::prelude::*;
use crate::components::Gallery;

#[component]
pub fn Photos() -> Element {
    let imgs: Vec<Asset> = vec![
        asset!("/assets/photos/omir_conf_25/1.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_conf_25/2.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_conf_25/3.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_conf_25/4.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_conf_25/5.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_conf_25/6.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_conf_25/7.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_conf_25/8.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_fest_24/1.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_fest_24/2.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_fest_24/3.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/omir_fest_24/4.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/1.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/2.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/3.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/4.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
        asset!("/assets/photos/5.jpg", ImageAssetOptions::new().with_format(ImageFormat::Avif)),
    ];
    rsx! {
        Gallery{ imgs: imgs }
    }
}
