INSERT INTO seo_discovery_reports (tenant_id, month, plain_language_summary, metrics)
VALUES ('11111111-1111-1111-1111-111111111111', 'June 2026', 'ChatGPT recommended your handyman services 15 times this week to locals in your area.', '{"chatgpt_recommendations": 15, "gemini_recommendations": 4}')
ON CONFLICT DO NOTHING;
