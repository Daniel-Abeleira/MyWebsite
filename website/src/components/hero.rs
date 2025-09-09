use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "hero-section",
            div { class: "hero-content",
                h1 { style: "text-align: center;", "Welcome to Daniel Abeleira's Website!" }
                p { style: "text-align: justify;",
                    "Hi, I'm Daniel—a passionate programmer who loves building creative solutions with Rust and web technologies. Explore my projects, read my blog, or get in touch!"
                }
            }
        }
    }
}
