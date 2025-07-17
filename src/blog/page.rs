use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use chrono::Utc;
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;
use serde_qs::axum::OptionalQsQuery;
use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crate::{
    blog,
    helpers::{remove_quotes, WebPage},
    projects::{self, ContentTag},
    BlogState,
};

fn is_tag_selected(tag: &ContentTag, current_filter_set: &Option<TagFilterSet>) -> bool {
    current_filter_set
        .as_ref()
        .filter(|filter_vec| filter_vec.filters.contains(&tag))
        .is_some()
    // current_filter_set.filters.contains(tag)
}

#[derive(Deserialize, Debug)]
pub struct TagFilterSet {
    #[serde(default)]
    pub filters: Vec<ContentTag>,
}

fn get_blog_post_date_str(post_created_at: SystemTime) -> String {
    let date: chrono::DateTime<Utc> = post_created_at.into();
    date.format("%a %B %d, %Y").to_string()
}

pub async fn blog_post_list_page(
    query: OptionalQsQuery<TagFilterSet>,
    state: State<Arc<Mutex<BlogState>>>,
) -> WebPage {
    WebPage {
        title: "Jordi's Blog Posts".into(),
        content: html! {
            div {
                h4 .large { "Blog Posts" }
                div .medium-space {}
                (blog_post_list_items(query, state).await)
            }
        },
        page_type: crate::helpers::PageType::Blog,
    }
}

fn get_posts_list_markup(posts: Vec<(&blog::BlogID, &blog::BlogPostInfo)>) -> Markup {
    html! {
        ul .list .border {
            @for (id, post_info) in posts {
                li {
                        div class="max" {
                            a href=(format!("blog/{}", id.file_name)) {
                                h6 class="large" {
                                    {(post_info.title)}
                                }
                                p {
                                    (post_info.tagline)
                                }
                            }
                            div {
                                @for tag in &post_info.tags {
                                    @let tag_string = {
                                        let t = serde_json::to_string(tag).unwrap();
                                        t[1..t.len()-1].to_string()
                                    };
                                    button .chip #filter_tag data-tag=(tag_string) {
                                        (tag)
                                    }
                                }
                            }
                            p {
                                (get_blog_post_date_str(post_info.created_at))
                            }
                    }
                }
            }
        }
    }
}

pub async fn blog_post_list_items(
    OptionalQsQuery(current_filter_set): OptionalQsQuery<TagFilterSet>,
    State(state_mutex): State<Arc<Mutex<BlogState>>>,
) -> Markup {
    let mut state = state_mutex.lock().unwrap();
    if state.is_dirty() {
        *state = match BlogState::load() {
            Ok(blog_state) => blog_state,
            Err(err) => todo!(),
        }
    }
    // println!("{:?}");
    let posts: Vec<(&blog::BlogID, &blog::BlogPostInfo)> = match current_filter_set {
        Some(ref filter_set) => state
            .index
            .as_sorted_vec()
            .into_iter()
            .filter(|(_, post_info)| {
                println!("{:?} {:?}", filter_set.filters, post_info.title);
                for filter in &filter_set.filters {
                    if !post_info.tags.contains(&filter) {
                        println!("{:?} {:?}, tag NO CONTAIn", filter_set.filters, filter);
                        return false;
                    }
                    println!("{:?} {:?}, tag YES", filter_set.filters, filter);
                }
                true
            })
            .collect(),
        None => state.index.as_sorted_vec(),
    };
    let posts_markup: Markup = 'check_post_empty_state: {
        if posts.is_empty() {
            break 'check_post_empty_state html! {
                article .border .round .middle-align .center-align .medium {
                    div  {
                        i .extra { "search" }
                        h2 { "No Posts Found" }
                        h4 { "Try removing some filters." }
                    }
                }
            };
        }
        break 'check_post_empty_state get_posts_list_markup(posts);
    };
    html! {
        div #blog_posts {
            article .border {
                h6 { "Filters" }
                div .space {}
                @for tag in &projects::ALL_CONTENT_TAGS {
                    @let selected = is_tag_selected(tag, &current_filter_set);
                    @let tag_string = {
                        remove_quotes(serde_json::to_string(tag).unwrap())
                    };
                    button .chip .primary-container[selected] #filter_tag data-tag=(tag_string) {
                        (tag)
                    }
                }
            }
            (posts_markup)
        }
    }
}
