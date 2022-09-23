use percent_encoding::utf8_percent_encode;

pub fn construct_google_search_url(query: &str) -> String {
    format!(
        "https://google.com/search?q={}",
        utf8_percent_encode(&query[3..], super::FRAGMENT).to_string()
    )
}

pub fn construct_google_image_search_url(query: &str) -> String {
    format!(
        "https://google.com/search?q={}&tbm=isch",
        utf8_percent_encode(&query[3..], super::FRAGMENT).to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        assert_eq!(
            construct_google_search_url("go hello"),
            "https://google.com/search?q=hello"
        );
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        assert_eq!(
            construct_google_search_url("go hello world"),
            "https://google.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_google_image_search_url() {
        assert_eq!(
            construct_google_image_search_url("im hello"),
            "https://google.com/search?q=hello&tbm=isch"
        );
    }

    #[test]
    fn test_construct_google_image_search_url_with_encoding() {
        assert_eq!(
            construct_google_image_search_url("im hello world"),
            "https://google.com/search?q=hello%20world&tbm=isch"
        );
    }
}
