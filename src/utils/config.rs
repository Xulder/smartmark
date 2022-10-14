use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
