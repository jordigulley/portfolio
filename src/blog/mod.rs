use matter::matter as separate_frontmatter_and_content;
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;
use std::{collections::HashMap, io, path::Path, str::FromStr, time::SystemTime};

use crate::{helpers::remove_quotes, projects::ContentTag};

pub mod page;

const BLOG_POST_FOLDER_PATH: &'static str = "./posts/";

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
#[serde(transparent)]
pub struct BlogID {
    pub file_name: String,
}

impl BlogID {
    pub const fn empty() -> Self {
        Self {
            file_name: String::new(),
        }
    }

    pub fn load_html(&self) -> Result<Markup, io::Error> {
        let md = std::fs::read_to_string(format!("./posts/{}.md", self.file_name))?;
        let content = match separate_frontmatter_and_content(&md) {
            Some((_, post_content)) => post_content,
            None => return Err(io::ErrorKind::InvalidData.into()),
        };
        Ok(PreEscaped(markdown::to_html(&content)))
    }
}

impl ToString for BlogID {
    fn to_string(&self) -> String {
        self.file_name.clone()
    }
}

#[derive(PartialEq, Eq, Hash, Deserialize, Debug)]
pub struct BlogPostInfo {
    pub title: String,
    pub tagline: String,
    pub image: Option<String>,
    #[serde(default)]
    pub tags: Vec<ContentTag>,
    #[serde(skip, default = "SystemTime::now")]
    pub created_at: SystemTime,
}

#[derive(Default)]
pub struct BlogPostIndex(pub HashMap<BlogID, BlogPostInfo>);

impl BlogPostIndex {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn as_sorted_vec(&self) -> Vec<(&BlogID, &BlogPostInfo)> {
        let mut vec = self.0.iter().collect::<Vec<(&BlogID, &BlogPostInfo)>>();
        vec.sort_by(|a, b| b.1.created_at.cmp(&a.1.created_at));
        vec
    }
}

pub fn index_blog_posts() -> Result<BlogPostIndex, io::Error> {
    let dir = std::fs::read_dir(Path::new(BLOG_POST_FOLDER_PATH))?;
    let mut index = BlogPostIndex::new();
    for result in dir {
        let entry = result?;
        let post_md = std::fs::read_to_string(entry.path())?;
        let frontmatter_yaml = match separate_frontmatter_and_content(&post_md) {
            Some((frontmatter, _)) => frontmatter,
            None => return Err(io::ErrorKind::InvalidData.into()),
        };
        let mut post_info: BlogPostInfo = match serde_yml::from_str(&frontmatter_yaml) {
            Ok(post_info) => post_info,
            Err(_) => return Err(io::ErrorKind::InvalidData.into()),
        };
        post_info.created_at = entry
            .metadata()
            .expect("Error metadata")
            .created()
            .expect(""); // TODO BETTER ERROR HANDLE
        index.0.insert(
            BlogID {
                file_name: String::from(
                    Path::new(&entry.file_name())
                        .with_extension("")
                        .to_string_lossy(),
                ),
            },
            post_info,
        );
    }
    Ok(index)
}

pub fn get_blog_posts_modified_timestamp() -> Result<SystemTime, io::Error> {
    let meta = std::fs::metadata(Path::new(BLOG_POST_FOLDER_PATH))?;
    meta.modified()
}

pub fn content_tags_html(tags: &Vec<ContentTag>) -> Markup {
    html! {
        @for tag in tags {
            a href={ "/blog?filters[0]=" (remove_quotes(serde_json::to_string(tag).unwrap())) }{
                button class="chip" {
                    (tag)
                }
            }
        }
    }
}
