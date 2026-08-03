pub mod models;
pub mod websocket;
pub mod api;
pub mod storage;

#[cfg(test)]
mod tests {
    #[test]
    fn test_dummy() {
        assert_eq!(1, 1);
    }
}
