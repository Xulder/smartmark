use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_ddg_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let ddg_search_url = format!("https://duckduckgo.com/?q={}", encoded_query);
    ddg_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_ddg_search() {
        let fake_query = "hello";
        assert_eq!(
            construct_ddg_search_url(fake_query),
            "https://duckduckgo.com/?q=hello"
        );
    }

    #[test]
    fn test_construct_ddg_search_url_with_encoding() {
        let fake_query = "hello world";
        assert_eq!(
            construct_ddg_search_url(fake_query),
            "https://duckduckgo.com/?q=hello%20world"
        );
    }
}
