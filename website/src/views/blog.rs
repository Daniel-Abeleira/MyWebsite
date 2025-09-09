use crate::Route;
use dioxus::prelude::*;
use serde::Deserialize;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[derive(Deserialize, Clone)]
pub struct BlogPost {
    pub title: String,
    pub content: String,
    pub publication_year: i32,
}

// List of blog post string ids and file paths
static BLOG_POSTS: &[(&str, &str)] = &[
    (
        "hello-world",
        "../../assets/blog_posts/blog_hellow_world.json",
    ),
    // Add more: ("another-id", "../../assets/blog_posts/blog_another.json"),
];

// Load all posts at startup, returning Vec<(id, BlogPost)>
fn load_all_posts() -> Vec<(String, BlogPost)> {
    BLOG_POSTS
        .iter()
        .filter_map(|(id, path)| {
            let json = match *id {
                "hello-world" => include_str!("../../assets/blog_posts/blog_hellow_world.json"),
                // Add more: "another-id" => include_str!("../../assets/blog_posts/blog_another.json"),
                _ => return None,
            };
            serde_json::from_str(json)
                .ok()
                .map(|post| (id.to_string(), post))
        })
        .collect()
}

// Get post by string id
fn get_post_by_id(id: &str) -> Option<BlogPost> {
    load_all_posts().into_iter().find_map(
        |(post_id, post)| {
            if post_id == id {
                Some(post)
            } else {
                None
            }
        },
    )
}

// Get all posts from the same year, excluding the current id
fn get_posts_by_year(year: i32, _exclude_id: &str) -> Vec<(String, BlogPost)> {
    load_all_posts()
        .into_iter()
        .filter(|(_post_id, post)| post.publication_year == year)
        .collect()
}

#[component]
pub fn Blog(id: String) -> Element {
    if id == "index" {
        let mut posts = load_all_posts();
        posts.sort_by(|a, b| {
            b.1.publication_year
                .cmp(&a.1.publication_year)
                .then_with(|| a.1.title.cmp(&b.1.title))
        });

        rsx! {
            link { rel: "stylesheet", href: "{BLOG_CSS}" }
            article {
                h1 { "All Blog Posts" }
                ul {
                    {
                        let items: Vec<_> = posts
                            .iter()
                            .map(|(post_id, post)| rsx! {
                                li {
                                    a { href: "/blog/{post_id}", "{post.title} ({post.publication_year})" }
                                }
                            })
                            .collect();
                        items.into_iter()
                    }
                }
            }
        }
    } else if let Some(post) = get_post_by_id(&id) {
        let other_posts = get_posts_by_year(post.publication_year, "");

        rsx! {
            link { rel: "stylesheet", href: "{BLOG_CSS}" }
            article {
                h1 { "{post.title}" }
                p { "Published: {post.publication_year}" }
                div { dangerous_inner_html: "{post.content}" }
            }
            hr {}
            section {
                h2 { "Posts from {post.publication_year}:" }
                ul {
                    {
                        let items: Vec<_> = other_posts
                            .iter()
                            .map(|(other_id, other_post)| rsx! {
                                li {
                                    a { href: "/blog/{other_id}", "{other_post.title}" }
                                }
                            })
                            .collect();
                        items.into_iter()
                    }
                }
            }
        }
    } else {
        rsx! {
            p { "Blog post not found." }
        }
    }
}
