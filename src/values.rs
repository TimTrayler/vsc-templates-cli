
extern crate dirs;

use std::path;

pub fn get_projects_dir() -> path::PathBuf {
    return dirs::config_dir().unwrap().join("Code").join("User").join("ProjectTemplates").to_path_buf();
}

pub const SYNTAX_MSG: &str = "Syntax: 'templates [new|delete|list|save] [<name>]'";
