issue_title: "[Architecture] Universal Autonomous AI Business Advisory Engine"
issue_description: |
  # Research Report: Autonomous Business Advisory Engine

  ## Summary
  Identified a critical feature gap in how OHC handles business intelligence compared to legacy platforms like Shopify and Wix. SMB owners (Maya, Carlos, Priya) lack the time and expertise to interpret complex analytics dashboards.

  ## Proposed Solution
  Implemented a research design for an asynchronous "AI Business Advisory Engine". This background service aggregates cross-departmental data (sales, inventory, CRM) nightly, using a PG `SKIP LOCKED` job queue. It generates plain-language insights and 1-tap actionable recommendations presented via a mobile-first Glassmorphism UI on the user's dashboard.

  ## Actionable Steps
  - Documented the architecture, data models, and asynchronous AI workflow.
  - Designed the mobile UI flow for a "Daily Briefing" card.
  - Defined the Implementer prompt and acceptance criteria in `docs/research/[architecture]_universal_autonomous_ai_business_advisory_engine.md`.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
