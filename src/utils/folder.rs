
use walkdir::WalkDir;
use std::path;
use std::io;
use std::fs;

pub fn copy_folder(src: &path::Path, dst: &path::Path) -> io::Result<()> {
    for entry in WalkDir::new(src).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        let src_path = entry.path();
        let dst_path = path::Path::new(dst).join(src_path.strip_prefix(src).unwrap());
        
        if entry.file_type().is_dir() {
            fs::create_dir(dst_path)?;
        } else {
            let cp_result = fs::copy(src_path, dst_path);

            if cp_result.is_err() {
                return Err(cp_result.unwrap_err());
            }
        }
    }

    Ok(())
}


pub fn remove_folder(folder: &path::Path) -> io::Result<()> {
    for entry in WalkDir::new(folder).min_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.file_type().is_dir() {
            if path.read_dir().unwrap().next().is_none() {
                let result = fs::remove_dir(path);

                if result.is_err() {
                    return Err(result.unwrap_err()).unwrap();
                }
            }
        } else {
            let result = fs::remove_file(path);

            if result.is_err() {
                return Err(result.unwrap_err()).unwrap();
            }
        }
    }

    let result = fs::remove_dir(folder);

    if result.is_err() {
        return Err(result.unwrap_err()).unwrap();
    }

    Ok(())
}


// Slightly modified from: https://stackoverflow.com/a/47725486/14230069
pub fn folder_exists(search_folder: String, folder: String) -> Result<bool, io::Error> {
    Ok(fs::read_dir(search_folder)?.find(|x| {
        let x = x.as_ref().unwrap();
        x.file_type().unwrap().is_dir() && x.file_name().to_str().unwrap() == folder
    }).is_some())
}
