use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    default_search: Option<String>,
    shortcut: Option<Vec<ShortcutConfig>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ShortcutConfig {
    keyword: String,
    url: String,
}

pub fn read_config<P>(path: P) -> Config
where
    P: AsRef<Path>,
{
    toml::from_str(&fs::read_to_string(path).expect("Trouble reading config file")).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_config() {
        let config: Config = read_config("./src/test.toml");

        // TODO: Probably will want to do some work using the url crate to allow for simply typing the keyword to omit the search query
        // NOTE: The above can be done by using URL to retrieve the naked url, but this might not have desired behavior for some sites
        assert_eq!(config.default_search.unwrap(), "https://duckduckgo.com/%s");
        assert_eq!(
            config.shortcut.unwrap().contains(&ShortcutConfig {
                keyword: "rs".to_string(),
                url: "https://docs.rs/%s".to_string()
            }),
            true
        );
    }
}
