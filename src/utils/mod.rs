use percent_encoding::{AsciiSet, CONTROLS};

pub mod duckduck;
pub mod github;
pub mod google;
pub mod protondb;
pub mod rustdocs;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn get_cmd_from_query(query_string: &str) -> &str {
    if query_string.contains(' ') {
        let idx_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..idx_space];
    }
    &query_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cmd_qry_no_whitespace() {
        // Test with cmd only
        assert_eq!(get_cmd_from_query("gh"), "gh");
    }

    #[test]
    fn test_get_cmd_qry_whitespace() {
        // Test with a query string
        assert_eq!(get_cmd_from_query("gh facebook/react"), "gh");
    }
}
