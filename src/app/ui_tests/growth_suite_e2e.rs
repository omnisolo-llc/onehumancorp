use crate::app;
use slint::ComponentHandle;

#[test]
fn test_end_to_end_growth_suite_journey() {
    crate::ui_tests::init();

    // 1. User starts on free tier and sets up website
    let wb_ui = app::WebsiteBuilder::new().unwrap();
    wb_ui.set_plan_tier("Free".into());
    assert_eq!(wb_ui.get_plan_tier(), "Free");

    // WebsiteBuilder has 'Viral Storefront Footer' embedded.

    // 2. User navigates to dashboard and hits limits
    let dashboard_ui = app::Dashboard::new().unwrap();
    let dashboard_handle = dashboard_ui.as_weak();

    dashboard_ui.on_action_add_product(move || {
        if let Some(ui) = dashboard_handle.upgrade() {
            ui.set_show_upgrade_prompt(true);
            ui.set_upgrade_prompt_message("You've reached your free tier limit of 10 products.".into());
        }
    });

    assert!(!dashboard_ui.get_show_upgrade_prompt());
    dashboard_ui.invoke_action_add_product();
    assert!(dashboard_ui.get_show_upgrade_prompt());
    assert_eq!(dashboard_ui.get_upgrade_prompt_message(), "You've reached your free tier limit of 10 products.");

    // 3. User achieves a milestone (10th order)
    dashboard_ui.set_show_milestone(true);
    dashboard_ui.set_milestone_title("🎉 You just got your 10th order!".into());
    assert_eq!(dashboard_ui.get_milestone_title(), "🎉 You just got your 10th order!");

    let milestone_dismissed = std::rc::Rc::new(std::cell::RefCell::new(false));
    let dismiss_clone = milestone_dismissed.clone();
    dashboard_ui.on_dismiss_milestone(move || { *dismiss_clone.borrow_mut() = true; });
    dashboard_ui.invoke_dismiss_milestone();
    assert!(*milestone_dismissed.borrow());

    // 4. User wants to grow, so they refer a friend
    let referrals_ui = app::Referrals::new().unwrap();
    referrals_ui.set_my_referral_link("ohc://join?ref=NOVA_VIP".into());

    let link_copied = std::rc::Rc::new(std::cell::RefCell::new(false));
    let copied_clone = link_copied.clone();
    referrals_ui.on_copy_link(move || { *copied_clone.borrow_mut() = true; });
    referrals_ui.invoke_copy_link();
    assert!(*link_copied.borrow());

    let invite_sent = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invite_clone = invite_sent.clone();
    referrals_ui.on_send_invite_message(move |_| { *invite_clone.borrow_mut() = true; });
    referrals_ui.invoke_send_invite_message("ohc://join?ref=NOVA_VIP".into());
    assert!(*invite_sent.borrow());

    // 5. User shares their business directly
    let share_ui = app::BusinessShare::new().unwrap();
    share_ui.set_business_name("Nova's Emporium".into());

    let x_shared = std::rc::Rc::new(std::cell::RefCell::new(false));
    let x_clone = x_shared.clone();
    share_ui.on_share_to_x(move || { *x_clone.borrow_mut() = true; });
    share_ui.invoke_share_to_x();
    assert!(*x_shared.borrow());

    // 6. User sets up social media auto-posting
    let social_ui = app::SocialPosting::new().unwrap();
    social_ui.set_is_connected_instagram(false);

    let ig_connected = std::rc::Rc::new(std::cell::RefCell::new(false));
    let ig_connect_clone = ig_connected.clone();
    social_ui.on_connect_instagram(move || { *ig_connect_clone.borrow_mut() = true; });
    social_ui.invoke_connect_instagram();
    assert!(*ig_connected.borrow());

    social_ui.set_is_connected_instagram(true);
    assert!(social_ui.get_is_connected_instagram());

    let post_scheduled = std::rc::Rc::new(std::cell::RefCell::new(false));
    let schedule_clone = post_scheduled.clone();
    social_ui.on_schedule_post(move || { *schedule_clone.borrow_mut() = true; });
    social_ui.invoke_schedule_post();
    assert!(*post_scheduled.borrow());

    // 7. User sends an email marketing campaign
    let email_ui = app::EmailMarketing::new().unwrap();
    email_ui.set_total_subscribers(1500);

    let email_sent = std::rc::Rc::new(std::cell::RefCell::new(false));
    let email_sent_clone = email_sent.clone();
    email_ui.on_send_campaign(move || { *email_sent_clone.borrow_mut() = true; });
    email_ui.invoke_send_campaign();
    assert!(*email_sent.borrow());
}
