fn main() {
    // SQLx embeds existing SQL files; track the directory as well so adding a
    // migration invalidates a cached executable on stable Rust.
    println!("cargo:rerun-if-changed=src/server/migrations");
}
