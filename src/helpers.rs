// Helper struct for base HTML file elements
// All pages create a WebPage and add their content through it.
// WebPage implements IntoResponse so it gets converted into an HTML response

use axum::response::{Html, IntoResponse};
use maud::{html, Markup, Render, DOCTYPE};

use crate::blog::{content_tags_html, BlogID, BlogPostInfo};

#[derive(PartialEq)]
pub enum PageType {
    Home,
    Blog,
    Error,
}

pub struct WebPage {
    pub title: String,
    pub content: Markup,
    pub page_type: PageType,
}

impl WebPage {
    pub fn error_page<T: std::fmt::Debug>(error: T) -> Self {
        Self {
            title: "Error".into(),
            content: html! {
                h1 {
                    (format!("An error has occured: {:?}",error))
                }
            },
            page_type: PageType::Error,
        }
    }

    pub fn blog_page(blog_id: BlogID, info: &BlogPostInfo) -> Self {
        let post_html = match blog_id.load_html() {
            Ok(html) => html,
            Err(err) => return WebPage::error_page(err),
        };
        WebPage {
            title: info.title.clone(),
            content: html! {
                header { h1 class="large" {
                    (info.title)
                }
                h4 class="large" { (info.tagline) }
                div {
                    (content_tags_html(&info.tags))
                }
            }
                (post_html)
            },
            page_type: PageType::Blog,
        }
    }

    pub fn get_markup(self) -> Markup {
        html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta charset="UTF-8" {}
                    // meta name="viewport" content="width=device-width, initial-scale=1.0" {}
                    link rel="stylesheet" href="/assets/beercss/beer.min.css" {}
                    link rel="stylesheet" href="/assets/stylesheet.css" {}
                    title {
                        (self.title)
                    }
                }
                body class="light" {
                    header class="center-align fixed max" {
                        h1 class="large" {
                            a href="/" {
                                h2 #hey_im { "Hey! I'm " span .primary-text #first_name_animate  {"Jordi"} span .primary-text #last_name_animate {" Gulley"}
                                img .portrait src="/assets/portrait.jpg" .margin {} }
                            }
                        }
                        nav class="tabbed space margin-bottom" #navbar {
                            a href="/" .active[self.page_type == PageType::Home] {
                                i {"home"}
                                "Home"
                            }
                            a href="/blog" .active[self.page_type == PageType::Blog] {
                                i {"book"}
                                "Blog"
                            }
                        }
                        div .right .absolute {
                            a href="https://github.com/jordigulley/" .margin {
                                button .transparent .circle {
                                    img .responsive src="/assets/github-mark.svg" {
                                    }
                                }
                            }
                        }
                    }
                    main class="responsive padding" #content {(self.content) }
                }
                script type="module" src="/assets/beercss/beer.min.js" {}
                script type="module" src="/assets/beercss/material-dynamic-colors.min.js" {}
                script type="module" src="/assets/qs.js" {}
                script type="module" src="/assets/animations.js" {}
                script type="module" src="/assets/filter_tag_buttons.js" {}
            }
        }
    }
}

impl IntoResponse for WebPage {
    fn into_response(self) -> axum::response::Response {
        self.get_markup().into_response()
    }
}

pub fn remove_quotes(string: String) -> String {
    string[1..string.len() - 1].to_string()
}
