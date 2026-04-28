use slint::ComponentHandle;

pub fn wire_business_share(ui: &crate::app::Dashboard) {
    let ui_handle = ui.as_weak();

    ui.on_show_share(move || {
        let ui = ui_handle.unwrap();
        let share_window = crate::app::BusinessShare::new().unwrap();

        let share_window_handle = share_window.as_weak();
        share_window.on_copy_link(move || {
            let win = share_window_handle.unwrap();
            let link = win.get_share_link();
            // Just a mock of a cross platform clipboard usage. We don't use direct OS commands for security.
            println!("Mock: copied link {} to clipboard", link);
        });

        let share_window_handle2 = share_window.as_weak();
        share_window.on_share_to_instagram(move || {
            println!("Mock: open url https://instagram.com");
        });

        let share_window_handle3 = share_window.as_weak();
        share_window.on_share_to_whatsapp(move || {
            let win = share_window_handle3.unwrap();
            let link = win.get_share_link();
            println!("Mock: open url https://wa.me/?text=Check+out+my+business!+{}", link);
        });

        let share_window_handle4 = share_window.as_weak();
        share_window.on_share_to_x(move || {
            let win = share_window_handle4.unwrap();
            let link = win.get_share_link();
            println!("Mock: open url https://twitter.com/intent/tweet?text=Check+out+my+business!+&url={}", link);
        });

        share_window.show().unwrap();
    });
}
