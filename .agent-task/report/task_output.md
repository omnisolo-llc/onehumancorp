issue_title: "Architectural Upgrade: Unified Omni-Channel AI Work Triage Inbox"
issue_description: |
  # Omni-Channel AI Work Triage Inbox for Owners

  ## Problem Statement
  Currently, owners like Maya (Home Baker) and Carlos (Field Service) are overwhelmed by incoming requests scattered across Instagram DMs, SMS, WhatsApp, emails, and web forms. While OHC has basic inbox endpoints (`src/server/api/inbox`), it lacks a unified, intelligent Triage Engine that aggregates these streams, dedupes customer identities across channels, and allows the AI (Customer Assistant) to draft contextual replies seamlessly. Without this, owners suffer from missed leads and "context switching" fatigue, while competitors like Inbox by Zendesk or HubSpot provide unified (though non-AI-first) experiences.

  ## Research Report
  - **Competitor Landscape**: HubSpot and Zendesk unify channels well but treat AI as an afterthought (sidebar copilot). Shopify Inbox is limited to web/email and lacks deep operational connectivity. WeCom and DingTalk provide unified messaging but are highly corporate and complex.
  - **OHC Architecture Gap**: OHC's current data model silos messages by integration (e.g., Meta, Twilio). The `Customer & Relationship Assistant` needs a unified `UnifiedThread` and `CustomerMemoryGraph` to provide holistic context (e.g., knowing the Instagram DM user is the same person who submitted a quote request last week).
  - **Persona Impact**: Carlos misses out on high-value repair jobs because he can't correlate a text message with an earlier email inquiry. Maya loses track of custom cake preferences discussed across multiple DM sessions.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. The owner opens the OHC app. The default view is the **"Work Triage Feed"**.
  2. The feed presents unified action cards (e.g., "Maya, 3 new inquiries: 2 Instagram DMs for vegan cakes, 1 email for a wedding consultation").
  3. Tapping a card opens a unified thread. The AI has already drafted a response based on the `CustomerMemoryGraph` (e.g., "Hi Sarah, yes we can do a vegan vanilla cake for your wedding. I see you previously ordered our gluten-free cupcakes.").
  4. The owner taps "Approve & Send" or edits the draft inline using the native mobile keyboard.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Channels: IG, SMS, Email] -->|Webhooks| B(Integration Webhook Handlers)
      B --> C[Event Bus / KAIROS Queue]
      C --> D(Omni-Context Routing Engine)
      D --> E[(Customer Memory Graph)]
      D --> F[Unified Triage Database]
      F --> G(Customer Assistant AI Agent)
      G -->|Drafts Reply & Suggests Action| H[Owner Triage Feed UI]
      H -->|Owner Approves| I(Action Execution Engine)
      I --> B
  ```

  ### AI Agent Integration Points
  - **Customer Assistant**: Listens to the `Unified Triage Database`. When a new message arrives, it queries the `CustomerMemoryGraph`, determines intent, drafts a response, and tags the thread urgency.
  - **Operations Assistant**: If the message implies a booking request, it proposes a calendar slot inline within the triage card.

  ### Key Design Decisions
  - **Zero Trust/Multi-Tenant**: The Triage Engine uses strict row-level security (`tenant_id`) on all unified message rows.
  - **Offline Resilience**: The mobile client caches the Triage Feed locally. Approvals made offline are queued and synced using CRDTs when the network is restored.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the backend aggregation layer for the Omni-Channel AI Work Triage Inbox and the corresponding mobile-first UI feed.
  - **User-Facing Outcome**: The owner sees a single feed of prioritized, actionable messages with AI-drafted replies, regardless of the source channel.
  - **CUJ**: An owner logs in on a 375px mobile device. They see a "Triage" card indicating 2 unread messages. They tap it, review an AI-drafted reply to an Instagram DM, tap "Approve", and the system marks the thread resolved.
  - **Acceptance Criteria**:
    1. Create a `UnifiedThread` data model and service that aggregates messages from the existing integration modules.
    2. Implement the AI drafting trigger in the Work Queue.
    3. Build the 375px mobile-first Triage Feed UI, ensuring it follows the premium translucent glass design tokens.
    4. Verify end-to-end flow with a Playwright test mimicking a mobile viewport (approving an AI-drafted IG DM).
    5. Achieve 100% unit test coverage for the new service layer.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
