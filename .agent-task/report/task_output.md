issue_title: "[Research] Architect the Invisible 'Agent Feed' Protocol"
issue_description: |
  # Research Report: Architect the Invisible 'Agent Feed' Protocol

  ## Problem Statement
  Business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed by notifications from disparate apps (Instagram DMs, Square payments, Shopify inventory). They don't have time to constantly switch context, read dashboards, or execute manual repetitive tasks. They need an assistant that doesn't just notify them, but *proposes the solution* ready for 1-tap approval.

  ## Research Report
  Based on the agent feed deep dive and SMB market report, the core differentiation of OHC is moving from "advising chatbots" to "executing autonomous agents."

  Currently, Shopify Sidekick requires users to navigate to a chat interface and ask questions. OHC's "Agent Feed" flips this paradigm. It proactively monitors system events (webhook ingestions, database changes) and pushes a feed of actionable cards directly to the user's mobile device (375px viewport optimized).

  This addresses the Top 10 SMB Pain Point #3 (Omnichannel Chaos) and #5 (Customer Follow-up) by unifying intents into a single decision stream.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant External as External Source (Stripe/IG)
      participant API as OHC Ingress API
      participant MsgBus as Event Queue
      participant IntentEngine as LLM Intent & Context Engine
      participant FeedDB as Feed Database (Multi-tenant)
      participant UI as OHC Flutter Mobile UI (375px)

      External->>API: Webhook (New DM / Abandoned Cart)
      API->>MsgBus: Publish Event
      MsgBus->>IntentEngine: Consume Event
      IntentEngine->>IntentEngine: Query for business context
      IntentEngine->>FeedDB: Insert Drafted Action Card (Status: Pending Approval)
      FeedDB-->>UI: Real-time sync / Push notification
      UI->>FeedDB: User taps "Approve"
      FeedDB->>API: Execute Action
  ```

  ### Mobile UX Flow
  1. User opens the OHC app and lands directly on the "Today's Feed".
  2. The feed consists of "Action Cards" styling following the Translucent Glass Mandate (macOS style, premium radius).
  3. A card displays:
     - The trigger event (e.g., "Maya asked about vegan cakes on IG").
     - The AI-proposed draft response (e.g., "Yes, we have 3 vegan options left! Link to order: [link]").
  4. Bottom of the card has two large, thumb-friendly buttons (44x44px minimum touch target): "Approve & Send" and "Edit".

  ### Key Design Decisions
  - **Asynchronous Event Driven**: The feed must be decoupled from the event source. Webhooks must return OK immediately and push to a background queue to prevent external timeouts.
  - **Multi-Tenant Isolation**: Every feed item must strictly enforce tenant-level access controls.
  - **Deterministic Execution**: The AI *drafts* the action, but execution must rely on deterministic internal APIs to ensure safety.

  ## Implementation Prompt
  **Goal:** Implement the backend foundation and data layer for the `AgentFeed` capability to enable a proactive, push-based decision queue of "Action Cards".

  **Tasks:**
  1.  Design a multi-tenant persistence layer for agent feed items. The entity must track the originating event, the contextual data payload, the proposed autonomous action drafted by the AI, and the current lifecycle state (e.g., pending approval, executed, dismissed). Ensure strict tenant isolation is enforced at the data access level.
  2.  Create a core service module containing standard CRUD operations for feed items to allow the AI pipeline to enqueue new drafts and users to review them.
  3.  Implement internal API endpoints for mobile clients to securely retrieve a paginated feed and to submit approval decisions for pending items.
  4.  Ensure 100% unit test coverage for the new service layer logic.

  **Acceptance Criteria:**
  - Clients can fetch a correctly paginated list of their own feed items.
  - Clients can update the state of an existing feed item to trigger action execution or dismissal.
  - Strict multi-tenant isolation guarantees that one organization cannot access or modify another organization's feed items.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
