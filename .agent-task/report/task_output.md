issue_title: "Implement 'AI as a Service' (AIaaS) Core Capabilities for OHC"
issue_description: |
  ## Problem Statement
  Small Business Owners lack the time and expertise to manage complex AI integrations.
  Existing platforms (Shopify, Wix) treat AI as bolted-on chat widgets or disjointed tools.
  OHC needs to make AI an invisible, automated service running continuously in the background
  across all business operations (Operations, Marketing, Sales, CS, Finance, Legal, Advisory).

  ## Research Report
  - **Market Context**: The `ohc_smb_market_report.md` states: "SMBs cannot afford dedicated marketing agencies. They are turning to fragmented AI tools (ChatGPT, Midjourney) but struggle to integrate them into a cohesive workflow. OHC must provide 'AI-in-the-loop' workflows where the AI drafts content and the user simply approves it with one tap."
  - **Competitive Landscape**: "legacy platforms like Shopify and Wix... core architecture was designed for a 'desktop-first, web-store-first' era, adapting to ai as a service (aiaas) for smbs requires complex workarounds or expensive third-party plugins. OHC's agentic architecture allows us to address this natively."

  ## Design Doc
  - **Architecture**:
    - Build core `AIAgentPersona` abstractions and `AIaaSWorkflow` definitions within the Go backend.
    - Define a generic schema for AI tasks (e.g., Drafting a social post, replying to a customer).
    - Expose endpoints via `Teammate Mesh` for the front-end to trigger and display 'one-tap approvals'.
    - Use `pgvector` for contextual memories specific to the tenant and persona.
  - **UI/UX**:
    - Translucent glass dashboard elements popping up notifications for AI drafts.
    - "One-Tap Approve" or "Regenerate" buttons on 375px mobile viewports using Flutter.

  ## Implementation Prompt
  - Create the backend Go services for `AIaaS`.
  - Expose API endpoints for managing AI workflows and awaiting approvals.
  - Update the Flutter UI to include a unified "AI Inbox" or context-aware popups for one-tap approvals.
  - Ensure everything works end-to-end starting from user login and triggering a background AI task.
  - Test via Playwright E2E and 100% unit test coverage.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
