use percent_encoding::utf8_percent_encode;

pub fn construct_ddg_search_url(query: &str) -> String {
    format!(
        "https://duckduckgo.com/?q={}",
        utf8_percent_encode(query, super::FRAGMENT).to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_ddg_search_url() {
        assert_eq!(
            construct_ddg_search_url("hello"),
            "https://duckduckgo.com/?q=hello"
        );
    }

    #[test]
    fn test_construct_ddg_search_url_with_encoding() {
        assert_eq!(
            construct_ddg_search_url("hello world"),
            "https://duckduckgo.com/?q=hello%20world"
        );
    }
}
