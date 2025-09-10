use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "hero-section",
            div { class: "hero-content",
                h1 { style: "text-align: center;", "Daniel Abeleira - C++/C#(Unity)/Rust Gameplay Programmer" }
                p { style: "text-align: justify;",
                    "Hi, I'm Daniel, welcome to my website. You can use the navigation bar at the top to explore my portfolio and profile. Feel free to reach out if you'd like to connect! My email is dani.abeleira@gmail.com."
                }
            }
        }
    }
}
