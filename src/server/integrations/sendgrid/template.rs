use serde_json::Value;

pub fn generate_branded_email(body: &str, toolbox: Option<&Value>) -> String {
    let mut primary_color = "#0071E3".to_string();
    let mut secondary_color = "#f5f5f7".to_string();
    let mut primary_font = "Inter".to_string();
    let mut secondary_font = "Outfit".to_string();

    if let Some(tb) = toolbox {
        if let Some(dna) = tb.get("dna") {
            if let Some(c) = dna.get("primary_color").and_then(|v| v.as_str()) {
                primary_color = c.to_string();
            }
            if let Some(c) = dna.get("secondary_color").and_then(|v| v.as_str()) {
                secondary_color = c.to_string();
            }
            if let Some(f) = dna.get("primary_font").and_then(|v| v.as_str()) {
                primary_font = f.to_string();
            }
            if let Some(f) = dna.get("secondary_font").and_then(|v| v.as_str()) {
                secondary_font = f.to_string();
            }
        }
    }

    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&family={}:wght@400;500;600;700&family={}:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        body {{
            font-family: '{}', sans-serif;
            background-color: {};
            color: #1D1D1F;
            margin: 0;
            padding: 40px;
        }}
        .container {{
            max-width: 600px;
            margin: 0 auto;
            background: #ffffff;
            border-radius: 12px;
            padding: 30px;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
            border-top: 5px solid {};
        }}
        h1, h2, h3 {{
            font-family: '{}', sans-serif;
            color: {};
        }}
        .footer {{
            margin-top: 30px;
            text-align: center;
            font-size: 12px;
            color: #888;
        }}
    </style>
</head>
<body>
    <div class="container">
        {}
        <div class="footer">
            Powered by OneHumanCorp
        </div>
    </div>
</body>
</html>
        "#,
        primary_font.replace(" ", "+"),
        secondary_font.replace(" ", "+"),
        primary_font,
        secondary_color,
        primary_color,
        secondary_font,
        primary_color,
        body
    )
}
