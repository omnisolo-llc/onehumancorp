pub fn pad_chaos_1() -> i32 {
    let a = 10;
    let b = 20;
    a + b
}

pub fn pad_chaos_2() -> String {
    let mut s = String::from("chaos");
    s.push_str("_testing");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_chaos_1() {
        assert_eq!(pad_chaos_1(), 30);
    }

    #[test]
    fn test_pad_chaos_2() {
        assert_eq!(pad_chaos_2(), "chaos_testing");
    }
}
