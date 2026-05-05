use crate::app;
use slint::{Model, ComponentHandle};

fn create() -> app::VideoTutorials { crate::ui_tests::init(); app::VideoTutorials::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn tutorials_flow_playback() {
    let ui = create();
    ui.set_selected_video_title("How to Scale".into());
    ui.set_is_playing(true);
    assert_eq!(ui.get_selected_video_title(), "How to Scale");
    assert!(ui.get_is_playing());
    ui.set_is_playing(false);
    assert!(!ui.get_is_playing());
}

#[test] fn tutorials_xss_title() {
    let ui = create();
    let xss = "<iframe src=javascript:alert('tutorial')>";
    ui.set_selected_video_title(xss.into());
    assert_eq!(ui.get_selected_video_title(), xss);
}

#[test] fn tutorials_injection_title() {
    let ui = create();
    let inj = "Intro'); DROP TABLE tutorials; --";
    ui.set_selected_video_title(inj.into());
    assert_eq!(ui.get_selected_video_title(), inj);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_selected_video_title() {
    let ui = create();
    ui.set_selected_video_title("Basic Setup".into());
    assert_eq!(ui.get_selected_video_title(), "Basic Setup");
    ui.set_selected_video_title("Advanced Agents".into());
    assert_eq!(ui.get_selected_video_title(), "Advanced Agents");
    ui.set_selected_video_title("Billing Help".into());
    assert_eq!(ui.get_selected_video_title(), "Billing Help");
}

#[test]
fn create_verify_is_playing() {
    let ui = create();
    ui.set_is_playing(true);
    assert_eq!(ui.get_is_playing(), true);
    ui.set_is_playing(false);
    assert_eq!(ui.get_is_playing(), false);
}


#[test]
fn test_video_tutorials_search_filter() {
    let ui = create();
    let models = vec![
        app::VideoMetadata {
            title: "Learn Rust".into(),
            description: "Rust basics".into(),
            duration_sec: 120,
            url: "".into(),
            thumbnail_url: "".into(),
            watched: false,
            category: "Beginner".into(),
        },
        app::VideoMetadata {
            title: "Advanced Slint".into(),
            description: "UI frameworks".into(),
            duration_sec: 300,
            url: "".into(),
            thumbnail_url: "".into(),
            watched: false,
            category: "Advanced".into(),
        }
    ];
    let all_videos = std::sync::Arc::new(std::sync::Mutex::new(models.clone()));
    let vt_weak = ui.as_weak();
    let all_videos_clone = all_videos.clone();
    ui.on_filter_videos(move || {
        if let Some(ui) = vt_weak.upgrade() {
            let query = ui.get_search_query().to_string().to_lowercase();
            let category = ui.get_active_category().to_string();
            let all = all_videos_clone.lock().unwrap();
            let filtered: Vec<app::VideoMetadata> = all.iter().filter(|v| {
                let match_query = query.is_empty() || v.title.to_lowercase().contains(&query) || v.description.to_lowercase().contains(&query);
                let match_category = category.is_empty() || v.category.to_string() == category;
                match_query && match_category
            }).cloned().collect();
            ui.set_videos(slint::ModelRc::new(slint::VecModel::from(filtered)));
        }
    });

    ui.invoke_filter_videos();

    ui.set_search_query("slint".into());
    ui.set_active_category("Advanced".into());
    ui.invoke_filter_videos();

    assert_eq!(ui.get_search_query(), "slint");
    assert_eq!(ui.get_videos().row_count(), 1);
    assert_eq!(ui.get_videos().row_data(0).unwrap().title, "Advanced Slint");
}

