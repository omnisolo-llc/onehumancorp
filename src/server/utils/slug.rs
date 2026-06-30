pub fn slugify(text: &str) -> String {
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }

    let mut slug = String::with_capacity(text.len());
    let mut last_was_dash = true; // Start true to prevent leading dash

    for c in text.chars() {
        if c.is_alphanumeric() {
            slug.extend(c.to_lowercase());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Maya's Cakes & Bakes"), "maya-s-cakes-bakes");
        assert_eq!(slugify("100% Organic!"), "100-organic");
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
}
