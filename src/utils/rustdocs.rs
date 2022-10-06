use percent_encoding::utf8_percent_encode;

pub fn construct_rustdocs_url(query: &str) -> String {
    // if you add a `?` before the crate name, it will do a search query
    if query.chars().nth(3) == Some('?') {
        format!(
            "https://docs.rs/releases/search?query={}",
            utf8_percent_encode(&query[4..], super::FRAGMENT).to_string()
        )
    } else {
        // Otherwise it will attempt to go straight to the docs page, even if it doesn't exist
        format!(
            "https://docs.rs/{}",
            // crate names can't have spaces, but this works with ones with `-` as well
            &query[3..].replace(" ", "_").to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_rustdocs_search_url() {
        assert_eq!(
            construct_rustdocs_url("rs ?gtt"),
            "https://docs.rs/releases/search?query=gtt"
        );
    }

    #[test]
    fn test_construct_rustdocs_search_url_with_encoding() {
        assert_eq!(
            construct_rustdocs_url("rs ?gtt work"),
            "https://docs.rs/releases/search?query=gtt%20work"
        );
    }

    #[test]
    fn test_construct_rustdocs_crate_url() {
        assert_eq!(
            construct_rustdocs_url("rs rocket"),
            "https://docs.rs/rocket"
        );
    }

    #[test]
    fn test_construct_rustdocs_crate_url_with_whitespace() {
        assert_eq!(
            construct_rustdocs_url("rs rocket codegen"),
            "https://docs.rs/rocket_codegen"
        );
    }
}
