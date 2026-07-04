issue_title: "AI Unified Inbox & Omnichannel Customer Memory Graph"
issue_description: |
  # Title: AI Unified Inbox & Omnichannel Customer Memory Graph

  ## Problem Statement
  Small business owners like Carlos (handyman) or Maya (baker) receive customer inquiries across fragmented, unlinked channels (Instagram DMs, WhatsApp, SMS, and email). Managing these manually leads to missed messages, slow response times, and lost sales. Traditional "unified inboxes" merely aggregate messages without context. They force the owner to manually piece together purchase history and type out responses, which is reactive and labor-intensive. OHC needs an autonomous system that proactively links identities and drafts context-aware replies.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies on manual responses or rigid, basic auto-replies. It fails to proactively draft contextual responses leveraging the full customer history across all channels.
  - **Wix Inbox:** Provides good aggregation but AI features are restricted to simple tone improvements or generic replies. It acts as an advisor, not an executor.
  - **Zendesk / Intercom:** Enterprise-grade platforms that are far too complex, overly technical, and expensive for single-person SMBs.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) reads messages, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CUSTOMER : has
      CUSTOMER ||--o{ CHANNEL_IDENTITY : owns
      CUSTOMER ||--o{ MESSAGE : sends_receives
      CUSTOMER ||--o{ ORDER : places
      CHANNEL_IDENTITY {
          uuid id
          string provider
          string external_id
          timestamp last_seen
      }
      MESSAGE {
          uuid id
          string content
          string intent
          string status
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant Omnichannel Gateway
      participant Identity Engine
      participant The Ambassador (Agent)
      participant Action Queue
      participant Owner (Mobile)

      Customer->>Omnichannel Gateway: Sends IG DM "Can I reorder my last cake?"
      Omnichannel Gateway->>Identity Engine: Match IG handle to Customer ID
      Identity Engine-->>Omnichannel Gateway: Returns Customer Context
      Omnichannel Gateway->>The Ambassador (Agent): Trigger reply drafting
      The Ambassador (Agent)->>The Ambassador (Agent): Query past orders & RAG policy
      The Ambassador (Agent)->>Action Queue: Push drafted reply
      Action Queue->>Owner (Mobile): Push Notification (375px)
      Owner (Mobile)->>Action Queue: 1-Tap Approve
      Action Queue->>Omnichannel Gateway: Dispatch response
      Omnichannel Gateway->>Customer: "Yes! Want the vegan chocolate again?"
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Home Feed (Mobile):** The owner opens the app. The top UniFi-style, glassmorphism card shows: "1 New Message from Sarah (Insta DM)".
  2. **Interaction:** Tapping the card opens a unified context view.
     - **Top Half:** Customer context summary (e.g., "Sarah bought a vegan cake 2 months ago").
     - **Bottom Half:** The AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  3. **Actionable Interface:** Prominent primary button "Approve & Send" (>= 44x44px touch target) and a secondary "Edit" button.
  4. **Visual Design:** Premium macOS Translucent Glass materials combined with clean UniFi modular dashboard card layouts.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by the Event Mesh upon new message ingestion. Uses RAG against the tenant's product catalog, store policies, and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the intent of the message involves an order change or booking request, The Manager agent is called first to verify inventory or calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions and Why
  - **Proactive Drafting vs Reactive Chat:** We move from read-and-reply to read-and-approve. The AI does the heavy lifting of context gathering and drafting *before* the user opens the app, saving immense time.
  - **Strict Tenant Isolation:** All customer data, identity graphs, and messages are isolated via `tenant_id` using PostgreSQL Row Level Security (RLS) to ensure zero cross-tenant data leakage.
  - **Omnichannel Identity Graph:** Vital to link various identities (an Instagram handle, an email address) to a single `Customer` entity so context is never lost across platforms.

  ## Implementation Prompt
  **User-Facing Outcome:**
  As an owner (e.g., Maya), when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I simply tap "Approve & Send," completing the interaction in 2 seconds instead of 2 minutes.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook representing an Instagram DM) is ingested by the Omnichannel Gateway.
  2. The Customer Identity Resolution Engine matches the incoming identifier to an existing customer record in the database.
  3. The Ambassador Agent is triggered and queries the customer's past orders and current product catalog.
  4. The Agent generates a contextual draft reply and places it in the ActionRequiredQueue for the specific tenant.
  5. **E2E Playwright Test:** A user logs into the mobile-sized UI (375px viewport), sees the drafted message card on their feed, taps "Approve," and the system dispatches the message back to the mocked external channel.
  6. **Code Quality:** 100% unit test coverage for the Identity Engine and Omnichannel Gateway. All `bazel test //...` run perfectly green.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
