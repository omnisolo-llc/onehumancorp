issue_title: "Implement Omnichannel Unified Inbox & Autonomous Inquiry Responder"
issue_description: |
  **Title:** Implement Omnichannel Unified Inbox & Autonomous Inquiry Responder

  **Problem Statement:**
  Small business owners (like Maya the baker and Carlos the handyman) suffer from "Customer Communication Chaos." They lose track of leads and inquiries scattered across Instagram DMs, WhatsApp, SMS, and email. Solopreneurs lose up to 30% of sales simply due to slow response times or forgotten messages. They need a single, centralized inbox where an AI "Silent Ambassador" watches the communication stream, proactively drafts context-aware replies, and presents them for a quick 1-tap approval from their phone's lock screen.

  **Research Report:**
  *   **User Pain Point:** Managing multiple communication apps (Instagram, WhatsApp Business, Email, SMS) causes context switching and dropped leads.
  *   **Competitor Analysis:**
      *   **Shopify:** Requires third-party apps (e.g., Gorgias, Inbox) which have complex setups and lack proactive, deep AI business context (they act as simple chatbots or macro repliers).
      *   **Wix:** Has a unified inbox, but it is passive. It requires the user to manually read and type responses.
      *   **GoDaddy:** Basic messaging integration, but no autonomous agents.
  *   **OHC Advantage:** Shift from an AI *Copilot* (requires prompt) to an AI *Teammate*. The AI watches the event mesh of incoming messages, uses the business's memory (inventory, pricing, policies), drafts the perfect reply, and queues it in an "Action Feed" for a 1-tap approval.

  **Design Doc:**
  **Architecture Diagram**
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
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
  ```

  **UI Wireframes & Mobile UX Flow (375px First)**
  *   **Lock Screen Notification:** "Maya, new IG DM from @john. AI drafted a reply. Tap to review."
  *   **Action Feed Screen (375px):**
      *   A clean, translucent glass card layout (macOS/UniFi style).
      *   **Header:** Customer Name & Channel Icon (e.g., Instagram logo).
      *   **Original Message:** "Hi! Do you make vegan chocolate cakes for this Saturday?"
      *   **AI Draft:** "Hi John! Yes, we have a delicious Vegan Chocolate Fudge cake. We can have it ready for Saturday if you order by tomorrow. It's $45. Shall I send the deposit link?"
      *   **Action Buttons:** Large "Approve & Send" (Primary, Green), "Edit" (Secondary), "Dismiss" (Tertiary).
  *   **Mobile UX Flow:** User receives a notification -> Taps into the Action Feed -> Reads the AI drafted reply -> Taps "Approve & Send". The message is dispatched invisibly through the correct channel.

  **AI Agent Integration Points**
  *   **The Ambassador:** Listens to incoming events on the unified messaging queue.
  *   **Context Fetching:** Before drafting, the agent queries the Catalog (for vegan cakes), Availability (for this Saturday), and Pricing (to quote $45).
  *   **Drafting & Queuing:** Generates the reply and saves it to the Inbox Ledger as `status: PENDING_APPROVAL`.

  **Key Design Decisions**
  *   **Event-Driven Architecture:** Decoupling the ingestion of messages from the processing ensures high availability and fast ingestion even if the AI takes a few seconds to draft a response.
  *   **Human-in-the-Loop (1-Tap):** We do not auto-send messages immediately to prevent hallucination errors. The 1-tap approval builds trust with the business owner.
  *   **Multi-tenant Isolation:** All incoming messages and AI drafts are strictly partitioned by `tenant_id` at the database and messaging queue layers to ensure Zero Trust security.

  **Implementation Prompt:**
  **Context:** We are building the Omnichannel Unified Inbox for OneHumanCorp.
  **Task:** Implement the backend event ingestion, the Unified Inbox data model, and the AI drafting coordination pipeline for The Ambassador.
  **Acceptance Criteria:**
  1. System can ingest messages from at least two test-mode external providers (e.g., IG, SMS).
  2. Incoming messages are strictly isolated by `tenant_id`.
  3. The AI Agent service listens to the ingress queue, fetches real seeded business context, and successfully generates a draft reply.
  4. The draft reply is exposed via an API for the mobile action feed, labeled as `PENDING_APPROVAL`.
  5. An approval mutation successfully dispatches the message back to the test-mode external provider.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
