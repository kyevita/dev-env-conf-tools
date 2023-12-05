use std::io::{Read, Write};
use std::{fs::File, path::PathBuf};

use home::home_dir;

use super::file_helper;

pub const ZSHRC_FILENAME: &str = ".zshrc";

pub enum ZshVariables {
    ZshTheme,
}

impl ZshVariables {
    fn as_string(&self) -> String {
        match self {
            ZshVariables::ZshTheme => String::from("ZSH_THEME"),
        }
    }
}

fn replace_zshrc_file(new_data: &[u8]) {
    let mut new_file = File::create(get_zshrc_path()).unwrap();
    new_file.write_all(new_data).unwrap()
}

pub fn get_zshrc_path() -> PathBuf {
    home_dir().unwrap().join(ZSHRC_FILENAME)
}

pub fn modify_zshrc_variable(variable: ZshVariables, value: &str) -> bool {
    let mut zshrc_file = open_zshrc_file();

    // Find variable line
    let old_var_line = match file_helper::search_line_by_string(&zshrc_file, variable.as_string()) {
        Some(l) => l,
        None => return false,
    };
    let mut new_var_line = old_var_line.clone();

    // Open and read the file entirely and store it into memory
    let mut old_zshrc_file_data = String::new();
    zshrc_file.read_to_string(&mut old_zshrc_file_data).unwrap();
    drop(zshrc_file); // Close the file early

    // Run the replace operation in memory
    let offset = new_var_line.find("=").unwrap_or(new_var_line.len());
    new_var_line.replace_range(offset.., value);

    let new_zshrc_file_data = old_zshrc_file_data.replace(&old_var_line, &new_var_line);

    // Replace the old zshrc file with the new one
    replace_zshrc_file(new_zshrc_file_data.as_bytes());

    true
}

pub fn open_zshrc_file() -> File {
    File::open(get_zshrc_path()).unwrap()
}
