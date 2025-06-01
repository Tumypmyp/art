
// use std::rc::Rc;
use crate::components::Gallery;

use dioxus::prelude::*;
// use dioxus::{html::geometry::euclid::Rect, prelude::*};


#[component]
pub fn Photos() -> Element {

    let imgs: Vec<String> = vec![
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
        "https://lh3.googleusercontent.com/pw/AP1GczP6K8TACdgK45x9CDaOretTg0bORmZ1uRqpQYkHAmmgC1u2dABDZ6WlyN4bqhPxIL34zrGky7_eNJFi_Iq0EDkESvLT1PrIzp2zM4gzmoU8DjcD2KkhJ5GXF4FyEyw220BiWUsEPv98fWmZSzdCD9FU=w1179-h1572-s-no?authuser=0",
        "https://lh3.googleusercontent.com/pw/AP1GczNrVmTzxvVKD3-MA7FmTx57bEDyvKcEqEeUXJmA4McITEX3Een3NSEuWu2l8vfYEyv6x0LKfXgh4vONWsbOkCbQITL_sycreTFYAUpNTrZ2l0OkMzIW8yiGURsvdhElo2DgMUDQcJ7tguzMfDNwrhxH=w1122-h1572-s-no?authuser=0",
        "https://lh3.googleusercontent.com/pw/AP1GczN8Ekh-lKVddk8ueMgy35MyPx6oGFZozveS-4T2wAfTkCGjJV8DmGMIlPJ80gjD8YYQe0XXMFPAJ5EyzHfMLu8J7c52wy5Snp7Xo8_fG2ob35v5rDM9eqq-Ht4Ds20OBe_PEPwWYPGFnvPR6ZWvjhMv=w1122-h1572-s-no?authuser=0",
    ].into_iter()
    .map(|s| s.to_string())
    .collect();
    
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
        // button {
        //     onclick: read_size,
        //     "Gallery size is {gallery_size():?}",
        // }
        div { class: "mobile-gallery",
            Gallery{ columns: 2, imgs: imgs.clone() }
        }
        div { class: "desktop-gallery",
            Gallery{ columns: 3, imgs: imgs.clone() }
        }
    }
}
