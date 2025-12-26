use anyhow::{Error, Result};
use url::Url;

pub fn get_bucket_prefix(url: &str) -> Result<String> {
    let parsed_url = Url::parse(url)?;
    let path = parsed_url.path();

    // Remove leading slash if present
    let path = path.strip_prefix('/').unwrap_or(path);

    // Split path into segments and remove the last segment (filename)
    let segments: Vec<&str> = path.split('/').collect();
    if segments.len() <= 1 {
        return Err(Error::msg("URL path too short to extract prefix"));
    }

    // Join all segments except the last one
    let prefix = segments[..segments.len() - 1].join("/");

    Ok(prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bucket_prefix() {
        let url = "https://example.org/archive/alst/directory_name/index.md";
        let prefix = get_bucket_prefix(url);
        assert!(prefix.is_ok());
        assert_eq!(prefix.unwrap(), "archive/alst/directory_name");
    }
}
