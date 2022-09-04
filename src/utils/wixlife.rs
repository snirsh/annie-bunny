extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');


pub fn construct_wix_life_search_url(query: &str) -> String {
    if query == "wl" {
        let wl_url = "https://www.wix-life.com";
        wl_url.to_string()
    } else if &query[..4] == "wl @" {
        let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
        let wix_life_profile_url = format!(
            "https://www.wix-life.com/profile/{}", encoded_query);
        wix_life_profile_url
    } else {
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let wix_life_search_url = format!(
            "https://www.wix-life.com/search?q={}", encoded_query);
        wix_life_search_url
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_construct_wl_search_url(){
        let fake_query = "wl hello";
        assert_eq!(
            construct_wix_life_search_url(fake_query),
            "https://www.wix-life.com/search?q=hello"
        );
    }

    #[test]
    fn test_construct_wl_search_url_with_encoding() {
        let fake_query = "wl hello world";
        assert_eq!(
            construct_wix_life_search_url(fake_query),
            "https://www.wix-life.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_wl_profile_url() {
        let fake_query = "wl @snirs";
        assert_eq!(
            construct_wix_life_search_url(fake_query),
            "https://www.wix-life.com/profile/snirs"
        );
    }
}