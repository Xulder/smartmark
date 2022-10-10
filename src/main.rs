#![feature(proc_macro_hygiene, decl_macro)]
use std::env;

use rocket::{response::Redirect, State};
use utils::config::{read_user_config, Config};

#[macro_use]
extern crate rocket;

mod utils;

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
    let user_config = env::var("XDG_CONFIG_HOME").unwrap();
    rocket::build()
        .mount("/", routes![index, search])
        .manage(read_user_config(format!(
            "{user_config}/smartmark/config.toml"
        )))
}

// TODO: Make cli and implement feature to reload config
// TODO: Add README & License
