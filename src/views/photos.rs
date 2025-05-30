
use dioxus::prelude::*;

const PHOTOS_CSS: Asset = asset!("/assets/styling/photos.css");

const imgs: [&[&str]; 2] = [
    &[
        "https://cdn.masto.host/mastodonart/media_attachments/files/114/558/318/676/472/774/original/ed3a2ea9a3e73a3e.jpg",
        "https://cdn.masto.host/mastodonart/media_attachments/files/114/558/318/682/946/333/original/6579d896b7981fc1.jpg",
        // "https://photos.app.goo.gl/a9g5d2phTfqnST3w5=d",
    ],
    &[
        "https://cdn.masto.host/mastodonart/media_attachments/files/114/558/318/663/048/055/original/a0aedfc2fb9025d3.jpg",
        "https://lh3.googleusercontent.com/pw/AP1GczOC5Ls1cuhhx86QFU3hK1_UzXORCUOWSGc8KQFQFGzgIRLj8h8_DWi323_L1MERrbeyMjQ-Q41Sdycx7RVMVYoY1ZRgETD-WmC0hi2YSxHil6YACnpnsrZ1RHWND-NkfpK6f5gNmHghA-PnHZTGc7Eh=w1212-h1819-s-no?authuser=0&pageId=102042004995913367149",
        // "sdfsdf",
    ],
    
];
#[component]
pub fn Photos() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PHOTOS_CSS }

        div { class: "gallery",
            // Content
            // h1 { "People photos." }

            for (i , row) in imgs.iter().enumerate() {
                div { class: "gallery-row",
                    for (j , id) in row.iter().enumerate() {
                        div { class: "img-container",
                            img {
                                class: "photo",
                                key: "{i}/{j}",
                                alt: "{id}",
                                src: "{id}",
                            }
                        }
                    }
                }
            }
        }
    }
}
