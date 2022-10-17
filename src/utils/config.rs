use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

const CONFIG_TEMPLATE: &[u8] = r#"# You can change your default search engine here.
default_search = "https://duckduckgo.com/"

# Anything typed after a given keyword will be appended to the `url` field.
[[shortcuts]]
keyword = "rs"
url = "https://docs.rs/"
# keywords with `?` appended will use the `search_url` if available.
search_url = "https://docs.rs/releases/search?query="
"#
.as_bytes();

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_search: String,
    pub shortcuts: Option<Vec<Shortcut>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Shortcut {
    pub keyword: String,
    pub url: String,
    pub search_url: Option<String>,
}

/// Creates default user config if one doesn't exist
pub fn get_user_config_dir() -> PathBuf {
    let mut config_path = PathBuf::new();
    if cfg!(windows) {
        config_path.push(env::var("LOCALAPPDATA").unwrap());
    } else if cfg!(unix) {
        config_path.push(env::var("XDG_CONFIG_HOME").unwrap());
    } else if cfg!(macos) {
        config_path.push(format!("{}/.config", env::var("HOME").unwrap()));
    }

    config_path.push("smartmark");

    if !config_path.exists() {
        fs::create_dir(&config_path)
            .expect("Error creating smartmark local configuration directory");
    }

    config_path.push("config.toml");

    if !config_path.exists() {
        let mut config_file =
            File::create(&config_path).expect("Error writing user-configuration file");
        config_file.write_all(CONFIG_TEMPLATE);
    }

    config_path
}

pub fn read_user_config<P>(path: P) -> Config
where
    P: AsRef<Path>,
{
    toml::from_str(&fs::read_to_string(path).expect("Trouble reading config file")).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_user_config() {
        let config: Config = read_user_config("./config.toml");

        assert_eq!(config.default_search, "https://duckduckgo.com/");
        assert_eq!(
            config.shortcuts.unwrap().contains(&Shortcut {
                keyword: "rs".to_string(),
                url: "https://docs.rs/".to_string(),
                search_url: Some("https://docs.rs/releases/search?query=".to_string())
            }),
            true
        );
    }
}
