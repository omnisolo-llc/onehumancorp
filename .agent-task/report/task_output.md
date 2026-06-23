issue_title: "Implement the Unified Agent Feed (Mobile-First Dashboard)"
issue_description: |
  # Unified Agent Feed Implementation

  ## Problem Statement
  Small business owners (Maya the baker, Carlos the handyman) are overwhelmed by complex dashboards with hidden menus. Existing platforms (Shopify, Wix) require users to hunt for actionable information or build complex automations themselves. OHC needs to replace the traditional "read-only dashboard" with a proactive, "action-oriented feed" where AI agents push drafted tasks, customer replies, and business suggestions directly to the user for 1-tap approval. This is the core "Assistant-first" paradigm of OHC.

  ## Research Report
  - **Market Gap:** Shopify Sidekick is a chatbot; it requires the user to initiate the conversation. OHC's agents must be proactive.
  - **Link-in-Bio Success:** Tools like Linktree succeed because they are built purely for mobile. The OHC feed must adopt this 375px-first mentality.
  - **Competitive Advantage:** Instead of a complex UI to configure a discount, the Marketing Agent simply places a card in the feed: "Sales are down 10% this week. Approve a weekend 15% promo code?"
  - **Reference:** See `docs/business/market_research/ohc_smb_mobile_first_design_research.md` and `docs/business/market_research/agent_feed_deep_dive.md`.

  ## Design Doc

  ### Architecture Flow (Mermaid)
  ```mermaid
  graph TD
      A[System Events] --> B(Agent Action Requests Queue)
      C[Customer Inquiries] --> B
      D[Scheduled Summaries] --> B
      B --> E[Unified Feed API]
      E --> F[Mobile Flutter App 375px]
      F --> G{User Reviews Card}
      G -->|Approve| H[Execute Action]
      G -->|Edit| I[Modify & Execute]
      G -->|Dismiss| J[Archive/Ignore]
  ```

  ### Mobile UX Flow (375px)
  1. User opens the OHC PWA/App.
  2. The home screen is *not* a chart; it is a vertical feed of "Action Cards."
  3. Each card has a distinct type (e.g., `MessageReply`, `MarketingPromo`, `InventoryAlert`).
  4. A card displays the AI's drafted intent clearly (e.g., "Drafted reply to Sarah's Instagram DM").
  5. Massive touch targets (min 44px) for primary actions ("Approve & Send", "Approve & Publish").
  6. Premium visual styling using Translucent Glass materials (backdrop-filter) and clean typography.

  ### AI Agent Integration Points
  - **The Feed API** acts as the presentation layer for the `AgentActionRequests` table.
  - Different background agents (The Ambassador, The Promoter) insert records into this table when they have prepared a draft that requires human review.

  ### Key Design Decisions
  - **Feed over Dashboard:** The primary interface is a chronological queue of suggested actions, not a grid of metrics.
  - **1-Tap Execution:** Approving a card must handle all underlying API calls automatically (e.g., creating the Stripe promo code AND scheduling the email).
  - **Offline/Flaky Network Tolerance:** Approvals should be queued locally if the network drops and synced when restored.

  ## Implementation Prompt
  **Outcome:** Build the core API and database schema for the Unified Agent Feed, and a basic frontend component demonstrating the 1-tap approval flow on a mobile viewport.

  **CUJ & Acceptance Criteria:**
  1. A background process (simulated agent) inserts an "Action Required" item into the database (e.g., "Drafted Instagram Reply").
  2. The OHC mobile UI fetches the feed and displays the Action Card.
  3. The user taps "Approve" on the card.
  4. The system processes the approval, executes the mocked action, and removes the card from the active feed.
  5. The UI must flawlessly support a 375px width with no horizontal scroll and use the premium glassmorphic design tokens.
  6. Provide E2E Playwright tests covering the feed rendering, the approval interaction, and the subsequent state change.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
