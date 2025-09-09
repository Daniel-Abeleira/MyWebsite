use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    rsx! {
        div {
            h2 { "Profile" }
            p { "This is a sample profile." }
        }
    }
}
