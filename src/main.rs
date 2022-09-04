#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_get_command_from_query_string_no_whitespace() {
//         // Test with command only
//         let actual = utils::get_command_from_query_string("tw");
//         let expected = "tw";
//         assert_eq!(actual, expected);
//     }
//     #[test]
//     fn test_get_command_from_query_string_with_whitespace() {
//         let actual = utils::get_command_from_query_string
//             ("tw @fbOpenSource");
//         let expected = "tw";
//         assert_eq!(actual, expected);
//     }
// }


use rocket::response::Redirect;
mod utils;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
        _ => utils::google::construct_google_search_url(&q)
    };
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![search]).launch();
}

