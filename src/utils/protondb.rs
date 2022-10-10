use percent_encoding::utf8_percent_encode;

pub fn construct_protondb_search_url(query: &str) -> String {
    format!(
        "https://protondb.com/search?q={}",
        utf8_percent_encode(&query[4..], super::FRAGMENT).to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_protondb_search_url() {
        assert_eq!(
            construct_protondb_search_url("pdb apex"),
            "https://protondb.com/search?q=apex"
        );
    }

    #[test]
    fn test_construct_protondb_search_url_with_encoding() {
        assert_eq!(
            construct_protondb_search_url("pdb apex legends"),
            "https://protondb.com/search?q=apex%20legends"
        );
    }
}
