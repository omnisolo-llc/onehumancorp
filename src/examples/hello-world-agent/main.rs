use ohc_builtin_agent::provider::{Provider, BuiltinProvider};

fn main() {
    let provider = BuiltinProvider::new();
    println!("Successfully loaded provider: {}", provider.provider_type());
    println!("Description: {}", provider.description());
    println!("Is Authenticated: {}", provider.is_authenticated());

    println!("Hello World! The agent provider is ready to use with zero configuration.");
}
