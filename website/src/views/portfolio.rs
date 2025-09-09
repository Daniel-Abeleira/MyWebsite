use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PortfolioProject {
    Website,
    MobileApp,
    DataScience,
    // Add more projects here
}

#[component]
pub fn Portfolio() -> Element {
    let mut selected_project = use_signal(|| None as Option<PortfolioProject>);

    rsx! {
        div {
            h2 { "Portfolio" }
            div {
                button { onclick: move |_| selected_project.set(Some(PortfolioProject::Website)),
                    "Website"
                }
                button { onclick: move |_| selected_project.set(Some(PortfolioProject::MobileApp)),
                    "Mobile App"
                }
                button { onclick: move |_| selected_project.set(Some(PortfolioProject::DataScience)),
                    "Data Science"
                }
            }
            div {
                match *selected_project.read() {
                    Some(PortfolioProject::Website) => rsx! {
                        p { "This is a sample Website project." }
                    },
                    Some(PortfolioProject::MobileApp) => rsx! {
                        p { "This is a sample Mobile App project." }
                    },
                    Some(PortfolioProject::DataScience) => rsx! {
                        p { "This is a sample Data Science project." }
                    },
                    None => rsx! {
                        p { "Select a project to see details." }
                    },
                }
            }
        }
    }
}
