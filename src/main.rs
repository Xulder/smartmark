#![feature(proc_macro_hygiene, decl_macro)]
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use rocket::{response::Redirect, State};
use utils::config::{self, Config};

#[macro_use]
extern crate rocket;

mod utils;

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

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

#[get("/search?<cmd>")]
fn search(cmd: String, state: &State<Config>) -> Redirect {
    Redirect::to(utils::construct_url(&cmd, &state))
}

#[launch]
fn rocket() -> _ {
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

    let mut config_file =
        File::create(&config_path).expect("Error writing user-configuration file");

    config_file.write_all(CONFIG_TEMPLATE);
    rocket::build()
        .mount("/", routes![index, search])
        .manage(config::read_user_config(config_path))
}

// TODO: Make cli and implement feature to reload config
// TODO: Create config directory and basic `config.toml` if it doesn't exist
// NOTE: Completed for Unix, need to test winblows, I guess.
// TODO: Make server features like port and address configurable
// TODO: Add README & License
