use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
	
	rsx! {
		div { id: "hero",
			h1 { "Hi, my name is Timur" }
			a {
				"I am an artist and made this website!
				I love to learn new stuff. 
				This is my space to show the things I made.
				"
			}
			h1 { "My links" }
			// img { src: HEADER_SVG, id: "header" }
			

			div { id: "links",    
				a {
					href: "https://mastodon.art/@Pandenko",
					target: "_blank",
					rel: "noopener noreferrer",
					// img {
					//     src: "https://img.icons8.com/external-tal-revivo-color-tal-revivo/96/external-mastodon-is-an-online-self-hosted-social-media-and-social-networking-service-logo-color-tal-revivo.png",
					//     alt: "mastodon",
					// }
					"Mastodon"
				}
				a {
					href: "https://www.youtube.com/@pandenko",
					target: "_blank",
					rel: "noopener noreferrer",
					// img {
					//     src: "https://img.icons8.com/color/96/youtube-play.png",
					//     alt: "youtube-play",
					// }
					"Youtube"
				}
				a {
					href: "https://www.instagram.com/valentin_pandenko/",
					target: "_blank",
					rel: "noopener noreferrer",
					// img {
					//     src: "https://img.icons8.com/ios/100/instagram-new--v1.png",
					//     alt: "instagram",
					// }
					"Instagram"
				}
			}
		}
	}
}
