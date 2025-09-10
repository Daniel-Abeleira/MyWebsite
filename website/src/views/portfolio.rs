use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PortfolioProject {
    FantasyWorldGenerator,
    FPSPlayground,
    SoftwareRaytracer,
    DirectXRenderer,
    AgainstTheDarkness,
    ZombieGame,
}

struct ProjectInfo {
    key: PortfolioProject,
    title: &'static str,
    description: &'static str,
    image_url: Option<Asset>, // Use Asset type for images
}

const PROJECTS: &[ProjectInfo] = &[
    ProjectInfo {
        key: PortfolioProject::AgainstTheDarkness,
        title: "Against The Darkness",
        description: "A couch-coop game made with a team of fellow students in Unity.",
        image_url: Some(asset!("/assets/images/ATD.png")),
    },
    ProjectInfo {
        key: PortfolioProject::SoftwareRaytracer,
        title: "Software Raytracer",
        description: "A pure C++ 3D raytracer.",
        image_url: Some(asset!("/assets/images/RT.png")),
    },
    ProjectInfo {
        key: PortfolioProject::DirectXRenderer,
        title: "DirectX Renderer",
        description: "A renderer built on DirectX with C++.",
        image_url: Some(asset!("/assets/images/DX.png")),
    },
    ProjectInfo {
        key: PortfolioProject::ZombieGame,
        title: "Zombie Game",
        description: "An autonomous agent for a zombie survival game, written in C++.",
        image_url: Some(asset!("/assets/images/ZombieGame.png")),
    },
    ProjectInfo {
        key: PortfolioProject::FantasyWorldGenerator,
        title: "Fantasy World Generator",
        description: "A procedural world generator with history simulation made in Unity.",
        image_url: Some(asset!("/assets/images/FWHG.png")),
    },
    ProjectInfo {
        key: PortfolioProject::FPSPlayground,
        title: "FPS Playground",
        description:
            "A level editor with autonomous bots made for FPS or TPS level designers in Unity.",
        image_url: Some(asset!("/assets/images/FPSPG.png")),
    },
];

fn project_detail(project: &PortfolioProject) -> Element {
    match project {
        PortfolioProject::FantasyWorldGenerator => rsx!(
            p { "This is a sample Fantasy World Generator project." }
        ),
        PortfolioProject::FPSPlayground => rsx!(
            p { "This is a sample FPS Playground project." }
        ),
        PortfolioProject::SoftwareRaytracer => rsx!(
            p { "This is a sample Software Rasterizer project." }
        ),
        PortfolioProject::DirectXRenderer => rsx!(
            p { "This is a sample DirectX Renderer project." }
        ),
        PortfolioProject::AgainstTheDarkness => rsx!(
            div {
                style: "display: flex; flex-direction: column; align-items: center; justify-content: center;",
                div {
                    dangerous_inner_html: r#"
                        <iframe
                            frameborder='0'
                            src='https://itch.io/embed/2543310?bg_color=000000&amp;fg_color=e8e7e3&amp;link_color=fb922b&amp;border_color=e42832'
                            width='552'
                            height='167'
                        >
                            <a href='https://delthor-games.itch.io/game-project-group-2'>
                                Against the Darkness by Delthor Games, MariusBrill, ThisIsLiam, drakkar2guerre
                            </a>
                        </iframe>
                    "#,
                }
                p { "This is a sample Against The Darkness project." }
            }
        ),
        PortfolioProject::ZombieGame => rsx!(
            p { "This is a sample Zombie Game project." }
        ),
    }
}

#[component]
pub fn Portfolio() -> Element {
    let mut selected_project = use_signal(|| None as Option<PortfolioProject>);
    rsx! {
        div {
            h2 { "Portfolio" }
            div { style: "margin-top: 2rem;",
                match *selected_project.read() {
                    None => rsx! {
                        div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                            {
                                PROJECTS
                                    .iter()
                                    .map(|project| {
                                        let show_details = move |_| {
                                            selected_project.set(Some(project.key.clone()))
                                        };
                                        rsx! {
                                            div { style: "display: flex; align-items: stretch; background: #18181b; border-radius: 1rem; box-shadow: 0 2px 8px #0002; overflow: hidden; margin-bottom: 1.5rem;",
                                                div { style: "position: relative; width: 400px; min-width: 220px; height: 200px; overflow: hidden; border-bottom-left-radius: 1rem; border-top-left-radius: 1rem; background: #222; display: flex; align-items: flex-end; justify-content: flex-start;",
                                                    if let Some(url) = &project.image_url {
                                                        img {
                                                            src: "{url}",
                                                            style: "width: 100%; height: 100%; object-fit: cover; object-position: center; display: block;",
                                                            alt: "{project.title}",
                                                        }
                                                    }
                                                    div { style: "position: absolute; left: 0; bottom: 0; width: 100%; background: linear-gradient(90deg, #222d 60%, #3C515C 100%); color: #fff; font-size: 1.5rem; font-weight: bold; padding: 0.5rem 1rem; text-shadow: 0 2px 8px #000a; border-bottom-left-radius: 1rem;",
                                                        "{project.title}"
                                                    }
                                                }
                                                div { style: "flex: 1; display: flex; flex-direction: column; justify-content: center; padding: 1.5rem; border-bottom-right-radius: 1rem; border-top-right-radius: 1rem; background: transparent;",
                                                    p { style: "margin: 0 0 1rem 0; color: #e5e5e5; font-size: 1.1rem;", "{project.description}" }
                                                    button {
                                                        style: "align-self: flex-start; padding: 0.5rem 1.2rem; font-size: 1rem; border-radius: 0.5rem; background: #3C515C; color: white; border: none; cursor: pointer; margin-top: 0.5rem;",
                                                        onclick: show_details,
                                                        "See details"
                                                    }
                                                }
                                            }
                                        }
                                    })
                            }
                        }
                    },
                    Some(ref project) => rsx! {
                        {project_detail(project)}
                        div { style: "margin-top: 1.5rem;",
                            button {
                                style: "padding: 0.5rem 1.2rem; font-size: 1rem; border-radius: 0.5rem; background: #3C515C; color: white; border: none; cursor: pointer;",
                                onclick: move |_| selected_project.set(None),
                                "Back to projects"
                            }
                        }
                    },
                }
            }
        }
    }
}
