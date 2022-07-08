
use crate::utils::folder;
use crate::values;
use crate::env;

pub fn delete() {
    let args: Vec<String> = env::args().collect();
    let project_name = &args[2];
    
    let delete_result = folder::remove_folder(values::get_projects_dir().join(project_name).as_path());

    if delete_result.is_ok() {
        println!("Deleted template '{}'.", project_name);
    } else {
        println!("Failed to delete template '{}'.", project_name);
    } 
}
