#[cfg(test)]
mod tests {
    use super::super::categorize_error_signal;

    #[test]
    fn test_categorize_security() {
        assert_eq!(categorize_error_signal("security violation detected"), "security");
        assert_eq!(categorize_error_signal("unauthorized access attempt"), "security");
        assert_eq!(categorize_error_signal("sandbox escape"), "security");
    }

    #[test]
    fn test_categorize_bug() {
        assert_eq!(categorize_error_signal("null pointer exception"), "bug");
        assert_eq!(categorize_error_signal("fatal crash in worker"), "bug");
        assert_eq!(categorize_error_signal("unexpected error"), "bug");
    }

    #[test]
    fn test_categorize_feature() {
        assert_eq!(categorize_error_signal("unimplemented feature X"), "feature");
        assert_eq!(categorize_error_signal("missing feature support"), "feature");
    }

    #[test]
    fn test_categorize_refactor() {
        assert_eq!(categorize_error_signal("performance optimization required"), "refactor");
        assert_eq!(categorize_error_signal("legacy code refactor"), "refactor");
    }

    #[test]
    fn test_categorize_cleanup() {
        assert_eq!(categorize_error_signal("memory leak detected"), "cleanup");
        assert_eq!(categorize_error_signal("obsolete handler removal"), "cleanup");
    }

    #[test]
    fn test_categorize_docs() {
        assert_eq!(categorize_error_signal("update readme with new info"), "docs");
        assert_eq!(categorize_error_signal("add documentation for api"), "docs");
    }
}
