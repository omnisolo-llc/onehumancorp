issue_title: "Implement Unified Agent Feed (Mobile MVP)"
issue_description: |
  ## Core Vision
  The Unified Agent Feed is the mobile-first (375px) core component of OHC platform that brings the "Invisible AI Automation" vision to life.
  Instead of navigating complex menus, users will see a central nervous system feed for their business. This feed proactively pushes critical updates, suggested actions, and drafted communications directly for review and approval.

  ## Problem Statement
  Currently, managing complex business tasks like drafting promo emails or resolving customer inquiries (e.g. Maya the Baker answering Instagram DMs on stock availability) requires navigating specialized modules or using external tools, creating a highly fragmented experience that is heavily biased towards desktop interfaces.
  The absence of an integrated, automated agent feed means that business owners have to rely on traditional, manual configuration tools rather than an AI assistant that actively drafts, proposes, and waits for user approval.

  ## Research Report
  Our competitive analysis in `docs/business/market_research/ohc_smb_mobile_first_design_research.md` clearly reveals the limitations of legacy platforms (Shopify, Wix), which use a companion-app model optimized for viewing analytics but not for editing or complex business management.
  On the other hand, simple link-in-bio tools are highly optimized for mobile devices but lack the backend features for managing inventory or complex workflows.

  To bridge this gap, OHC's unique differentiator is the Agentic Workflow—we must implement a "Unified Agent Feed". Instead of users piecing together tools (the "App Tax"), the AI acts as a department worker that proposes actions via simple, touch-friendly Action Cards. The user only needs to hit one big "Approve" button.
  Reference: `docs/business/market_research/agent_feed_deep_dive.md` & `docs/business/market_research/agentic_autonomous_website_builders_smb_platform_gap_analysis.md`.

  ## Design Doc
  - **Architecture Diagram (Mental Map)**:
      [Webhook/Event (Stripe, Instagram)] --> (Event Ingestion Pipeline) --> (Intent & Context Resolution via LLM, e.g., MiniMax/Gemini) --> [Action Card generated in DB (Agent Feed Item)] --> [Pushed via API to OHC Mobile App/PWA Dashboard]
  - **Mobile UX Flow (375px First)**:
    1.  **Dashboard/Feed View**: The first screen after login displays a vertical list of cards.
    2.  **Action Cards**: Each card corresponds to a pending task (e.g., an Ops card for fulfillment, an Advisory card for drafting a promo, or a Marketing card for responding to an Instagram DM).
    3.  **Interaction Design**: Each card features a clear, descriptive title/message and prominent actionable buttons (e.g., "Approve", "Approve & Send", "Yes, draft it", "Dismiss"). The touch targets must be at least 44x44px to comply with accessibility standards. The layout will strictly adhere to the 375px width, utilizing OHC Premium Tokens (Glassmorphism, crisp typography).
    4.  **Handoff/Approval**: Tapping an action button dismisses the card and sends an API request to execute the intent or advance the workflow state.

  ## AI Agent Integration Points
  - The `AgentFeedService` (backend) is responsible for processing events, calling the LLM to generate draft actions or responses based on contextual business data, and saving them as pending tasks in the database (`AgentFeedItem` and `AgentApproval`).
  - The Frontend feed consumes these items and displays them as AI-generated Action Cards.

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Develop the "Unified Agent Feed" mobile MVP on the frontend.
  **User-Facing Outcome**: When a user logs in, their dashboard should present a mobile-optimized vertical feed of "Agent Proposals" (Action Cards).
  **Critical User Journey (CUJ)**:
  - User opens the app on a 375px mobile screen.
  - The user sees a feed displaying Action Cards from various departments (Operations, Marketing, Advisory).
  - Example cards: "3 new orders to fulfill", "Draft promo email?", "Do you make custom vegan cakes?" (Instagram DM draft reply).
  - User taps the primary action button on a card (e.g., "Approve", "Yes, draft it!", "Approve & Send") or the secondary "Dismiss" button.
  - The card disappears from the feed.

  **Acceptance Criteria**:
  - The layout must fit perfectly in a 375px viewport with no horizontal scrolling.
  - All interactive buttons must have a minimum touch target size of 44x44px.
  - The UI must reflect OHC's premium visual style (Glassmorphism, defined typography).
  - The E2E tests in `src/e2e/unified_agent_feed.spec.ts` must pass successfully, validating card visibility and the dismiss functionality upon interaction.
  - Ensure zero mock data is present in the final implementation; the frontend should fetch the real Agent Feed data from the backend.

  **Priority**: P0
  **Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []