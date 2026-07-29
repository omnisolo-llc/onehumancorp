issue_title: "Implement Native WhatsApp Business Channel Connector (Rust)"
issue_description: |
  **Problem Statement:**
  For owner/operators like Maya (Home Baker) and Carlos (Field Service), WhatsApp is the primary way customers request custom orders, ask for quotes, and check on service status. Right now, they have to jump between their personal/business WhatsApp app and their other tools, leading to missed messages, lost revenue, and fragmented customer context. They need their WhatsApp messages to flow directly into the OHC Work Triage feed so the Customer Assistant can draft replies, remember customer preferences, and help them take action without leaving OHC.

  **Research Report (Chatwoot Benchmark & Market Need):**
  - **Market Need:** In LATAM, EMEA, and parts of Asia, WhatsApp is the dominant communication channel for small businesses. Competitors like Chatwoot, WeCom, and HubSpot all offer robust WhatsApp integrations because it is mission-critical for lead capture and customer service.
  - **Chatwoot Source Benchmarking:**
    - Evaluated `https://github.com/chatwoot/chatwoot`, specifically `app/models/channel/whatsapp.rb` and the WhatsApp channel webhook controllers.
    - Chatwoot handles WhatsApp through multiple providers (WhatsApp Cloud API, Twilio).
    - Key features identified for replication natively in OHC:
      - Phone number health monitoring.
      - Message template syncing (required for business-initiated messages outside the 24-hour customer service window).
      - Inbound webhook processing for text, media (images/audio), and read receipts.
      - Outbound message delivery.
  - **SaaS Viability:** The official WhatsApp Cloud API is free for the first 1,000 service conversations per month, making it highly accessible for OHC's target small business personas. It operates well in multi-tenant cloud environments via Meta's business portfolio management.

  **Design Doc:**
  - **Integration Trigger:** A customer sends a WhatsApp message to the owner's WhatsApp Business number.
  - **OHC Processing:** The new native Rust WhatsApp connector receives the message via webhook, identifies the OHC workspace (tenant) associated with the destination phone number, and creates or updates a unified customer conversation.
  - **User Experience (Owner View):**
    - The inbound message appears in the owner's Work Triage feed.
    - The Customer Assistant analyzes the message (e.g., a cake inquiry for Maya) and drafts a reply.
    - When the owner approves and sends the reply, the Rust connector dispatches it back to the customer's WhatsApp app.
    - The owner can also send pre-approved WhatsApp message templates to customers to initiate conversations (e.g., order is ready for pickup, or service technician is on the way).

  **Implementation Prompt:**
  - Build a native Rust channel connector for the WhatsApp Cloud API that matches Chatwoot's WhatsApp channel capabilities.
  - Enable owners to connect their WhatsApp Business phone number to their OHC workspace.
  - Implement inbound message handling to turn WhatsApp messages (text, images, audio) into actionable items in the owner's Work Triage feed.
  - Implement outbound message delivery so replies drafted by the owner or AI assistant are sent back to the customer's WhatsApp.
  - Support the syncing and sending of Meta-approved WhatsApp Message Templates to handle notifications outside the standard 24-hour reply window.
  - **Acceptance Criteria:** Maya can link her WhatsApp Business number. A customer messages her on WhatsApp asking about a cake. The message appears in OHC. Maya types a reply in OHC, and the customer receives it on WhatsApp. Maya can also trigger an "Order Ready" template message to the customer the next day.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
