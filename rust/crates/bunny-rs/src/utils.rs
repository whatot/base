use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn get_command_from_query_string(query_string: &str) -> &str {
    if let Some(index) = query_string.find(' ') {
        &query_string[..index]
    } else {
        query_string
    }
}

pub fn utf8_encode(query: &str) -> String {
    utf8_percent_encode(query, FRAGMENT).to_string()
}

#[cfg(test)]
mod test {
    use super::get_command_from_query_string;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let actual = get_command_from_query_string("query 991281j");
        let expected = "query";
        assert_eq!(actual, expected);
    }
}
