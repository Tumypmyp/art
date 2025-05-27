
use dioxus::prelude::*;

const PHOTOS_CSS: Asset = asset!("/assets/styling/photos.css");

const imgs: [&[&str]; 2] = [
    &[
        "https://cdn.masto.host/mastodonart/media_attachments/files/113/821/933/628/241/869/original/172627786241deca.png",
        "https://cdn.masto.host/mastodonart/media_attachments/files/113/843/645/071/204/798/original/b4464b9c7920c3a6.png",
        // "https://cdn.masto.host/mastodonart/media_attachments/files/114/558/318/682/946/333/original/6579d896b7981fc1.jpg",
        // "https://cdn.masto.host/mastodonart/media_attachments/files/114/558/318/682/946/333/original/6579d896b7981fc1.jpg",
    ],
    &[
        // "https://cdn.masto.host/mastodonart/media_attachments/files/114/558/318/663/048/055/original/a0aedfc2fb9025d3.jpg",
        // "sdfsdf",
    ],
];

#[component]
pub fn Art() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PHOTOS_CSS }
 
        div {
            class: "gallery",
            // Content
            // h1 { "People photos." }

            for (i, row) in imgs.iter().enumerate() {
                div {
                    class: "gallery-row",    
                    for (j, id) in row.iter().enumerate() {
                        div { 
                            class: "img-container",
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