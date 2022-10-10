use percent_encoding::{AsciiSet, CONTROLS};

use self::config::{Config, Shortcut};

pub mod config;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// Keywords should never be more than one word, should only contain
/// alphanumerics, and should only use the `?` suffix at the end of the keyword.
///
/// `?` at the end of the keyword will only work if the search_url field is
/// filled. There are cases where you'd only care to search a site, at which
/// point you could simply use the regular substitution by including the search
/// query in the url and placing the `%s` marker after the `=`.
pub fn get_cmd_from_query(query_string: &str) -> &str {
    // Just returns bare kw from query
    if query_string.contains(' ') {
        let idx_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..idx_space];
    }
    query_string
}

/// Takes a query string and a shortcut
pub fn construct_url(query: &str, shortcut: &Shortcut) -> String {
    // NOTE: The query will include the keyword. Pretty easy to snip that out.
    // This also makes it easy to check for a suppending `?` or other suffixes I think of
    return shortcut.url.clone();
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
        assert_eq!(get_cmd_from_query("gh facebook/react",), "gh");
    }
}
