issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. We are retiring the external Chatwoot dependency and building a native Rust omnichannel customer support and chat engine inside OHC to achieve 100% feature parity with deep integration into our AI systems.

  # Research Report
  - **Chatwoot Source Code Audit:** Checked out `https://github.com/chatwoot/chatwoot` source code to audit its omnichannel data models, controllers, channels, WebSocket real-time messaging, and inbox architecture.
  - **Competitor Analysis:** Shopify Inbox and Wix Inbox lack deep AI contextual drafting based on omnichannel history. Our solution needs to do proactive RAG based drafting before the owner even opens the app.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy (The Ambassador). This system won't just aggregate; it will resolve identities and draft replies.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Rust Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Mobile Feed:** "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tap opens a unified view showing customer context (e.g., past orders) and an AI-drafted reply.
  - **Action:** Primary button "Send Draft", secondary "Edit".
  - **Visuals:** Glassmorphism cards, blurred backgrounds, Apple/Ubiquiti-style clean hierarchy.

  ### AI Agent Integration Points
  - **The Ambassador:** Triggered by incoming messages. Uses RAG against product catalog and customer history to draft replies.
  - **The Manager:** Checks inventory/calendar if the message implies a booking or order change.

  ### Key Design Decisions
  - **Native Rust Implementation:** High performance, tightly integrated into the OHC monolith.
  - **Proactive Drafting:** Read-approve instead of read-reply.
  - **Identity Resolution:** Crucial for omnichannel context linking.

  # Implementation Prompt
  **User-Facing Outcome:** When a customer DMs a business owner, the owner opens the OHC app to find a pre-written, perfectly accurate response already drafted. Tapping one button sends it.
  **CUJ & Acceptance Criteria:**
  1. Implement a native Rust Omnichannel Gateway that can receive simulated webhooks.
  2. Implement a Customer Identity Resolution Engine to match incoming handles to existing records.
  3. Integrate The Ambassador Agent to query history and draft a reply upon message receipt.
  4. Place the drafted reply in an `ActionRequiredQueue`.
  5. E2E Playwright Tests: A user logs in on a mobile viewport, sees the drafted message, taps "Approve", and the system dispatches it to the mock channel.
  6. 100% test coverage and ensure `bazel test //...` passes.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
