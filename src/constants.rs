pub const URL: &str = "https://new-api.udbapp.com";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        assert_eq!(URL, "https://new-api.udbapp.com");
    }
}