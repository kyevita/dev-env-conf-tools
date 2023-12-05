use std::path::PathBuf;

use home::home_dir;

use crate::helpers::zsh_helper::open_zshrc_file;

use super::file_helper::search_line_by_string;

const OH_MY_ZSH_INSTALL_LINE: &str = "export ZSH=\"$HOME/.oh-my-zsh\"";
const OH_MY_ZSH_INSTALLATION_PATH: &str = ".oh-my-zsh/";
const OH_MY_ZSH_CUSTOM_PATH: &str = ".oh-my-zsh/custom";

pub fn get_installation_path() -> PathBuf {
    let home_path = home_dir().unwrap();

    home_path.join(OH_MY_ZSH_INSTALLATION_PATH)
}

pub fn get_custom_path() -> PathBuf {
    let home_path = home_dir().unwrap();

    home_path.join(OH_MY_ZSH_CUSTOM_PATH)
}

pub fn get_custom_themes_path() -> PathBuf {
    get_custom_path().join("/themes")
}

pub fn verify_installation() -> bool {
    let zshrc_file = open_zshrc_file();
    get_installation_path().exists()
        && search_line_by_string(&zshrc_file, String::from(OH_MY_ZSH_INSTALL_LINE)).is_some()
}
