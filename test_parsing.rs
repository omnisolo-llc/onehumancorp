use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct TestStruct {
    a: String,
}

fn main() {
    println!("Testing");
}
