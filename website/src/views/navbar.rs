use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link {
                to: Route::Blog {
                    id: "index".to_string(),
                },
                "Blog"
            }
            Link { to: Route::Profile {}, "Profile" }
            Link { to: Route::Portfolio {}, "Portfolio" }
        }

        Outlet::<Route> {}

        // Bottom bar with licenses and attribution
        div {
            id: "bottom-bar",
            style: "text-align: center; margin-top: 2rem; font-size: 0.9rem; color: #888;",
            "I programmed this website's full stack using only Rust (Dioxus and Fly.io). "
            a {
                href: "https://github.com/Daniel-Abeleira/MyWebsite",
                target: "_blank",
                "Inspect the source code here"
            }
            br {}
            "Fonts used: Segoe UI, Tahoma, Geneva, Verdana"
        }
    }
}
