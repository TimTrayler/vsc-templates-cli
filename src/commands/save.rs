
use crate::utils::folder;
use crate::values;
use crate::env;
use std::path;
use std::fs;

pub fn save() {
    let args: Vec<String> = env::args().collect();
    let name = &args[2];

    fs::create_dir_all(path::Path::new(values::get_projects_dir().as_path()).join(name)).unwrap();
    let cpf_result = folder::copy_folder(env::current_dir().unwrap().as_path(), values::get_projects_dir().join(name).as_path());

    if cpf_result.is_ok() {
        println!("Template saved as '{}'.", name);
    } else {
        println!("Failed to save template '{}'.", name);
    }
}
