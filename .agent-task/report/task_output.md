issue_title: "Implement Mobile-First Agent Feed (375px)"
issue_description: |
  # Mobile-First Agent Feed Implementation

  ## Problem Statement
  Legacy business platforms like Shopify and Wix relegate the mobile app to a companion dashboard while reserving core operational workflows (inventory, discounts, store design, automated outreach) for desktop environments. This breaks the experience for creators, solopreneurs, and small-business owners (like Maya the baker and Fatima the food cart owner) who run their entire business from their phones.
  Our research (e.g., `docs/business/market_research/ohc_smb_mobile_first_design_research.md` and `docs/business/market_research/agent_feed_deep_dive.md`) highlights that to empower users to manage complex workflows on a 375px screen without clutter, OHC needs a "Unified Agent Feed". Instead of complex menus, the platform should proactively push "Action Cards" (Agent proposals and urgent items) to a vertical feed.

  ## Research Report
  - **The Gap**: Operations are too complex to represent via standard responsive design on mobile. Link-in-bio tools win on simplicity but fail on business depth. Legacy platforms win on depth but fail on mobile simplicity.
  - **The OHC Differentiator**: Instead of traditional UI toggles and tables, we will use Agentic "Approval" UI. An agent classifies intent, queries data, drafts the action (e.g., a promotional email, an Instagram DM reply), and presents a card with a large (minimum 44x44px) "Approve" button.
  - **Core Personas Affected**: Maya (Baker, runs everything from iPhone), Carlos (Handyman, Android phone only), Fatima (Food Cart, limited English, low-end Android).

  ## Design Doc
  **Architecture Overview:**
  - **Event Ingestion**: External/Internal events are routed into a central Agent processing pipeline.
  - **Agent Hub**: The backend creates `AgentCard` instances based on these events (e.g., "Operations: 3 new orders", "Marketing: Post Draft").
  - **UI Shell**: The Tauri/React (or Flutter equivalent if re-added, though canonical is currently Tauri/web) desktop/PWA client displays a feed of these cards.

  **Mobile UX Flow (375px Target):**
  1. Open App -> Home screen is a vertical stack of Agent Action Cards.
  2. Each card is distinct (e.g., color-coded or icon-labeled for Operations, Advisory, Marketing).
  3. Action buttons (e.g., [Fulfill Now], [Yes, draft it], [Approve & Post]) are clearly visible with > 44x44px touch targets.
  4. Expanding a card (e.g., tapping "Yes, draft it") transitions smoothly (using macOS Translucent Glass styling) to show the full drafted content and a final "Approve" confirmation.
  5. The UI must avoid horizontal scrolling entirely.

  **Mermaid Diagram:**
  ```mermaid
  sequenceDiagram
      participant User
      participant AgentFeed_UI
      participant OHC_Backend
      participant LLM_Agent

      OHC_Backend->>LLM_Agent: Webhook/Event (e.g. New DM)
      LLM_Agent->>OHC_Backend: Drafts response & creates Action Card
      OHC_Backend->>AgentFeed_UI: Pushes Card to Unified Feed
      AgentFeed_UI->>User: Displays Card on 375px screen
      User->>AgentFeed_UI: Taps "Approve" (>44px target)
      AgentFeed_UI->>OHC_Backend: Confirms Action
      OHC_Backend->>LLM_Agent: Executes underlying business logic
  ```

  ## Implementation Prompt
  **Role**: Implementer Agent

  **Task**: Build the Unified Agent Feed UI for the OHC mobile experience (targeting 375px viewports) and its backend API endpoints.

  **Requirements**:
  1. **Frontend**: Create a vertical feed layout that consumes an Agent Action Cards API. Use OHC Premium Tokens (translucent glass, clear hierarchy). Ensure no horizontal scroll exists at 375px width. Implement minimum 44x44px touch targets on all interactive elements.
  2. **Backend**: Implement the API endpoints to serve mock or real `AgentCard` payloads to the feed. Ensure multi-tenant isolation via `tenant_id` context.
  3. **Interaction**: Implement the flow where a user can tap an action button on a card (e.g., "Approve") which triggers a state update/API call, transitioning the card to a "Done" or "Processing" state.
  4. **Verification**: Write full Playwright E2E tests executing the Critical User Journey (CUJ): Login -> View Agent Feed -> Tap Approve on a card -> Verify success state. Ensure 100% unit test coverage for new files.

  **Acceptance Criteria**:
  - Feed renders perfectly on 375px viewport.
  - At least 3 types of Agent cards (Marketing, Operations, Advisory) are visually distinct.
  - Playwright E2E tests pass for the core interaction flow.
  - Zero mock data in production (use proper DB seeds for tests).

  **Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
