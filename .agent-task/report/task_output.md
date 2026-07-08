issue_title: "Implement Agent Feed Action Cards for iOS-style Approval Workflows"
issue_description: |
  # Research Report: Agent Feed Action Cards for Proactive Approvals

  ## Problem Statement
  Current OHC users face a fragmented notification experience. When an agent (like the Marketing Agent) drafts a social post, or the Operations Agent suggests an inventory restock, the business owner must navigate deep into specific modules to review and approve these actions. This reactive design contradicts our core value of "Invisible Automation" and requires too much cognitive load from non-technical owners on mobile devices.

  ## Research Report (Track 1 & 2)
  - **Competitive Analysis:**
    - **Shopify Sidekick:** Excellent at generating insights but presents them in a conversational chat interface, requiring the user to initiate the dialogue.
    - **Lindy.ai / 11x.ai:** Push specific "approval required" tasks to email or Slack, which is highly effective but lives outside the platform's core operating system.
  - **The OHC Opportunity:** By standardizing the "Action Card" UI pattern in a centralized "Agent Feed," OHC can unify all agent interactions. The owner opens the app and sees a prioritized list of drafted actions (e.g., "Approve Instagram Reply", "Confirm Restock Order") that can be executed with a single tap.

  ## Design Doc (Track 3)
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD;
      Events[System Events: Webhooks, Stock Delta, CRON] --> AgentLayer[OHC Agents: Marketing, Ops, CS];
      AgentLayer -->|Drafts Proposal| FeedDB[(agent_feed_items Table)];
      FeedDB --> API[GraphQL / gRPC API];
      API --> UI[Flutter Mobile App: Unified Agent Feed];
      UI -->|1-Tap Approve| API;
      API --> Execution[Execution Engine: Stripe, Instagram, etc.];
  ```

  ### Mobile UX Flow (375px)
  1. **The Feed:** The primary tab of the OHC mobile app is the "Agent Feed". It consists of vertically scrolling, glassmorphic cards.
  2. **Action Card Anatomy:**
     - **Header:** Agent Identity (e.g., "The Ambassador") & Urgency Indicator.
     - **Context:** Brief summary of *why* this action is needed (e.g., "Customer asked about vegan cake availability").
     - **Draft:** The exact proposed action (e.g., the text of the reply).
     - **Actions:** Prominent, touch-friendly (min 44x44px) buttons: `[Approve & Send]`, `[Edit]`, `[Dismiss]`.
  3. **Interaction:** Tapping "Approve" triggers an optimistic UI update (card slides away), dispatches the action via the backend, and displays a subtle success toast.

  ### AI Agent Integration
  - Agents must serialize their proposed actions into a standard JSON schema that the frontend can reliably render as an Action Card.

  ## Implementation Prompt (Track 4)
  **Feature Name:** Unified Agent Feed Action Cards
  **Target Persona:** Maya the Baker

  **Outcome:** Maya opens her OHC app and sees a card from "The Ambassador" containing a drafted reply to a customer DM. She taps "Approve" and the message is sent.

  **Critical User Journey (CUJ):**
  1. An agent (e.g., Operations Agent) creates a new record in the `agent_feed_items` table with a `PENDING_APPROVAL` state.
  2. The mobile UI fetches the pending items and renders them as Action Cards using the OHC Premium Token design system.
  3. The user taps "Approve" on a card.
  4. The UI optimistic-updates, and the backend processes the approval, delegating the execution back to the originating agent.

  **Acceptance Criteria:**
  - Standardize the `agent_feed_items` schema to support generalized action payloads.
  - Build the mobile-first (375px) Action Card UI component.
  - Implement the API endpoints to fetch, approve, and dismiss feed items.
  - No specific UI framework logic (Flutter/React) is prescribed here, but the 44x44px touch targets and Glassmorphism styling are mandatory.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
