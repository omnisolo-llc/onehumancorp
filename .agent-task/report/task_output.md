issue_title: "Integrate WhatsApp Cloud API for Unified Customer Messaging"
issue_description: |
  # Integrate WhatsApp Cloud API for Unified Customer Messaging

  ## Problem Statement
  Small business owners like Maya (the Home Baker) and Carlos (Field Service) interact heavily with customers on WhatsApp, but these conversations are currently siloed from OHC's operational and quoting flows. Owners have to constantly switch between their personal/business WhatsApp app and OHC. This breaks the "Unified Work Triage" promise, making it hard to track commitments, draft professional quotes from DMs, and assign follow-ups.

  ## Research Report
  ### Market Context & Need
  - WhatsApp is the default communication channel for small businesses in LATAM, EMEA, and SEA, and is growing rapidly in the US for B2C communication.
  - Competitors like Sirena (Zenvia), Trengo, and Wati built entire businesses around this single gap. E-commerce platforms like Shopify have dedicated, highly-rated apps just for WhatsApp notification and chat.
  - Native integration inside OHC would pull the most critical demand channel into the Work Triage feed.

  ### Evaluation of WhatsApp Cloud API
  - **Provider:** Meta's Official WhatsApp Cloud API.
  - **Capabilities:**
    - Bi-directional messaging (text, images, documents, interactive buttons).
    - Webhook events for incoming messages, read receipts, and delivery status.
    - Template messages for proactive outreach (e.g., appointment reminders, order updates).
  - **Limitations/Constraints for non-technical users:**
    - Setting up a Meta Developer account and linking a WhatsApp Business Account (WABA) is historically painful. We must use the **Embedded Signup Flow** to abstract away Meta's complex onboarding.
    - Meta enforces a "24-hour customer service window". If 24 hours pass since the customer's last message, the business can only reply using pre-approved Template messages.
  - **Pricing:** First 1,000 service conversations per month are free. Very viable for small operators. Template messages incur per-conversation costs, which would need to be passed through or bundled in an OHC tier.

  ## Design Doc
  ### How it integrates with OHC
  - **Onboarding (User Settings):** A new "Connect WhatsApp" button in the Settings view triggers Meta's Embedded Signup flow in a popup. OHC captures the resulting identifiers and saves them to the tenant's configuration.
  - **Incoming Messages (Work Triage):**
    - OHC registers a global webhook URL with Meta.
    - Incoming webhooks are routed to the specific tenant based on the destination phone number.
    - Messages are saved as a new interaction entity.
    - If no recent interaction exists, it creates a new item in the owner's Work Triage feed.
  - **Outgoing Messages (Customer Assistant):**
    - When the owner (or the AI on behalf of the owner) replies from the OHC Triage feed, OHC sends a request to the WhatsApp Cloud API.
    - If the 24-hour window has closed, the UI disables the free-form text input and prompts the owner to select an approved "Template" (e.g., "Hi {{1}}, just following up on your quote...").

  ## Implementation Prompt
  1. **Data Model:** Create necessary data structures to securely store WhatsApp connection configurations at the tenant level.
  2. **Webhook Verification:** Implement a public webhook receiving mechanism that verifies incoming data via signatures and securely processes the messages.
  3. **Message Sync:** Process incoming messages into the unified communication domain model. Ensure they appear in the Work Triage UI for the specific tenant.
  4. **Reply Action:** Implement the backend service to send outgoing text messages via the chosen provider API. Expose a unified method for the frontend to call this capability.
  5. **UI Updates:**
     - Add a settings card to connect/disconnect a WhatsApp account using an embedded signup or OAuth-like flow.
     - Update the Work Triage feed to clearly mark messages sourced from this channel.
     - Provide a reply box for WhatsApp messages that warns the user or prevents sending if the 24-hour response window constraint has passed.
  6. **Acceptance Criteria:** A user can connect their WhatsApp (mocked or real), see an incoming message in their Triage feed (simulated or real), and successfully submit a reply that hits the mocked or real outgoing API integration.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
