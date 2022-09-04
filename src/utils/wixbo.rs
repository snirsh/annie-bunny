extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');


pub fn construct_wix_bo_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
    let wix_bo_search_url = format!(
        "https://bo.wix.com/?q={}", encoded_query);
    wix_bo_search_url
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_construct_wbo_search_url(){
        let fake_query = "wbo hello";
        assert_eq!(
            construct_wix_bo_search_url(fake_query),
            "https://bo.wix.com/?q=hello"
        );
    }

    #[test]
    fn test_construct_wbo_search_url_with_encoding() {
        let fake_query = "wbo hello world";
        assert_eq!(
            construct_wix_bo_search_url(fake_query),
            "https://bo.wix.com/?q=hello%20world"
        );
    }
}