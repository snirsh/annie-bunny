#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        // Test with command only
        let actual = utils::get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let actual = utils::get_command_from_query_string
            ("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}


use rocket::response::Redirect;
mod utils;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let command = utils::get_command_from_query_string(&cmd);
    let redirect_url = match command {
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        "wbo" => utils::wixbo::construct_wix_bo_search_url(&cmd),
        "wl" => utils::wixlife::construct_wix_life_search_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd)
    };
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}

