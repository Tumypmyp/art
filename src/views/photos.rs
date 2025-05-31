
use std::rc::Rc;

use dioxus::{html::geometry::euclid::Rect, prelude::*};

const PHOTOS_CSS: Asset = asset!("/assets/styling/photos.css");

const IMGS: &[&str] = &[
    "https://cdn.masto.host/mastodonart/media_attachments/files/114/558/318/676/472/774/original/ed3a2ea9a3e73a3e.jpg",
    "https://cdn.masto.host/mastodonart/media_attachments/files/114/558/318/682/946/333/original/6579d896b7981fc1.jpg",
    "https://lh3.googleusercontent.com/pw/AP1GczPhQGSxITejwseJKhXvlWd7omfaEvYS-oTB6-9HDFtHWsacke9TUPqBzFyv7p6VvbNB3X25pjAcaurErtZmFoJT2tok56szqhtZmPJIC87AAEIvqfe6VYt0NrDFlud9xW62htGN2q0kHyfnfrTFlyFH=w2275-h1819-s-no?authuser=0",
    "https://cdn.masto.host/mastodonart/media_attachments/files/114/558/318/663/048/055/original/a0aedfc2fb9025d3.jpg",
    "https://lh3.googleusercontent.com/pw/AP1GczOC5Ls1cuhhx86QFU3hK1_UzXORCUOWSGc8KQFQFGzgIRLj8h8_DWi323_L1MERrbeyMjQ-Q41Sdycx7RVMVYoY1ZRgETD-WmC0hi2YSxHil6YACnpnsrZ1RHWND-NkfpK6f5gNmHghA-PnHZTGc7Eh=w1212-h1819-s-no?authuser=0&pageId=102042004995913367149",
    "https://lh3.googleusercontent.com/pw/AP1GczMmtbiRsz1oRwQ8eZLfPvTRaKW1QJZbshLhSzdGDe7Br6tIFTFkFSxy70ur5jYWDxDvZ_14i6lDKBoo9wWXjARcnmN57gzZ28lyL5SU-hp8w0Uvvs0SrdZFZDwOW0VmpReidycJBbJ5VZ1zbivEX77g=w1456-h1819-s-no?authuser=0",
    "https://lh3.googleusercontent.com/pw/AP1GczOjZ9byQnndvFR9gQzr0hvuqlqo6W1PH75mYkQMEuJEmrErhuAN1RwkuGQO1P5_7-TOl5nEmWmsfRDF8qXb84dhrSne5n7vEWoZmhtjpK6_7Pl3WUMLapoHreT2trdsqOj9v-7PQ_3m4_rd3pxgyUK4=w2619-h1636-s-no?authuser=0",
    "https://lh3.googleusercontent.com/pw/AP1GczPJdNodmSQjf8WF6mzhL_SWNacHvoyd_3THwRt0ly8XQd8ug5M_dCwDlylwv2w4L76TCdLrODqXfaeETtjcG51m8LYORXqp9qyz6J30rTloCbA-3lCkOx-ui7B2N0Yhu9w5jeUBJbIfqjaiCOuLoVHE=w1364-h1819-s-no?authuser=0",
    "https://lh3.googleusercontent.com/pw/AP1GczN3tUrlgZipjjdbSaAqYY6yJgXkn3jG0wr9W3Y9m12QDEsUKKsO6HH8qAzg3eWllqc-Xe2D9e7rXM9VSx5VMQsBEpIH7KCvaqvKtfbr88uozT2DXqE74jtZKj9ZhEoyFdU70jcNSLrNosHvOtvOnCwc=w1671-h1763-s-no?authuser=0",
    "https://lh3.googleusercontent.com/pw/AP1GczMdYH5Ry0vuZjfyUIp4UDBEkPQcRm_BgaeuFzmSUZI6vtMONdX7v8v06WL6K6vS0F1Dcmvr2cmkBJPYuZ5-cZNt28i24RlSrY_YjnSUs2DRVy2nm1znHferTjM0uXQfYAbGuNAqeD3WTkDy0_TK3npB=w1364-h1819-s-no?authuser=0",
    ];

#[component]
pub fn Photos() -> Element {
    // let mut gallery_ref = use_signal(|| None as Option<Rc<MountedData>>);
    // let mut gallery_size = use_signal(|| Rect::zero());

    
    // let read_size = move |_| async move {
    //     let read = gallery_ref.read();
    //     let client_rect = read.as_ref().map(|el| el.get_client_rect());

    //     if let Some(client_rect) = client_rect {
    //         if let Ok(rect) = client_rect.await {
    //             gallery_size.set(rect);
    //         }
    //     }
    // };
    
    rsx! {
        document::Link { rel: "stylesheet", href: PHOTOS_CSS }
        // button {
        //     onclick: read_size,
        //     "Gallery size is {gallery_size():?}",
        // }
        div { class: "gallery",
            // Content
            // h1 { "People photos." }
            // onmounted: move |cx| gallery_ref.set(Some(cx.data())),
            for (i , id) in IMGS.iter().enumerate() {
                div { class: "img-container",
                    img {
                        class: "photo",
                        key: i,
                        alt: "{id}",
                        src: "{id}",
                    }
                }
            }
        }    
    }
}
