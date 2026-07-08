use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EventRsvpQuery {
    pub tenant: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub location: Option<String>,
    pub theme: Option<String>,
    pub branding: Option<bool>,
}

pub async fn handle_event_rsvp_embed(Query(query): Query<EventRsvpQuery>) -> impl IntoResponse {
    let title = query.title.unwrap_or_else(|| "Summer Pop-up".to_string());
    let date = query.date.unwrap_or_else(|| "Aug 15 @ 12 PM".to_string());
    let location = query.location.unwrap_or_else(|| "Main Street Plaza".to_string());
    let theme = query.theme.unwrap_or_else(|| "light".to_string());
    let branding = query.branding.unwrap_or(true);
    let tenant = query.tenant.unwrap_or_else(|| "DEFAULT".to_string());

    let (bg_color, text_color, card_bg, border_color, input_bg, input_text) = if theme == "dark" {
        (
            "#1f2937",
            "#f9fafb",
            "rgba(255, 255, 255, 0.05)",
            "rgba(255, 255, 255, 0.1)",
            "rgba(0, 0, 0, 0.2)",
            "#f9fafb",
        )
    } else {
        (
            "#ffffff",
            "#111827",
            "#ffffff",
            "#e5e7eb",
            "#f9fafb",
            "#111827",
        )
    };

    let branding_html = if branding {
        format!(
            r#"
            <div style="margin-top: 1.5rem; text-align: center;">
                <a href="https://ohc.app/invite/{}" target="_blank" rel="noopener noreferrer" style="font-size: 0.75rem; color: #9ca3af; text-decoration: none; font-weight: 500; display: inline-flex; align-items: center; gap: 0.25rem;">
                    Powered by <span style="color: #4f46e5; font-weight: 600;">OHC</span>
                </a>
            </div>
            "#,
            tenant
        )
    } else {
        "".to_string()
    };

    let html = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Event RSVP</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Outfit:wght@600;700&display=swap" rel="stylesheet">
    <style>
        body {{
            margin: 0;
            padding: 1.5rem;
            font-family: 'Inter', sans-serif;
            background-color: {bg_color};
            color: {text_color};
            box-sizing: border-box;
            min-height: 100vh;
        }}
        .font-outfit {{
            font-family: 'Outfit', sans-serif;
        }}
        .card {{
            background: {card_bg};
            border: 1px solid {border_color};
            border-radius: 1rem;
            padding: 1.5rem;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
        }}
        h2 {{
            margin: 0 0 1rem;
            font-size: 1.5rem;
            font-weight: 700;
        }}
        .detail {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
            margin-bottom: 0.5rem;
            font-size: 0.875rem;
            font-weight: 500;
            color: {text_color};
            opacity: 0.8;
        }}
        .form-group {{
            margin-top: 1.25rem;
        }}
        .form-group label {{
            display: block;
            font-size: 0.875rem;
            font-weight: 500;
            margin-bottom: 0.5rem;
        }}
        .form-group input {{
            width: 100%;
            padding: 0.75rem;
            border: 1px solid {border_color};
            border-radius: 0.5rem;
            background-color: {input_bg};
            color: {input_text};
            font-size: 0.875rem;
            box-sizing: border-box;
            font-family: inherit;
        }}
        .form-group input:focus {{
            outline: none;
            border-color: #4f46e5;
            box-shadow: 0 0 0 2px rgba(79, 70, 229, 0.2);
        }}
        button {{
            width: 100%;
            margin-top: 1.25rem;
            padding: 0.75rem;
            background-color: #4f46e5;
            color: white;
            border: none;
            border-radius: 0.5rem;
            font-size: 0.875rem;
            font-weight: 600;
            cursor: pointer;
            transition: background-color 0.2s;
            font-family: inherit;
        }}
        button:hover {{
            background-color: #4338ca;
        }}
        .success-message {{
            display: none;
            margin-top: 1rem;
            padding: 0.75rem;
            background-color: rgba(16, 185, 129, 0.1);
            color: #10b981;
            border-radius: 0.5rem;
            font-size: 0.875rem;
            text-align: center;
            font-weight: 500;
        }}
    </style>
</head>
<body>
    <div class="card">
        <h2 class="font-outfit">{title}</h2>

        <div class="detail">
            <svg width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
            {date}
        </div>
        <div class="detail">
            <svg width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
            {location}
        </div>

        <form id="rsvpForm" onsubmit="event.preventDefault(); document.getElementById('rsvpForm').style.display='none'; document.getElementById('success').style.display='block';">
            <div class="form-group">
                <label for="name">Your Name</label>
                <input type="text" id="name" placeholder="John Doe" required>
            </div>
            <div class="form-group">
                <label for="email">Email Address</label>
                <input type="email" id="email" placeholder="john@example.com" required>
            </div>
            <button type="submit">RSVP Now</button>
        </form>

        <div id="success" class="success-message">
            Thanks for RSVPing! See you there.
        </div>

        {branding_html}
    </div>
</body>
</html>
        "#,
        bg_color = bg_color,
        text_color = text_color,
        card_bg = card_bg,
        border_color = border_color,
        input_bg = input_bg,
        input_text = input_text,
        title = title,
        date = date,
        location = location,
        branding_html = branding_html
    );

    Html(html)
}
