use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

use self::config::Config;

pub mod config;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// Keywords should never be more than one word, should only contain
/// alphanumerics, and should only use the `?` suffix at the end of the keyword.
///
/// `?` at the end of the keyword will only work if the search_url field is filled.
pub fn get_cmd_from_query(query_string: &str) -> &str {
    // Just returns bare kw from query
    if query_string.contains(' ') {
        let idx_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..idx_space];
    }
    query_string
}

/// Takes a query string and a shortcut
pub fn construct_url(query: &str, config: &Config) -> String {
    let mut kw = get_cmd_from_query(query).to_string();
    let query_contents = query.strip_prefix(&format!("{} ", &kw));
    let has_search_suffix = kw.ends_with('?');
    if has_search_suffix {
        kw.pop();
    }

    if let Some(shortcuts) = &config.shortcuts.as_ref() {
        // TODO: Implement this all with a hashmap so this for loop can go.
        // NOTE: This will also declutter how the config structs are handled...
        // NOTE: And make implementing cmd and reload features easier
        for sc in shortcuts.iter() {
            if sc.keyword == kw && has_search_suffix {
                if query_contents.is_none() {
                    eprintln!("Cannot use search suffix `?` with an empty query")
                } else {
                    return format!(
                        "{}{}",
                        sc.search_url.clone().expect(&format!("Search suffix `?` does not work for shortcut `{}` because the `search_url` field was not added", sc.keyword)),
                        utf8_percent_encode(query_contents.unwrap(), FRAGMENT).to_string()
                    )
                    .to_string();
                }
            } else if sc.keyword == kw && !has_search_suffix {
                if query_contents.is_some() {
                    return format!(
                        "{}{}",
                        &sc.url,
                        utf8_percent_encode(query_contents.unwrap(), FRAGMENT).to_string()
                    )
                    .to_string();
                }
                return sc.url.clone();
            }
        }
    }
    return format!(
        "{}{}",
        &config.default_search,
        utf8_percent_encode(&query, FRAGMENT).to_string()
    );
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
