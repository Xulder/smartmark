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

mod cli;
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
    rocket::build()
        .mount("/", routes![index, search])
        .manage(config::read_user_config(config::get_user_config_dir()))
}

// TODO: Make cli and implement feature to reload config
// NOTE: Make server features like port and address configurable
// TODO: Add README & License
