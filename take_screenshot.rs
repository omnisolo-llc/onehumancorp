fn main() {
    let status = std::process::Command::new("node")
        .arg("scripts/capture_screenshots.js")
        .status()
        .expect("Failed to execute run-playwright.mjs");

    if status.success() {
        println!("Screenshots generated via Playwright E2E tests.");
    } else {
        eprintln!("Failed to run Playwright E2E tests.");
        std::process::exit(1);
    }
}
