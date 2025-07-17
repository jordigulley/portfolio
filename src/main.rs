use std::{
    collections::HashMap,
    io,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::{Arc, Mutex},
    time::SystemTime,
};

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
mod blog;
mod helpers;
mod projects;
use axum_server::tls_rustls::RustlsConfig;
use futures::future::{try_join, try_join_all};
use helpers::WebPage;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use projects::PROJECTS;
use serde_qs::axum::OptionalQsQuery;
use tower_http::services::ServeDir;

use crate::{
    blog::{BlogID, BlogPostIndex},
    helpers::PageType,
};

// Port we will host our HTTPS Server on.
const HTTPS_PORT: u16 = 443;

struct BlogState {
    index: BlogPostIndex,
    creation_timestamp: SystemTime, // is_dirty checks creation_timestamp and folder modify timestamp
}

impl BlogState {
    pub fn load() -> Result<Self, io::Error> {
        Ok(BlogState {
            index: blog::index_blog_posts()?,
            creation_timestamp: blog::get_blog_posts_modified_timestamp()?,
        })
    }

    pub fn is_dirty(&self) -> bool {
        self.creation_timestamp
            != blog::get_blog_posts_modified_timestamp()
                .expect("Error checking blog posts folder dirty.")
    }
}

#[tokio::main]
async fn main() {
    let blog_state = Arc::new(Mutex::new(
        BlogState::load().expect("Error loading blog state."),
    ));
    let config = RustlsConfig::from_pem_file("certs/cert.pem", "certs/key.pem")
        .await
        .unwrap();
    // let http_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), PORT);
    let https_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), HTTPS_PORT);
    // let http_redirect_server = tokio::spawn(start_http_redirect_server(http_addr));
    // let https_server = tokio::spawn(start_https_server(https_addr, config, blog_state));
    start_https_server(https_addr, config, blog_state).await;
}

async fn start_https_server(
    addr: SocketAddr,
    config: RustlsConfig,
    blog_state: Arc<Mutex<BlogState>>,
) {
    let app = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog::page::blog_post_list_page))
        .route("/blog_post_list", get(blog::page::blog_post_list_items))
        .route("/blog/{blog_id}", get(blog_post))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(blog_state);
    println!("Running on https://{:?}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home(state: State<Arc<Mutex<BlogState>>>) -> Html<WebPage> {
    let html = WebPage {
        title: "Jordi's Portfolio".into(),
        content: html! {
            h1 .primary-text { "About Me" }
            article .responsive .large-padding .primary-container .on-primary-container {
                h5 {
                    "Full-stack software engineer with experience in web technologies and low-level languages like React, C++, and Rust. Passionate about writing declarative, maintainable code with unit testing to ensure correct code behavior. Being a great communicator is one of the best skills a person can have! Read more about my work below."
                }
            }
            div .large-space {	}
            div .responsive {
                div {
                h3 .large { "My Projects" }
                div .space {	}
                (get_project_cards())
                }
                div .large-space {}
                div .blog-post-home-page-section {
                h3 .large { "Blog Posts" }
                div .space {	}
                (blog::page::blog_post_list_items(OptionalQsQuery(None), state).await)
                }
            }
            h1 .secondary-text { "Contact Me" }
            article .large-padding .secondary-container .on-secondary-container {
                h5 {
                    "Phone: " a href="tel:469-315-5417" {"469-315-5417"} br {}
                    "Email: " a href="mailto:creptthrust@gmail.com" {"creptthrust@gmail.com"} br {}
                    "Thank you for your time!"
                }
            }
        },
        page_type: PageType::Home,
    };
    Html(html)
}

async fn blog_post(
    Path(blog_id): Path<BlogID>,
    State(state_mutex): State<Arc<Mutex<BlogState>>>,
) -> WebPage {
    let mut state = state_mutex.lock().unwrap();
    if state.is_dirty() {
        *state = match BlogState::load() {
            Ok(blog_state) => blog_state,
            Err(err) => return WebPage::error_page(err),
        }
    }
    let info = match state.index.0.get(&blog_id) {
        Some(info) => info,
        None => return WebPage::error_page("Blog post not found."),
    };
    WebPage::blog_page(blog_id, info)
}

fn get_project_cards() -> Markup {
    html! {
        div class="" {
            @for project_info in PROJECTS {
                article class="border" {
                    h4 {
                        (project_info.project)
                    }
                    p {
                        (project_info.short_description)
                    }
                    div {
                        (blog::content_tags_html(&project_info.get_tags()))
                    }
                    @if !project_info.learn_more_link.is_empty() {
                        div class="right-align" {a href=(project_info.learn_more_link) {
                            button { "Learn More" }
                        }
                    }
                }
                }
            }
        }
    }
}
