#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/search?<query>")]
fn search(query: String) -> Redirect {
    let command = utils::get_command_from_query_string(&query);
    let redirect_url = match command {
        "gh" => utils::github::construct_github_url(&query),
        "ghx" => utils::github::construct_wix_github_url(&query),
        "wbo" => utils::wixbo::construct_wix_bo_search_url(&query),
        "wl" => utils::wixlife::construct_wix_life_search_url(&query),
        _ => utils::google::construct_google_search_url(&query)
    };
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}

