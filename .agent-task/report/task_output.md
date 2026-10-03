outcome: blocked
issue_title: "Architectural Design: Unified Omnichannel AI Inbox & Triage Engine"
issue_description: "The task is blocked. The environment fails to run Playwright E2E tests utilizing PostgreSQL/pgvector Docker images via the isolated Docker environment due to an overlayfs permission error (`failed to convert whiteout file \"etc/alternatives/.wh.pager.1.gz\": operation not permitted`). The acceptance criteria strictly mandate Playwright E2E tests to verify a message flows from ingestion to the UI feed, and that the AI draft is visible and actionable by the owner. Without functioning Docker containers to provide the required test DB fixture, this task is blocked."
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
