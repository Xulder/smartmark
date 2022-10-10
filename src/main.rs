#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{response::Redirect, State};
use url::Url;
use utils::{
    config::{read_user_config, Config, Shortcut},
    construct_url,
};

#[macro_use]
extern crate rocket;

mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

// FIXME: Uri redirect failing properly, not matching shortcuts for some reason.
#[get("/search?<cmd>")]
fn search(cmd: String, state: &State<Config>) -> Redirect {
    let command = utils::get_cmd_from_query(&cmd);
    println!("{}", command);
    let mut def_search = state.default_search.clone();

    if let Some(shortcuts) = &state.shortcuts {
        for shortcut in shortcuts.iter() {
            println!("{}", shortcut.keyword);
            match shortcut {
                Shortcut {
                    // TODO: Rewrite this to accept keywords with `?` suffixes
                    keyword: command,
                    ..
                } => println!("{}", utils::construct_url(&cmd, shortcut)),
                _ => continue,
            };
        }
    };
    def_search.push_str(&cmd);
    Redirect::to(def_search)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, search])
        .manage(read_user_config("./test.toml"))
}

/*
NOTE: What might be interesting is altering the config setup like so;
```toml
[[shortcut]]
keyword = "bl"
# root  url/page you want bookmarked, last part lets you go wherever under that page
url = "blahblah.com/%s" # use the base url
# search query used on website
search_query = "search?q=" # this would be optional
```
Naked keyword should go straight to root url
Any words after should be immediately substituted in where %s is
`?` proceeding a word after the keyword implies a search query be applied to the resulting url
This makes it so a user won't have to make new shortcuts for every site that uses search queries
Additionally, for urls like "www.google.com", the `search_query` would be unneccessary and simply entered as "www.google.com/search?q=%s"
what's interesting is with this feature using ? before any word would insert a search query
this means that something like github.com with a keyword of `gh` could be use like this;
gh rust-lang/rust ?stdio
Could also do multiple
*/
