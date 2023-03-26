#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
mod utils;


#[get("/")]
fn index() -> &'static str {
    "Annie-bunny is up!"
}

#[get("/search?<q>")]
fn search(q: String) -> Redirect {
    let command = utils::get_command_from_query_string(&q);
    let redirect_url = match command {
        "gh" => utils::github::construct_github_url(&q),
        "ghx" => utils::github::construct_wix_github_url(&q),
        "wbo" => utils::wixbo::construct_wix_bo_search_url(&q),
        "wl" => utils::wixlife::construct_wix_life_search_url(&q),
        "cal" => utils::calendar::construct_calendar_search_url(&q),
        "jr" => utils::jira::construct_jira_search_url(&q),
        "yt" => utils::youtube::construct_youtube_url(&q),
        "pr" => utils::github::construct_github_prs(),
        _ => utils::google::construct_google_search_url(&q)
    };
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![search]).launch();
}

