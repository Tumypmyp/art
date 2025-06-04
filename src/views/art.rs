use dioxus::prelude::*;
use crate::components::Gallery;

#[component]
pub fn Art() -> Element {
    
    let imgs: Vec<String> = vec![
        "https://cdn.masto.host/mastodonart/media_attachments/files/113/821/933/628/241/869/original/172627786241deca.png",
        "https://cdn.masto.host/mastodonart/media_attachments/files/113/843/645/071/204/798/original/b4464b9c7920c3a6.png",
        "https://lh3.googleusercontent.com/pw/AP1GczMl4BNc6SMS-7-WcrFHhfrFbx56-ROtHD3yb66tIxamDaPXXpm1251WozCBrR1L6M6OwWUOolPq1BEcusP7T2AaGTnETsTPp8BZbqkt0y8NsuBw4-gIgOUlWTz1lPFARr3GKw7NK597qIXnSAALgx6HyQ=w1368-h1710-s-no?authuser=0",
        "https://lh3.googleusercontent.com/pw/AP1GczNrTVWxhqa88TJ2RPbMdXQgMgG8vLfv0PM8gD8SbA08N0ubBeS8GJcbNUCef77BX5ycHF-S0JYO8PpI9LvZzihy2ejBoDf5-py93S3O_UrEKM94bowOxojRzuDZCwX0PIjdtxA2dHOba3CJUFVwQXqV0Q=w1283-h1710-s-no?authuser=0",
    ].into_iter()
    .map(|s| s.to_string())
    .collect();

    rsx! {
        Gallery{ imgs: imgs }
    }
}