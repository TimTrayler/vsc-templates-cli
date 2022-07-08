
use crate::utils::folder;
use crate::values;
use crate::env;

pub fn new() {
    let args: Vec<String> = env::args().collect();
    let project_name = &args[2];

    if folder::folder_exists(values::get_projects_dir().as_path().display().to_string(), project_name.to_string()).unwrap() {
        let cp_folder_result = folder::copy_folder(values::get_projects_dir().join(project_name).as_path(), env::current_dir().unwrap().as_path());

        if cp_folder_result.is_ok() {
            println!("Setup using template '{}' finished.", project_name);
        } else {
            println!("Failed to setup template '{}'.", project_name);
        }
    } else {
        println!("No such template: {}", project_name);
    }
}
