#![feature(proc_macro_hygiene, decl_macro)]
use rocket::response::Redirect;

#[macro_use]
extern crate rocket;

mod utils;
mod config;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let command = utils::get_cmd_from_query(&cmd);

    let redirect_url = match command {
        "rs" => utils::rustdocs::construct_rustdocs_url(&cmd),
        "go" => utils::google::construct_google_search_url(&cmd),
        "im" => utils::google::construct_google_image_search_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        _ => utils::duckduck::construct_ddg_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search])
}
