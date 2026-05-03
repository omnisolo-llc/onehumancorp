use std::env;
use xcap::Window;

fn main() {
    let windows = Window::all().unwrap();
    for window in windows {
        println!("Window: {}", window.title());
        if window.title().contains("One Human Corp") {
            let image = window.capture_image().unwrap();
            image.save("/home/jules/verification/verification.png").unwrap();
            println!("Screenshot saved!");
            return;
        }
    }
    println!("App window not found.");
}
