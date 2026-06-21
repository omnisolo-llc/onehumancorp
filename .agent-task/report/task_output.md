issue_title: "Integrate WhatsApp Business API for Unified Customer Messaging"
issue_description: |
  **Problem Statement**
  Many owners (like Maya the Home Baker and Carlos the Field Service Owner) rely heavily on WhatsApp to interact with customers. Currently, managing conversations across personal/business WhatsApp and OHC leads to fragmented communication, missed leads, and manual data entry. Owners need a single, unified inbox where their assistant can triage messages, remember customer context, and draft replies directly.

  **Research Report**
  *   **Market Need:** WhatsApp is the dominant communication channel for small businesses globally, particularly in LATAM, EMEA, and APAC. Competitors like WeCom, HubSpot, and localized CRMs offer deep messaging integration.
  *   **Tool Capabilities:** The WhatsApp Cloud API supports sending and receiving text, media, and interactive messages (buttons/lists). It uses a webhook model for incoming messages.
  *   **Pricing & SaaS Viability:** WhatsApp charges per conversation (user-initiated vs. business-initiated). Meta provides the Cloud API which avoids hosting the on-premise WhatsApp Business API core, making it highly viable for multi-tenant SaaS environments.
  *   **Ease of Use:** For the non-technical owner, the integration is seamless after initial OAuth/Phone number connection. They manage everything through the OHC interface, avoiding technical setup.

  **Design Doc**
  *   **Integration Points:** OHC connects to the WhatsApp Cloud API via Meta App OAuth. A tenant (owner) links their WhatsApp Business number.
  *   **Triggers:** Incoming webhooks from Meta alert OHC of new messages.
  *   **Actions:** Send text, send media, send approved message templates (e.g., "Your cake order is ready for pickup!").
  *   **User Interface:** Messages appear in the OHC Work Triage feed alongside Instagram DMs and emails. The AI Assistant provides suggested replies based on past customer history, current orders, and knowledge docs. The owner can one-tap approve or edit before sending.

  **Implementation Prompt**
  *   Implement the WhatsApp Cloud API integration to route incoming WhatsApp messages directly into the OHC Work Triage feed.
  *   Enable the Customer & Relationship Assistant to read these messages, retrieve customer context, and draft appropriate replies.
  *   Provide a seamless UI for the owner to review, edit, and send the assistant-drafted replies back through WhatsApp.
  *   Acceptance Criteria:
      1. Incoming WhatsApp messages securely create inbox items in OHC.
      2. The AI assistant automatically drafts a reply based on tenant context.
      3. The owner can send a reply from OHC that is delivered to the customer's WhatsApp.
      4. Webhook verification and idempotency are handled properly.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
