use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_github_url(query: &str) -> String {
    if query == "gh" {
        let github_dotcom = "https://github.com";
        github_dotcom.to_string()
    } else {
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let github_url = format!("https://github.com/{}", encoded_query);
        github_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_profile() {
        let fake_query = "gh";
        assert_eq!(construct_github_url(fake_query), "https://github.com");
    }

    #[test]
    fn test_construct_github_search_profile() {
        let fake_query = "gh rust-lang";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/rust-lang"
        );
    }

    #[test]
    fn test_construct_github_search_repo() {
        let fake_query = "gh rust-lang/rust";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/rust-lang/rust"
        );
    }
}
