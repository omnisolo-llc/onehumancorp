issue_title: "Integration: Twilio WhatsApp Business API for unified customer communications"
issue_description: |
  **Mission Queue Protocol Brief:**

  **Title:** Integration: Twilio WhatsApp Business API for unified customer communications

  **Problem Statement:**
  Small-business owners like Maya (Home Baker) and Carlos (Field Service Owner) frequently communicate with customers via WhatsApp. Currently, these messages exist in a siloed mobile app, forcing owners to constantly switch contexts between their primary messaging tool and their OHC workspace. This leads to missed leads, delayed responses, and fragmented customer histories. An owner needs all WhatsApp inquiries, bookings, and updates centralized within the OHC Assistant feed to maintain context and draft rapid AI-assisted replies.

  **Research Report:**
  *   **Discovery Track:** Through community mining (Reddit, Trustpilot) and analyzing competitor platforms (Tencent Workbuddy, WeCom, DingTalk, Wix), WhatsApp is consistently identified as the primary communication channel for SMBs in many global markets, especially LATAM, India, and parts of Europe.
  *   **Tool Deep-Dive (Twilio API for WhatsApp):** Twilio offers a robust, developer-friendly API wrapper around the official WhatsApp Cloud API.
      *   **Capabilities:** Send/receive free-form messages, template messages (essential for notifications outside the 24-hour window), media (images, PDFs), and location data. It provides reliable webhooks for incoming messages and delivery statuses.
      *   **Pricing:** Twilio operates on a pay-as-you-go model per conversation. Inbound user-initiated conversations are often free or very low cost, making it viable for free-tier/low-volume SMBs, while scaling predictably for larger operations.
      *   **SaaS Viability:** Fully viable for Cloud (multi-tenant) via API keys. For Standalone modes, the owner would need to provide their own Twilio credentials.
      *   **User-First Value Mapping:** For Maya, a customer messaging "How much for a custom cake?" on WhatsApp instantly appears in her OHC Work Triage feed. The Customer Assistant drafts a reply based on her pricing PDF, and she approves it with one tap. She never has to open the standalone WhatsApp app.

  **Design Doc:**
  *   **Integration Points:**
      *   **Work Triage:** Incoming Twilio webhooks are mapped to OHC tasks/messages and surfaced in the prioritized owner feed.
      *   **Customer & Relationship Assistant:** Maintains message context, links WhatsApp threads to specific customer profiles, and drafts replies.
      *   **Operations Assistant:** Can trigger outbound WhatsApp template messages for order confirmations or service reminders.
  *   **User Experience:** The owner connects their Twilio account via a straightforward OAuth or API key setup in an "Integrations" panel. Once connected, WhatsApp messages look and feel like any other unified message in the OHC shell. They can reply directly from the shell, and AI drafts are suggested inline.
  *   **Architecture considerations:** Requires secure storage of Twilio credentials (API Key/Secret or Auth Token), webhook ingestion endpoints (handling Twilio's specific signature validation), and background workers to process incoming messages without blocking the webhook response.

  **Implementation Prompt:**
  *   **Outcome:** Build the Twilio WhatsApp integration so that owners can receive and reply to WhatsApp messages directly within the OHC assistant interface.
  *   **Acceptance Criteria:**
      1.  Owners can configure a Twilio integration (providing Account SID, Auth Token, and WhatsApp sender number).
      2.  Incoming WhatsApp text messages trigger webhooks that create unified message entries in the OHC UI, attached to the correct customer profile (matched by phone number).
      3.  Owners can send replies from the OHC UI, which are delivered via the Twilio WhatsApp API.
      4.  The AI Assistant can read the history of these WhatsApp messages to provide context for new drafts.
      5.  Include E2E tests simulating incoming webhooks and outbound message generation.

  **Priority:** P1 (High - core communication channel for target personas)
  **Estimated Scope:** Large (involves credential management, webhook handling, UI updates, and AI context integration)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
