extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');


pub fn construct_calendar_search_url(query: &str) -> String {
    if query == "cal" {
        let calendar_url = "https://calendar.google.com/calendar/u/0/r";
        calendar_url.to_string()
    } else if &query[4..] == "ev" {
        let calendar_ev_url = "http://www.google.com/calendar/event?action=TEMPLATE";
        calendar_ev_url.to_string()
    } else {
        let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
        let calendar_query_url = format!(
            "https://calendar.google.com/calendar/u/0/r/search?q={}", encoded_query);
        calendar_query_url
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_construct_calendar_base_url(){
        let fake_query = "cal";
        assert_eq!(
            construct_calendar_search_url(fake_query),
            "https://calendar.google.com/calendar/u/0/r"
        );
    }

    #[test]
    fn test_construct_calendar_search_url(){
        let fake_query = "cal hello";
        assert_eq!(
            construct_calendar_search_url(fake_query),
            "https://calendar.google.com/calendar/u/0/r/search?q=hello"
        );
    }

    #[test]
    fn test_construct_calendar_search_url_with_encoding() {
        let fake_query = "cal hello world";
        assert_eq!(
            construct_calendar_search_url(fake_query),
            "https://calendar.google.com/calendar/u/0/r/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_calendar_event_url() {
        let fake_query = "cal ev";
        assert_eq!(
            construct_calendar_search_url(fake_query),
            "https://www.google.com/calendar/event?action=TEMPLATE"
        );
    }
}