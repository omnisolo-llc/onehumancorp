issue_title: "WhatsApp Business Integration (Meta Cloud API)"
issue_description: |
  # Mission Queue Protocol: WhatsApp Business API Integration

  ## Problem Statement
  For owners like **Maya (Home Baker)**, **Carlos (Field Service Owner)**, and **Fatima (Food Cart Operator)**, customer inquiries, order updates, and follow-ups happen largely on WhatsApp, not email. Currently, these messages are siloed on their personal or business phones. They have to manually copy-paste appointment details, quotes, and delivery times between WhatsApp and OHC. When they are busy baking or driving, messages go unanswered, leading to lost revenue. OHC needs to unify WhatsApp conversations into the Work Triage feed so the AI Customer Assistant can auto-draft replies, quote prices, and schedule services instantly without requiring the owner to switch contexts.

  ## Research Report: Meta WhatsApp Cloud API vs Twilio
  I evaluated Meta's native WhatsApp Cloud API against Twilio's WhatsApp API to determine the best integration path for non-technical owner/operators.

  **1. Meta WhatsApp Cloud API (Winner)**
  - **Usability for Owners:** Meta provides "Embedded Signup" flows that allow OHC to let owners connect their WhatsApp Business accounts directly within the OHC app using a simple popup window. There is no need for the owner to create developer accounts or understand API keys.
  - **Pricing:** Free for the first 1,000 user-initiated service conversations per month. After that, it is competitively priced per conversation (varies by region, e.g., ~$0.008 to $0.015 per service message). This is highly viable for a SaaS free-tier.
  - **Capabilities:** Supports rich media, interactive messages (buttons, lists), and template messages (critical for order updates and appointment reminders).
  - **Cloud/Standalone Viability:** Meta Cloud API handles the infrastructure globally. For standalone deployments, users can configure their own Meta App ID.

  **2. Twilio (Runner Up)**
  - **Usability:** Requires the owner to either sign up for Twilio (too technical) or OHC to act as an ISV, which adds significant platform markup and compliance complexity.
  - **Pricing:** Adds a per-message markup on top of Meta's fees, eroding margins for small businesses.

  **Conclusion:** Meta's native Cloud API is the best path. It empowers our AI Assistant to read customer DMs and draft structured, interactive replies seamlessly.

  ## Design Doc
  - **Trigger:** Owner navigates to "Integrations" and clicks "Connect WhatsApp". They complete the Meta embedded signup flow.
  - **Ingestion:** OHC registers a webhook with Meta. Incoming WhatsApp messages hit the OHC webhook and are routed to the appropriate `tenant_id` based on the recipient phone number.
  - **Work Triage:** Messages appear in the OHC feed. The AI Customer Assistant reads the context (and past OHC CRM data) and drafts a contextual reply.
  - **Action:** The owner reviews the AI-drafted reply in OHC and clicks "Send" or configures auto-approval. OHC sends the message via the Meta Graph API back to the customer's WhatsApp.

  ## Implementation Prompt
  Implement the Meta WhatsApp Cloud API integration.
  - **Acceptance Criteria 1 (Setup):** A new non-technical owner can click "Connect WhatsApp" in OHC settings, authenticate via Meta's embedded flow, and map their WhatsApp Business number to their OHC tenant.
  - **Acceptance Criteria 2 (Receive):** When a customer sends a WhatsApp message to the connected number, it appears in the OHC Work Triage feed within 2 seconds.
  - **Acceptance Criteria 3 (Draft & Send):** The AI Assistant can draft a reply to the WhatsApp message, and the owner can approve/send it directly from the OHC UI. The customer receives the message on WhatsApp.
  - **Acceptance Criteria 4 (Interactive):** Support receiving media (e.g., images of repair issues for Carlos) and sending interactive buttons (e.g., for Maya's cake order confirmations).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []