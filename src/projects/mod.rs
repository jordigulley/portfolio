use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

// Update `ALL_CONTENT_TAGS` when this enum changes.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ContentTag {
    Cpp,
    Rust,
    GodotEngine,
    Game,
    LanguageDevelopment,
    Compiler,
    BytecodeVm,
    Website,
    WebDevelopment,
    OpenSource,
}
pub const ALL_CONTENT_TAGS: [ContentTag; 10] = [
    ContentTag::Cpp,
    ContentTag::Rust,
    ContentTag::GodotEngine,
    ContentTag::Game,
    ContentTag::LanguageDevelopment,
    ContentTag::Compiler,
    ContentTag::BytecodeVm,
    ContentTag::Website,
    ContentTag::WebDevelopment,
    ContentTag::OpenSource,
];

impl Display for ContentTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ContentTag::Cpp => "C++",
            ContentTag::Rust => "Rust",
            ContentTag::GodotEngine => "Godot Engine",
            ContentTag::Game => "Game",
            ContentTag::Website => "Website",
            ContentTag::OpenSource => "Open Source Project",
            ContentTag::LanguageDevelopment => "Programming Language",
            ContentTag::Compiler => "Compiler",
            ContentTag::BytecodeVm => "Bytecode Virtual Machine",
            ContentTag::WebDevelopment => "Web Development",
        })
    }
}

#[derive(Copy, Clone)]
pub enum Project {
    Jem = 0,
    Strawberry = 1,
    FmodGd4 = 2,
    LeptosMaterial = 3,
    Ticks = 4,
}

// impl ToString for Project {
//     fn to_string(&self) -> String {
//         .to_string()
//     }
// }

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Project::Jem => "JEM",
            Project::Strawberry => "StrawberryLang",
            Project::FmodGd4 => "FMOD Integration for Godot 4",
            Project::LeptosMaterial => "Leptos Material Web Components",
            Project::Ticks => "Ticks",
        })
    }
}

pub const PROJECTS:[ProjectInfo; 5] = [
    ProjectInfo {
        project: Project::Jem,
        short_description: "A fast-paced RPG made with a custom version of the Godot Engine, written in C++.",
        learn_more_link: "",
    },
    ProjectInfo {
        project: Project::Strawberry,
        short_description: "A game programming language inspired by Rust's simple syntax and functional programming influences. Made for Godot.",
        learn_more_link: "",
    },
    ProjectInfo {
        project: Project::FmodGd4,
        short_description: "C++ integration of the FMOD Sound System for the Godot Engine.",
        learn_more_link: "https://github.com/jordigulley/fmod_gd4",
    },
    ProjectInfo {
        project: Project::LeptosMaterial,
        short_description: "A Leptos component wrapper for Material Web Components (MWC), along with some extra components to fill in the gaps.",
        learn_more_link: "https://github.com/jordigulley/leptos-material",
    },
    ProjectInfo {
        project: Project::Ticks,
        short_description: "Simple, ergonomic Rust wrapper for the TickTick Open API",
        learn_more_link: "https://github.com/jordigulley/ticks",
    },
];

#[derive(Clone)]
pub struct ProjectInfo {
    pub project: Project,
    pub short_description: &'static str,
    pub learn_more_link: &'static str,
}

impl ProjectInfo {
    pub fn get_tags(&self) -> Vec<ContentTag> {
        match self.project {
            Project::Jem => vec![ContentTag::Cpp, ContentTag::GodotEngine, ContentTag::Game],
            Project::Strawberry => vec![
                ContentTag::Cpp,
                ContentTag::GodotEngine,
                ContentTag::LanguageDevelopment,
                ContentTag::BytecodeVm,
                ContentTag::Compiler,
            ],
            Project::FmodGd4 => vec![
                ContentTag::Cpp,
                ContentTag::GodotEngine,
                ContentTag::OpenSource,
            ],
            Project::LeptosMaterial => vec![
                ContentTag::Rust,
                ContentTag::WebDevelopment,
                ContentTag::OpenSource,
            ],
            Project::Ticks => vec![
                ContentTag::Rust,
                ContentTag::WebDevelopment,
                ContentTag::OpenSource,
            ],
        }
    }
}
