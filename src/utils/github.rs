use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

pub fn construct_github_url(query: &str) -> String {
    if query == "gh" {
        "https://github.com".to_string()
    } else {
        format!(
            "https://github.com/{}",
            utf8_percent_encode(&query[3..], super::FRAGMENT).to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_profile() {
        assert_eq!(construct_github_url("gh"), "https://github.com");
    }

    #[test]
    fn test_construct_github_search_profile() {
        assert_eq!(
            construct_github_url("gh rust-lang"),
            "https://github.com/rust-lang"
        );
    }

    #[test]
    fn test_construct_github_search_repo() {
        assert_eq!(
            construct_github_url("gh rust-lang/rust"),
            "https://github.com/rust-lang/rust"
        );
    }
}
