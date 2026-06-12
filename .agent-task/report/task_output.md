issue_title: "Implement Agentic Unified Triage Feed for Mobile"
issue_description: |
  **Title**: Implement Agentic Unified Triage Feed for Mobile

  **Problem Statement**:
  Small business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed by notifications scattered across Instagram DMs, SMS, emails, and platform alerts (e.g., "inventory low"). They lack a single place where the most urgent actions are presented with context and a clear "next step." Existing platforms expect the owner to dig through dashboards or a generic inbox to find work. They need a proactive, AI-driven feed that drafts responses and actions so they can simply hit "Approve" from their phone while on the go.

  **Research Report**:
  - **Shopify/Wix**: Both offer "Unified Inboxes" but they are essentially just multi-channel chat aggregators. They don't synthesize system events (e.g., "Customer X who just DMed you also has an abandoned cart from 2 hours ago").
  - **GoDaddy/Squarespace**: Focus heavily on email marketing but lack real-time, cross-channel triage.
  - **The OHC Advantage**: By leveraging the "Event Ingestion Pipeline" and the "Intent & Context Resolution (LLM Layer)", OHC can convert an incoming message into an actionable "Action Card". Instead of just showing the message, the Operations and Customer Success agents collaborate to draft a response and attach relevant actions (e.g., "Send Payment Link"). This saves the owner from having to type out messages on a small mobile keyboard.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    graph TD
        A[Incoming Webhook: IG/SMS/Email] --> B[Event Pipeline / Queue]
        B --> C[Intent Resolution LLM]
        C --> D[RAG Context: Inventory & Policies]
        D --> E[Draft Action/Message]
        E --> F[Agent Feed Database]
        F --> G[Mobile 375px UI: Action Card]
        G -->|User Taps Approve| H[Execute Action & Send Reply]
    ```
  - **UI Wireframes (375px first)**:
    - **Home View**: A scrollable vertical feed of translucent "Action Cards".
    - **Action Card**:
      - **Header**: Customer Name & Intent Tag (e.g., "Maya Smith • Custom Cake Inquiry").
      - **Body**: The customer's message snippet and the Agent's drafted reply in an editable text box.
      - **Footer**: Large, 44x44px touch targets. Primary Button: "Approve & Send". Secondary Button: "Edit". Swipe left to dismiss/archive.
  - **Mobile UX Flow**:
    1. User opens the OHC app.
    2. The default screen is the Unified Triage Feed.
    3. User reviews the top card, reads the AI-drafted reply, and taps "Approve & Send".
    4. The card animates out, and the next most urgent item slides up.
  - **AI Agent Integration Points**:
    - **Triage Agent**: Classifies the incoming message priority.
    - **Customer Success Agent ("The Ambassador")**: Drafts the reply based on the customer's history and current inventory (queried via the Operations Agent).
  - **Key Design Decisions**:
    - Avoid a traditional "inbox" view with read/unread dots. Use a "feed" of actionable items to reduce cognitive load.
    - Ensure the draft is editable but optimized for 1-tap approval to fit the "zero-click automation" philosophy.

  **Implementation Prompt**:
  Implement the user-facing "Agent Feed" on the Flutter/Tauri mobile app layout. The feed should pull from the unified backend event queue and display pending agent-drafted actions as cards.
  - **CUJ**: Maya logs into the app, sees an Action Card for a new IG DM asking about vegan cakes, reads the drafted "Yes, we have them!" reply, and taps "Approve & Send". The message is sent, and the card is cleared.
  - **Acceptance Criteria**:
    - UI must render perfectly on a 375px viewport without horizontal scrolling.
    - Action buttons must be at least 44x44px.
    - Implement a clear loading/pending state when an action is approved.
    - Connect the UI to the real backend Agent Feed endpoints.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
