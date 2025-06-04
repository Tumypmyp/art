use dioxus::prelude::*;
use crate::components::Gallery;

#[component]
pub fn Art() -> Element {
    
    let imgs: Vec<String> = vec![
        "https://cdn.masto.host/mastodonart/media_attachments/files/113/821/933/628/241/869/original/172627786241deca.png",
        "https://cdn.masto.host/mastodonart/media_attachments/files/113/843/645/071/204/798/original/b4464b9c7920c3a6.png",
    ].into_iter()
    .map(|s| s.to_string())
    .collect();

    rsx! {
        Gallery{ imgs: imgs }
    }
}