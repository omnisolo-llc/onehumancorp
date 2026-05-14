#[cfg(test)]
mod tests {
    use crate::miser::get_active_recommendations;

    #[test]
    fn test_recommendation_generation() {
        let recs = get_active_recommendations();
        assert!(!recs.is_empty());
    }
}
