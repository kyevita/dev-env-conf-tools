use envy;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Configuration {
    pub oh_my_zsh_git_install_sh: String,
    pub powerlevel10k_git: String,
}

pub fn load_config() -> Configuration {
    return envy::from_env::<Configuration>().expect("Invalid env file");
}