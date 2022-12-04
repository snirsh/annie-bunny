extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');


pub fn construct_youtube_url(query: &str) ->  String {
    if query == "yt" {
        let youtube_full_url = "https://youtube.com";
        youtube_full_url.to_string()
    } else if &query[..4] == "yt @" {
        let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
        let youtube_full_url = format!("https://youtube.com/@{}", encoded_query);
        youtube_full_url
    } else {
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let youtube_full_url = format!("https://www.youtube.com/results?search_query={}", encoded_query);
        youtube_full_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_youtube_profile_url_with_yt() {
        let fake_query = "yt @metallica";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://youtube.com/@metallica"
        );
    }

    #[test]
    fn test_construct_youtube_search_url_with_yt() {
        let fake_query = "yt thunderbolt";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://www.youtube.com/results?search_query=thunderbolt"
        );
    }
}
