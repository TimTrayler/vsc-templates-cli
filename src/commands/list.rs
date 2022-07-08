
use crate::values;
use std::fs;

pub fn list() {
    let templates = fs::read_dir(values::get_projects_dir().as_path()).unwrap();

    println!("Available Templates:");

    for template in templates {
        println!("> {}", template.unwrap().file_name().to_str().unwrap());
    }
}
