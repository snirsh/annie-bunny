extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');


pub fn construct_jira_search_url(query: &str) -> String {
    if query == "jr" {
        let jira_url = "https://jira.wixpress.com/";
        jira_url.to_string()
    } else if &query[..4] == "jr @" {
        let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
        let wix_life_search_url = format!(
            "https://jira.wixpress.com/browse/{}", encoded_query);
        wix_life_search_url
    } else {
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let wix_life_search_url = format!(
            "https://jira.wixpress.com/secure/QuickSearch.jspa?searchString={}", encoded_query);
        wix_life_search_url
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_construct_jira_search_url(){
        let fake_query = "jr hello";
        assert_eq!(
            construct_jira_search_url(fake_query),
            "https://jira.wixpress.com/secure/QuickSearch.jspa?searchString=hello"
        );
    }

    #[test]
    fn test_construct_jira_search_url_with_encoding() {
        let fake_query = "jr hello world";
        assert_eq!(
            construct_jira_search_url(fake_query),
            "https://jira.wixpress.com/secure/QuickSearch.jspa?searchString=hello%20world"
        );
    }

    #[test]
    fn test_construct_jira_search_url_with_at() {
        let fake_query = "jr @1";
        assert_eq!(
            construct_jira_search_url(fake_query),
            "https://jira.wixpress.com/browse/1"
        );
    }
}