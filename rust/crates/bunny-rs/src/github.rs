use crate::utils;

pub fn construct_github_url(query: &str) -> String {
    if query == "gh" {
        "https://github.com".to_string()
    } else {
        format!("https://github.com/{}", utils::utf8_encode(&query[3..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_profile_url_with_gh() {
        let fake_query = "gh";
        assert_eq!(construct_github_url(fake_query), "https://github.com");
    }

    #[test]
    fn test_construct_github_profile_url_with_repo_url() {
        let fake_query = "gh facebook";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/facebook"
        );
    }

    #[test]
    fn test_construct_github_search_url_with_repo_url() {
        let fake_query = "gh facebook/docusaurus";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/facebook/docusaurus"
        );
    }
}
