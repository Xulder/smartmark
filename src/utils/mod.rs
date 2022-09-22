pub mod duckduck;
pub mod github;

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
        let actual = get_cmd_from_query("gh");
        let expected = "gh";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_cmd_qry_whitespace() {
        // Test with a query string
        let actual = get_cmd_from_query("gh facebook/react");
        let expected = "gh";
        assert_eq!(actual, expected);
    }
}
