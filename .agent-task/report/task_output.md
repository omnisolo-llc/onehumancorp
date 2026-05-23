issue_title: "[Architecture] Autonomous Social Ad Campaign Engine"
issue_description: |
  # Autonomous Social Ad Campaign Engine

  ## Problem
  Small business owners struggle with complex ad platforms (Facebook Ads, TikTok Ads). They need a zero-config way to spend a small budget to get local traffic or messages, without learning about lookalike audiences or pixels.

  ## Solution
  An AI-driven ad engine that takes a merchant's intent ("spend $50 to sell more cakes"), automatically generates ad creatives using their existing catalog, targets local audiences, and continuously optimizes the budget.

  ## Key Components
  - Conversational mobile UI for setting budget and goal.
  - Marketing Agent for creative generation and budget pacing.
  - Background jobs to sync with Meta/TikTok APIs and auto-pause ads if inventory runs out.
  - See `docs/research/[architecture]_autonomous_social_ad_campaign_engine.md` for full design and architecture.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []