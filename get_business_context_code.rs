use std::fs;

fn main() {
    let content = fs::read_to_string("src/server/builder/api.rs").unwrap();
    let contains_struct = content.contains("pub struct BusinessContext");
    let contains_deserialize = content.contains("#[derive(Serialize, Deserialize, Clone)]\npub struct BusinessContext");
    println!("Struct exists: {}", contains_struct);
    println!("Derive exists: {}", contains_deserialize);
}
