extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');


pub fn construct_github_url(query: &str) ->  String {
    if query == "gh" {
        let github_full_url = "https://github.com";
        github_full_url.to_string()
    } else if &query[..4] == "gh @" {
        let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
        let github_full_url = format!("https://github.com/{}", encoded_query);
        github_full_url
    } else {
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let github_full_url = format!("https://github.com/search?q={}&ref=opensearch", encoded_query);
        github_full_url
    }
}

pub fn construct_wix_github_url(query: &str) -> String {
    if query == "ghx" {
        let github_wix_url = "https://github.com/search?q=org%3Awix-p+org%3Awix-private";
        github_wix_url.to_string()
    } else {
        let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
        let w_github_full_url = format!("https://github.com/search?q={}+org%3Awix-p+org%3Awix-private", encoded_query);
        w_github_full_url
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_profile_url_with_gh() {
        let fake_query = "gh wix-private";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/wix-private"
        );
    }

    #[test]
    fn test_construct_github_search_url_with_repo_url() {
        let fake_query = "gh @wix-private/thunderbolt";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/wix-private/thunderbolt"
        );
    }

    #[test]
    fn test_construct_wix_github_profile_url_with_gh() {
        let fake_query = "ghx thunderbolt";
        assert_eq!(
            construct_wix_github_url(fake_query),
            "https://github.com/search?q=thunderbolt+org%3Awix+org%3Awix-p+org%3Awix-private+org%3Awix-playground+org%3Awix-platform+org%3Awix-system+org%3Awix-incubator"
        );
    }
}
