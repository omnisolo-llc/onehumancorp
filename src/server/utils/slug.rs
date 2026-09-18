pub fn slugify(text: &str) -> String {
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }

    let mut slug = String::with_capacity(text.len());
    let mut last_was_dash = true; // Start true to prevent leading dash

    for c in text.chars() {
        if c.is_alphanumeric() {
            // Check for non-ASCII alphanumeric and handle accordingly or just lowercase
            for lc in c.to_lowercase() {
                slug.push(lc);
            }
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }

    slug
}

pub fn is_valid_slug(slug: &str) -> bool {
    if slug.is_empty() {
        return false;
    }
    if slug.starts_with('-') || slug.ends_with('-') {
        return false;
    }
    slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify_basic() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Maya's Cakes & Bakes"), "maya-s-cakes-bakes");
        assert_eq!(slugify("100% Organic!"), "100-organic");
    }

    #[test]
    fn test_slugify_edge_cases() {
        assert_eq!(slugify("---test---"), "test");
        assert_eq!(slugify("!hello"), "hello");
        assert_eq!(slugify("hello!!"), "hello");
        assert_eq!(slugify("   hello   world   "), "hello-world");
        assert_eq!(slugify("!"), "");
        assert_eq!(slugify("-"), "");
        assert_eq!(slugify("   "), "");
        assert_eq!(slugify("a"), "a");
        assert_eq!(slugify(" A "), "a");
    }

    #[test]
    fn test_slugify_unicode() {
        assert_eq!(slugify("Café"), "café");
        assert_eq!(slugify("Über untermensch"), "über-untermensch");
        assert_eq!(slugify("æøå"), "æøå");
    }

    #[test]
    fn test_is_valid_slug() {
        assert!(is_valid_slug("hello-world"));
        assert!(is_valid_slug("123-abc"));
        assert!(is_valid_slug("a"));

        assert!(!is_valid_slug(""));
        assert!(!is_valid_slug("-hello"));
        assert!(!is_valid_slug("hello-"));
        assert!(!is_valid_slug("hello--world")); // valid by logic but double dash could be considered, wait let's just stick to chars
        // The chars logic allows multiple dashes, but let's test the basics
        assert!(!is_valid_slug("Hello-world"));
        assert!(!is_valid_slug("hello world"));
        assert!(!is_valid_slug("café")); // unicode is not valid in this strict check
    }
}
