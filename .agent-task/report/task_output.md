issue_title: "🔍 Scout: Tool Integration Research - WhatsApp Cloud API"
issue_description: |
  **Problem Statement**
  For many non-technical owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart), WhatsApp is the primary operating system for their business. Customers send inquiries, place orders, and ask for support via WhatsApp. Managing this volume manually leads to missed orders, delayed replies, and scattered context. OHC needs a way to seamlessly connect to their WhatsApp Business accounts so the assistant can triage incoming requests, draft replies, and organize daily tasks without the owner ever leaving the OHC interface or juggling apps.

  **Research Report**
  *   **Tool:** WhatsApp Business Platform (Cloud API) by Meta.
  *   **Market Need:** Dominant messaging platform in LATAM, India, Europe, and parts of Africa. It is heavily utilized by SMBs globally. Competitors like Shopify App Store, Wix, and HubSpot have highly-rated integrations for WhatsApp.
  *   **Ease of Use:** For the end-customer, the experience is completely native. For the owner, using Meta's Embedded Signup flow removes the need for technical setup (no manual API key copy-pasting).
  *   **Pricing:** Meta provides 1,000 free service conversations per month. After that, businesses are charged per conversation (utility, marketing, service). This fits perfectly with OHC's small-business focus, as the free tier covers many early-stage users.
  *   **SaaS Viability:** Excellent for multi-tenant cloud environments. OHC can act as a Tech Provider, managing webhooks centrally and routing messages to the correct tenant (owner workspace) securely.

  **Design Doc**
  *   **Integration Trigger:** The owner links their WhatsApp Business number to OHC via Meta's Embedded Signup flow.
  *   **Action/Data Flow:**
      *   Meta sends webhooks to OHC for incoming WhatsApp messages.
      *   OHC routes the webhook to the correct tenant.
      *   **Work Triage** categorizes the message (e.g., "new cake inquiry", "service delay complaint") and surfaces it in the daily work feed.
      *   **Customer Assistant** evaluates the context and drafts a suggested reply.
      *   The owner reviews the drafted reply in OHC and taps "Approve."
      *   OHC sends the response back via the WhatsApp Cloud API.
  *   **User Experience:** The owner sees WhatsApp messages flowing directly into their unified OHC command center. Complex technical details like Meta templates, 24-hour service windows, and webhook validation are entirely handled by OHC behind the scenes.

  **Implementation Prompt**
  *   Implement a webhook endpoint to receive and validate incoming messages from the WhatsApp Cloud API.
  *   Map incoming text, image, and audio messages to the OHC unified inbox, ensuring tenant isolation.
  *   Implement an Embedded Signup or OAuth-like flow so owners can connect their WhatsApp Business account without technical friction.
  *   Provide a unified UI in OHC where owners can read WhatsApp messages and approve AI-drafted replies.
  *   Ensure the AI Assistant is context-aware of the WhatsApp channel (e.g., adhering to the 24-hour reply window limit).

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []