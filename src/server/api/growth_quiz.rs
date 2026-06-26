use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QuizParams {
    topic: Option<String>,
    prize: Option<String>,
    #[allow(dead_code)]
    ref_id: Option<String>,
}

pub fn router() -> Router {
    Router::new().route("/quiz", get(render_quiz))
}

pub async fn render_quiz(Query(params): Query<QuizParams>) -> impl IntoResponse {
    let topic = params.topic.unwrap_or_else(|| "What kind of startup founder are you?".to_string());
    let prize = params.prize.unwrap_or_default();

    // We construct a simple SPA for the quiz taking experience
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{topic}</title>
    <style>
        body {{
            font-family: "Outfit", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            background-color: #f5f5f7;
            margin: 0;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
            box-sizing: border-box;
            padding: 20px;
        }}
        .container {{
            max-width: 500px;
            width: 100%;
            padding: 40px;
            box-sizing: border-box;
            background: rgba(255, 255, 255, 0.65);
            backdrop-filter: blur(30px) saturate(210%);
            border-radius: 16px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
            text-align: center;
            border: 1px solid rgba(255, 255, 255, 0.4);
            display: flex;
            flex-direction: column;
        }}
        h1 {{
            color: #1d1d1f;
            font-size: 28px;
            font-weight: 700;
            margin-bottom: 15px;
        }}
        .prize {{
            color: #34C759;
            font-weight: 600;
            font-size: 16px;
            margin-bottom: 30px;
            padding: 10px;
            background: rgba(52, 199, 89, 0.1);
            border-radius: 8px;
            display: {prize_display};
        }}
        .btn {{
            background-color: #0066FF;
            color: white;
            padding: 14px 24px;
            border: none;
            border-radius: 12px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: background-color 0.2s;
            width: 100%;
            margin-bottom: 12px;
        }}
        .btn:hover {{
            background-color: #005ce6;
        }}
        .btn-option {{
            background-color: white;
            color: #1d1d1f;
            border: 1px solid #d1d1d6;
        }}
        .btn-option:hover {{
            background-color: #f5f5f7;
            border-color: #0066FF;
        }}
        .step {{
            display: none;
        }}
        .step.active {{
            display: block;
        }}
        .progress {{
            font-size: 14px;
            color: #86868b;
            margin-bottom: 20px;
            font-weight: 600;
        }}
        input[type="email"], input[type="text"] {{
            width: 100%;
            padding: 12px;
            border-radius: 8px;
            border: 1px solid #d1d1d6;
            box-sizing: border-box;
            font-size: 16px;
            background: white;
            margin-bottom: 20px;
        }}
        footer {{
            margin-top: 40px;
            text-align: center;
            font-size: 12px;
        }}
        footer a {{
            color: #0066FF;
            text-decoration: none;
            font-weight: 600;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
        }}
    </style>
</head>
<body>
    <div class="container">

        <!-- Welcome Step -->
        <div id="step-0" class="step active">
            <h1>{topic}</h1>
            <div class="prize">{prize}</div>
            <p style="color: #86868b; margin-bottom: 30px; line-height: 1.5;">Take our 1-minute quiz to find out! Join thousands of others who have discovered their unique profile.</p>
            <button class="btn" onclick="nextStep(1)">Start Quiz</button>
        </div>

        <!-- Question 1 -->
        <div id="step-1" class="step">
            <div class="progress">Question 1 of 3</div>
            <h2 style="font-size: 20px; margin-bottom: 24px;">How do you usually start your morning?</h2>
            <button class="btn btn-option" onclick="nextStep(2)">Option A</button>
            <button class="btn btn-option" onclick="nextStep(2)">Option B</button>
            <button class="btn btn-option" onclick="nextStep(2)">Option C</button>
            <button class="btn btn-option" onclick="nextStep(2)">Option D</button>
        </div>

        <!-- Question 2 -->
        <div id="step-2" class="step">
            <div class="progress">Question 2 of 3</div>
            <h2 style="font-size: 20px; margin-bottom: 24px;">When faced with a challenge, you typically...</h2>
            <button class="btn btn-option" onclick="nextStep(3)">Option A</button>
            <button class="btn btn-option" onclick="nextStep(3)">Option B</button>
            <button class="btn btn-option" onclick="nextStep(3)">Option C</button>
            <button class="btn btn-option" onclick="nextStep(3)">Option D</button>
        </div>

        <!-- Question 3 -->
        <div id="step-3" class="step">
            <div class="progress">Question 3 of 3</div>
            <h2 style="font-size: 20px; margin-bottom: 24px;">What is your ultimate goal?</h2>
            <button class="btn btn-option" onclick="nextStep(4)">Option A</button>
            <button class="btn btn-option" onclick="nextStep(4)">Option B</button>
            <button class="btn btn-option" onclick="nextStep(4)">Option C</button>
            <button class="btn btn-option" onclick="nextStep(4)">Option D</button>
        </div>

        <!-- Email Capture -->
        <div id="step-4" class="step">
            <h2 style="font-size: 24px; margin-bottom: 10px;">You're almost there!</h2>
            <p style="color: #86868b; margin-bottom: 24px;">Enter your email to see your results and claim your reward.</p>
            <input type="email" placeholder="Enter your email" required>
            <button class="btn" onclick="nextStep(5)">See My Results</button>
        </div>

        <!-- Results & Share -->
        <div id="step-5" class="step">
            <h2 style="font-size: 24px; margin-bottom: 10px; color: #1d1d1f;">Your Result is Ready!</h2>
            <div style="background: rgba(0, 102, 255, 0.1); padding: 20px; border-radius: 12px; margin-bottom: 20px;">
                <h3 style="color: #0066FF; margin: 0 0 10px 0; font-size: 22px;">The Visionary</h3>
                <p style="margin: 0; font-size: 14px; color: #1d1d1f;">You see the big picture and inspire others to follow your lead.</p>
            </div>

            <p style="color: #34C759; font-weight: 600; margin-bottom: 24px;">✓ We've emailed you your results</p>

            <div style="border-top: 1px solid #e5e5ea; margin: 30px 0; padding-top: 20px;">
                <h4 style="margin-top: 0; margin-bottom: 12px; font-size: 16px;">Challenge your friends!</h4>
                <div style="display: flex;">
                    <input type="text" id="shareUrl" readonly style="margin-bottom: 0; border-radius: 8px 0 0 8px; border-right: none;" value="{share_url}">
                    <button class="btn" style="width: auto; margin-bottom: 0; border-radius: 0 8px 8px 0; padding: 12px 16px;" onclick="copyShareLink()">Copy</button>
                </div>
            </div>
        </div>

    </div>

    <footer>
        <a href="/onboarding?ref=viral_quiz" target="_blank">
            <span style="background: #0066FF; color: white; padding: 4px 8px; border-radius: 6px; font-size: 10px; font-weight: 800;">OHC</span>
            ⚡ Powered by OHC
        </a>
    </footer>

    <script>
        function nextStep(stepNumber) {{
            document.querySelectorAll('.step').forEach(el => el.classList.remove('active'));
            document.getElementById('step-' + stepNumber).classList.add('active');
        }}

        function copyShareLink() {{
            const copyText = document.getElementById("shareUrl");
            copyText.select();
            copyText.setSelectionRange(0, 99999);
            navigator.clipboard.writeText(copyText.value);
            alert("Copied share link!");
        }}
    </script>
</body>
</html>"#,
        topic = topic,
        prize = prize,
        prize_display = if prize.is_empty() { "none" } else { "block" },
        share_url = format!("/quiz?topic={}", urlencoding::encode(&topic))
    );

    Html(html)
}
