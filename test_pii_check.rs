fn is_sensitive_key(key: &str) -> bool {
    let k = key.to_lowercase();
    k.contains("password") ||
    k.contains("secret") ||
    k.contains("key") ||
    k.contains("token") ||
    k.contains("auth") ||
    k.contains("cookie") ||
    k.contains("credential") ||
    k.contains("email") ||
    k.contains("phone") ||
    k.contains("ssn") ||
    k.contains("address") ||
    k.contains("name") ||
    k.contains("pii") ||
    k.contains("tenant_id") ||
    k.contains("organization_id") ||
    k.contains("session_id") ||
    k.contains("payload")
}

fn main() {
    println!("{}", is_sensitive_key("user_id"));
    println!("{}", is_sensitive_key("first_name"));
    println!("{}", is_sensitive_key("phone_number"));
}
