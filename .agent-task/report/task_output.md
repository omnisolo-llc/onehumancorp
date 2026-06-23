issue_title: "Integrate WhatsApp Cloud API for Direct Customer Messaging and Alerts"
issue_description: |
  **Problem Statement**
  Many non-technical operators (like Maya the home baker or Carlos the field service owner) communicate with their customers entirely through WhatsApp. Currently, owners must switch back and forth between their personal WhatsApp or WhatsApp Business app and OHC to coordinate messages, create quotes, or confirm bookings. This context switching causes missed leads, fragmented customer history, and delays. Owners need OHC to capture incoming WhatsApp inquiries automatically and allow them to reply, send quotes, or confirm bookings directly from the OHC unified inbox without touching their phones.

  **Research Report**
  *   **Tool:** WhatsApp Business Platform (Cloud API), hosted by Meta.
  *   **Need:** In regions like LATAM, India, Europe, and increasingly the US, WhatsApp is the default communication channel for small businesses.
  *   **Alternatives Evaluated:**
      *   Twilio API for WhatsApp (good developer experience but adds middleman markup and complexity).
      *   User-level web scraping/automation tools (unreliable, frequently banned).
  *   **Why Meta's Cloud API?** Meta provides a direct API for businesses to send and receive messages. It offers a free tier (1,000 service conversations per month), which is sufficient for many of our target operators (like Fatima or Leo) to get started. It supports sending rich media, interactive buttons (e.g., "Accept Quote"), and location pins. It is available globally and operates reliably.
  *   **SaaS Viability:** High. OHC can act as a tech provider (using Embedded Signup) to allow multi-tenant cloud users to connect their WhatsApp Business numbers directly. Standalone/local instances can use their own Meta App credentials to connect via the Cloud API.
  *   **Non-Technical Fit:** Once connected, the owner just sees an "Inbox" in OHC. They don't need to know it's powered by an API. They read messages, and OHC's AI can draft replies directly in the chat thread.

  **Design Doc**
  *   **Connection Flow (Settings > Integrations):** The owner clicks "Connect WhatsApp". They are guided through Meta's Embedded Signup flow (or prompted to enter a standalone Access Token in local mode) to link their phone number.
  *   **Incoming Messages (Work Triage/Inbox):** Webhooks from Meta arrive at OHC. The system looks up the customer by phone number, creates a new lead if necessary, and adds the message to the Unified Inbox.
  *   **AI Integration:** The Customer Assistant capability reads the WhatsApp thread and drafts suggested replies or actions (e.g., "Draft a quote for Maya").
  *   **Outgoing Messages:** When the owner approves a draft or types a reply in OHC, the system calls the WhatsApp Cloud API to send the message. OHC also uses WhatsApp template messages to send automated booking confirmations or invoice links if the user opts in.
  *   **Visibility:** Messages are tagged with a WhatsApp icon. Delivery and read receipts are shown to the owner.

  **Implementation Prompt**
  Implement the WhatsApp Cloud API integration.
  1.  **Integrations UI:** Add a new card in the integrations page (Settings) for "WhatsApp Business" that allows the user to connect their account. Use a clear UI state showing if it is "connected" or "disconnected".
  2.  **Inbox Integration:** Update the unified inbox UI to distinctly identify messages sourced from WhatsApp. When a user clicks to reply, the system must support sending plain text back to the WhatsApp channel.
  3.  **Hiding Complexity:** Ensure the setup process explains the benefit (e.g., "Receive and reply to WhatsApp messages directly in OHC") without technical jargon like "Webhooks" or "Cloud API".
  4.  **Acceptance Criteria:** A non-technical user can successfully view a simulated incoming WhatsApp message in their inbox, read the AI-drafted reply, and click "Send" to push the response out, with the UI correctly showing the message as sent via WhatsApp. All UI interactions must be verifiable via Playwright tests.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
