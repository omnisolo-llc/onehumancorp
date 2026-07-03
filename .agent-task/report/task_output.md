issue_title: "Implement Mobile-First Unified Agent Feed (375px UI & DB Consolidation)"
issue_description: |
  ## Problem Statement
  Legacy business platforms treat mobile apps as supplementary dashboards and require desktop setups for configuring discounts or creating workflows. This breaks the experience for personas like Fatima (food cart) or Maya (baker), who run their entire businesses from their phones. They need complex capabilities (setting up sales, changing inventory, drafting marketing emails) abstracted into single-tap approvals on a 375px screen, rather than form-heavy interfaces.

  ## Research Report
  Our competitive analysis in `ohc_smb_mobile_first_design_research.md` confirms that Link-in-Bio tools succeed via extreme simplicity, while platforms like Shopify falter on mobile due to "The Approval Interface Paradigm" gap. Currently, OHC's backend separates `agent_feed_items`, `agent_approvals`, and `agent_action_requests` in `AgentFeedRepository` but drops contextual payloads when optimizing for mobile.

  Competitors rely on complex desktop admin panels, but OHC must enable complex operations via simple Agent proposed actions surfaced in a unified feed.

  ## Design Doc
  **Architecture Diagram**
  ```mermaid
  graph TD;
      MobileClient[Mobile App 375px View] --> API[Rust API `/api/v1/agent-feed`];
      API --> AgentFeedRepo[Agent Feed Repository];
      AgentFeedRepo --> DB[(PostgreSQL)];
      AgentFeedRepo --> Redis[(Redis Cache)];
      DB -.-> ActionRequests[agent_action_requests];
      DB -.-> Approvals[agent_approvals];
      DB -.-> FeedItems[agent_feed_items];
  ```

  **UI Wireframes & Mobile UX Flow (375px Target)**
  1. **Home Screen / Unified Feed**: A continuous vertical scroll of actionable "Cards". No multi-column layout.
  2. **Card Anatomy**:
     - **Header**: Icon (Agent Dept) + Timestamp.
     - **Body**: Plain language explanation of the event/proposal (e.g., "3 new orders to fulfill", "Drafted promo email").
     - **Action Area**: Full-width primary button (>44x44px touch target) for "Approve/Fulfill" and secondary "Dismiss" button.
  3. **Interaction**: Tapping "Approve" triggers an API call that resolves the state of the agent's task asynchronously.

  **AI Agent Integration Points**
  - Agents (Operations, Marketing, Advisory) asynchronously append structured proposals into the relevant database tables.
  - The API layer unifies these tables into a single timeline response for the client.

  **Key Design Decisions**
  - Enforce the 375px width baseline.
  - OHC Premium Tokens: Use clean, translucent materials for the cards against a simple background.
  - Unify multiple data sources (`agent_feed_items`, `agent_approvals`, `agent_action_requests`) into a singular `MobileAgentFeedItem` structure so the frontend logic remains exceptionally simple and fast.

  ## Implementation Prompt
  **Outcome**: Build a fully functional, mobile-first (375px) Unified Agent Feed UI that fetches and displays agent tasks/proposals from the backend. The backend must be updated to ensure `MobileAgentFeedItem` objects returned by the API contain the necessary `proposed_action` identifiers so the UI can construct action buttons.

  **CUJ**:
  1. User opens the app on a 375px viewport.
  2. The app fetches the unified feed.
  3. The user sees a feed of actionable cards (e.g., "Review Instagram Post").
  4. User taps the primary "Approve" action on a card.
  5. The card enters a loading state, the API confirms the action, and the card transitions to a "Done" state or is dismissed.

  **Acceptance Criteria**:
  - The UI MUST NOT have horizontal scrolling on a 375px viewport.
  - All interactive elements must have at least 44x44px touch targets.
  - The feed correctly merges items from different agent departments.
  - E2E Playwright tests verify the entire flow from opening the feed to approving an action on a mobile-simulated viewport.
  - 100% unit test coverage for any new UI components or backend API modifications.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
