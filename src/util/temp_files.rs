use std::{
    fs::{create_dir, remove_dir_all, File, remove_file, self},
    path::{Path, PathBuf}, io::Write,
};

use crate::errors::temp_file_error::{TempFilesError, TempFilesErrorActions};

const TEMP_FILES_PATH: &str = "./temp";


fn temp_path() -> &'static Path {
    Path::new(TEMP_FILES_PATH)
}

pub fn init() {
    let path = temp_path();

    if path.exists() {
        remove_dir_all(path).unwrap();
    }
    
    create_dir(path).unwrap();
}

pub fn new_temp_file<'a>(name: &'a str, content: &'a str) -> Result<PathBuf, TempFilesError<'a>> {
    let file_path = temp_path().join(name);
    match File::create(file_path.clone()) {
        Err(e) => Err(TempFilesError {
            action: TempFilesErrorActions::CreateFile,
            target: name,
            message: e.to_string(),
        }),
        Ok(mut file) => {
            file.write_all(content.as_bytes()).unwrap();
            Ok(file_path)
        }
    }
}

pub fn new_temp_dir(dir_name: &str) -> Result<PathBuf, TempFilesError> {
    let new_path = temp_path().join(dir_name);
    match fs::create_dir(new_path.clone()) {
        Err(e) => Err(TempFilesError {
            action: TempFilesErrorActions::CreateDir,
            target: dir_name,
            message: e.to_string(),
        }),
        Ok(_) => Ok(new_path)
    }
}


pub fn delete_temp_file(name:&str) -> Result<bool, TempFilesError> {
    let file_path = temp_path().join(name);
    match remove_file(file_path) {
        Err(e) => Err(TempFilesError {
            action: TempFilesErrorActions::DeleteFile,
            target: name,
            message: e.to_string(),
        }),
        Ok(_) => Ok(true)
    }
}
