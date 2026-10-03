INSERT INTO help_articles (tenant_id, category, title, desc_text, link)
VALUES
  ('default', 'Getting Started', 'Understanding the Owner Outcome Feed', 'Learn how to interpret the owner outcome feed, identify pending exceptions, and understand the cost and evidence attached to tasks completed by the AI team.', '/help/outcome-feed-1'),
  ('default', 'Advanced', 'Managing Standing Authority Limits', 'Learn how the AI team executes routine work within standing authority and how to resolve high-impact exceptions requiring owner intervention.', '/help/outcome-feed-2')
ON CONFLICT DO NOTHING;
