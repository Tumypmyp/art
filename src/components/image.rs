use dioxus::prelude::*;


#[derive(Clone, PartialEq, Props, Debug)]
pub struct Image {
    pub asset: Asset,
    pub description: Option<&'static str>,
    pub tags: Option<Vec<&'static str>>,
}

#[macro_export]
macro_rules! image {
    ($path:expr) => {
        $crate::components::image::Image {
            asset: dioxus::prelude::asset!($path, dioxus::prelude::ImageAssetOptions::new().with_format(dioxus::prelude::ImageFormat::Avif)),
            description: None,
            tags: None,
        }
    };
    ($path:expr, description: $desc:expr) => {
        $crate::components::image::Image {
            asset: dioxus::prelude::asset!($path, dioxus::prelude::ImageAssetOptions::new().with_format(dioxus::prelude::ImageFormat::Avif)),
            description: Some($desc),
            tags: None,
        }
    };
}
