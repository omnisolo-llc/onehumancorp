issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System (Chatwoot Replacement)

  ## Problem Statement
  Currently, Maya the baker and Carlos the handyman struggle to maintain continuous customer conversations across Instagram DMs, website chat, SMS, and WhatsApp without switching between different apps. While an external service like Chatwoot could unify these channels, relying on a third-party application breaks the seamless, single-assistant experience we promise. Owners need a native, lightning-fast "inbox" built directly into the OHC assistant that instantly unifies messages, allows the AI to draft context-aware replies, and works perfectly on their 375px mobile screens, fully offline-tolerant. They shouldn't have to manage another tool or integration.

  ## Research Report
  - **Source Code Benchmarking (Chatwoot):** An audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals key capabilities necessary for an omnichannel engine: data models for Conversations, Inboxes, and Contacts; channel adapters (Web, API, Social); real-time WebSocket messaging; webhooks; macro execution; and AI-assisted agent routing.
  - **Competitor Analysis:**
    - *Shopify Inbox:* Provides native integration of chat directly into the store owner's app with basic automated responses.
    - *Wix Chat / Ascend:* Unifies email, forms, and chat but can feel heavy.
    - *Stripe:* Relies on third-party apps for chat, focusing strictly on payments.
  - **Findings:** A high-performance native chat system is essential for real-time customer support. Building this natively in Rust inside our platform allows for deep multi-tenant data isolation, lower latency, tighter integration with our AI agent swarm (for auto-drafting replies and suggesting next actions), and removing reliance on an external dependency.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "owns"
      Inbox ||--o{ ChannelAdapter : "has"
      ChannelAdapter ||--o{ Conversation : "creates"
      Conversation ||--o{ Message : "contains"
      Conversation }|--|| Contact : "with"
      Message }o--o| AIAgent : "drafted_by"
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Platform
      participant AIAgent
      participant Owner_Mobile
      Customer->>OHC_Platform: Sends message (Insta/Web)
      OHC_Platform->>OHC_Platform: Route to Tenant Inbox via Rust adapter
      OHC_Platform->>AIAgent: Trigger background response draft
      AIAgent-->>OHC_Platform: Save AI draft
      OHC_Platform->>Owner_Mobile: WebSocket push notification
      Owner_Mobile->>OHC_Platform: Owner approves/edits draft
      OHC_Platform->>Customer: Delivers final message
  ```

  ### UI Wireframes & Screen Flow (375px Mobile First)
  - **Screen 1 (Work Triage/Inbox):** A clean unified feed showing active conversations and alerts. Unread messages have a distinct dot. Clean Apple/Ubiquiti-style typography.
  - **Screen 2 (Conversation View):** Standard chat interface. Messages from the customer are left-aligned. Messages from the owner (or AI drafts) are right-aligned. AI drafts appear with a translucent, shimmering glass effect and a clear "Approve & Send" or "Edit" button.
  - **Screen 3 (Customer Context Drawer):** Tapping the customer's name pulls up a bottom sheet showing past orders, current quotes, and customer preferences.

  ### Mobile UX Flow
  1. Owner receives a native push notification of a new message.
  2. Tapping opens the specific conversation on the 375px viewport.
  3. The screen instantly loads the chat history (optimistically loaded from a local cache for offline tolerance).
  4. At the bottom, the AI agent's drafted reply is pre-filled in the text area.
  5. The owner taps "Send" with one thumb, or taps the text box to invoke the native mobile keyboard to edit.

  ### AI Agent Integration Points
  - **Customer Assistant Agent:** Listens for new `Conversation` events. Instantly reads tenant context and past interaction memory to generate a suggested `Message` draft.
  - **Operations Agent:** Monitors the chat for intent (e.g., "I'd like to book a repair") and surfaces a "Create Booking" action card inline within the chat.

  ### Key Design Decisions
  - **Rust Native Backend:** Moving off Chatwoot to a native Rust implementation guarantees low latency, strict multi-tenant row-level security, and unified deployment without extra services.
  - **WebSocket Real-time Layer:** Real-time push is critical for chat. WebSockets will handle live message sync.
  - **Optimistic UI & Offline Tolerance:** The mobile UI will cache conversations using PWA techniques so owners can read and draft replies in poor network conditions (e.g., a food cart in a bad cell zone).

  ## Implementation Prompt
  Implement a native Rust omnichannel inbox service and its corresponding Flutter/Web UI components to replace the external Chatwoot dependency.
  - **User-facing outcome:** The business owner can see, read, and reply to customer messages from multiple channels (web, social) in a single unified mobile feed. AI agents will automatically draft replies that the owner can approve.
  - **CUJ:** The owner logs in on their phone (375px width), sees a new customer message in the inbox, opens the thread, reviews the AI-drafted reply, edits one word, and sends it.
  - **Acceptance Criteria:**
    - Chatwoot integration is entirely removed from the application stack.
    - A new native Rust chat service is implemented supporting Inboxes, Conversations, and Messages.
    - The UI is fully functional on a 375px width.
    - The AI agent successfully drafts a reply upon receiving a new message.
    - Real-time updates work via WebSockets.
    - E2E Playwright tests cover the full owner chat lifecycle.
    - All tests must pass, and the feature must follow strict multi-tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
