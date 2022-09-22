use rocket::response::Redirect;

#[macro_use]
extern crate rocket;

mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let command = utils::get_cmd_from_query(&cmd);

    let redirect_url = match command {
        "go" => String::from("https://google.com"),
        "im" => String::from("https://images.google.com"),
        "gh" => utils::github::construct_github_url(&cmd),
        _ => utils::duckduck::construct_ddg_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search])
}