#[test]
fn test_video_tutorials_category_change() {
    let ui = create();
    let models = vec![
        app::VideoMetadata {
            title: "Learn Rust".into(),
            description: "Rust basics".into(),
            duration_sec: 120,
            url: "".into(),
            thumbnail_url: "".into(),
            watched: false,
            category: "Beginner".into(),
        },
        app::VideoMetadata {
            title: "Advanced Slint".into(),
            description: "UI frameworks".into(),
            duration_sec: 300,
            url: "".into(),
            thumbnail_url: "".into(),
            watched: false,
            category: "Advanced".into(),
        }
    ];
    let all_videos = std::sync::Arc::new(std::sync::Mutex::new(models.clone()));
    let vt_weak = ui.as_weak();
    let all_videos_clone = all_videos.clone();
    ui.on_filter_videos(move || {
        if let Some(ui) = vt_weak.upgrade() {
            let query = ui.get_search_query().to_string().to_lowercase();
            let category = ui.get_active_category().to_string();
            let all = all_videos_clone.lock().unwrap();
            let filtered: Vec<app::VideoMetadata> = all.iter().filter(|v| {
                let match_query = query.is_empty() || v.title.to_lowercase().contains(&query) || v.description.to_lowercase().contains(&query);
                let match_category = category.is_empty() || v.category.to_string() == category;
                match_query && match_category
            }).cloned().collect();
            ui.set_videos(slint::ModelRc::new(slint::VecModel::from(filtered)));
        }
    });

    ui.set_active_category("Beginner".into());
    ui.invoke_filter_videos();
    assert_eq!(ui.get_active_category(), "Beginner");
    assert_eq!(ui.get_videos().row_count(), 1);
    assert_eq!(ui.get_videos().row_data(0).unwrap().title, "Learn Rust");

    ui.set_active_category("Advanced".into());
    ui.invoke_filter_videos();
    assert_eq!(ui.get_active_category(), "Advanced");
    assert_eq!(ui.get_videos().row_count(), 1);
    assert_eq!(ui.get_videos().row_data(0).unwrap().title, "Advanced Slint");
}

#[test]
fn test_video_tutorials_mark_watched() {
    let ui = create();
    let models = vec![
        app::VideoMetadata {
            title: "Test Video".into(),
            description: "Test".into(),
            duration_sec: 120,
            url: "".into(),
            thumbnail_url: "".into(),
            watched: false,
            category: "Beginner".into(),
        }
    ];

    let all_videos = std::sync::Arc::new(std::sync::Mutex::new(models.clone()));

    let vt_weak_filter = ui.as_weak();
    let all_videos_clone_filter = all_videos.clone();
    ui.on_filter_videos(move || {
        if let Some(ui) = vt_weak_filter.upgrade() {
            let all = all_videos_clone_filter.lock().unwrap();
            ui.set_videos(slint::ModelRc::new(slint::VecModel::from(all.clone())));
        }
    });

    let vt_weak_watch = ui.as_weak();
    let all_videos_clone_watch = all_videos.clone();
    ui.on_mark_video_watched(move |title| {
        if let Some(ui) = vt_weak_watch.upgrade() {
            {
                let mut current_videos = all_videos_clone_watch.lock().unwrap();
                if let Some(v) = current_videos.iter_mut().find(|v| v.title == title) {
                    v.watched = true;
                }
            }
            ui.invoke_filter_videos();
        }
    });

    ui.invoke_filter_videos();

    ui.invoke_mark_video_watched("Test Video".into());

    assert!(ui.get_videos().row_data(0).unwrap().watched);
}

#[test]
fn test_video_tutorials_duration_display_logic() {
    let ui = create();
    let models = vec![
        app::VideoMetadata {
            title: "Short".into(),
            description: "Video".into(),
            duration_sec: 65,
            url: "".into(),
            thumbnail_url: "".into(),
            watched: false,
            category: "Beginner".into(),
        }
    ];
    ui.set_videos(std::rc::Rc::new(slint::VecModel::from(models)).into());
    assert_eq!(ui.get_videos().row_data(0).unwrap().duration_sec, 65);
}

#[test]
fn test_video_tutorials_play_pause_state() {
    let ui = create();
    ui.set_selected_video_title("Video 1".into());
    ui.set_is_playing(true);
    assert_eq!(ui.get_selected_video_title(), "Video 1");
    assert!(ui.get_is_playing());

    ui.set_is_playing(false);
    assert!(!ui.get_is_playing());
}
