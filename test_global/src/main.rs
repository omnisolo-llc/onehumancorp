use slint::ComponentHandle;
slint::slint! {
    export global UserSettings {
        in-out property <bool> is_advanced: false;
        callback toggle_advanced();
    }
    export component App inherits Window {
    }
}
fn main() {
    let app = App::new().unwrap();
    let global = app.global::<UserSettings>();
    global.on_toggle_advanced(move || {
        println!("toggled");
    });
}
