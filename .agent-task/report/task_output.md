issue_title: "Scout: Tool Integration Research - WhatsApp Business API"
issue_description: |
  **Title**: WhatsApp Business API Integration for Unified Inbox

  **Problem Statement**:
  Our owner/operator personas—especially Maya (Home Baker) and Fatima (Food Cart Operator)—rely heavily on direct messages to capture demand, confirm orders, and coordinate pickups. Currently, WhatsApp messages are disconnected from the OHC ecosystem, requiring owners to constantly switch between their personal/business WhatsApp app and OHC. This causes missed leads, untracked deposits, and fragmented customer history, breaking the "One Assistant" promise.

  **Research Report**:
  *   **Market Need**: WhatsApp is the dominant messaging platform in LATAM, EMEA, and parts of APAC. Competitors like Shopify (via Inbox/plugins), Wix, and local POS systems increasingly offer WhatsApp integration.
  *   **Tool Evaluated**: WhatsApp Business API (via direct Meta Cloud API or Twilio wrapper). Twilio provides a highly reliable webhook-based API that abstracts Meta's onboarding, but Meta's direct Cloud API is now much easier and cheaper for multi-tenant SaaS.
  *   **Capabilities & Limits**:
      *   Supports sending/receiving text, images (great for cake/menu photos), and interactive templates (quick replies, buttons).
      *   24-hour customer service window applies: we can freely reply to inbound messages within 24h.
      *   OAuth/Embedded Signup allows a tenant (owner) to connect their own number smoothly.
  *   **SaaS Viability**: Meta Cloud API charges per conversation. We can integrate this such that OHC owners bring their own Meta app setup, or OHC acts as a BSP (Business Solution Provider). For v1, allowing users to plug in a Twilio WhatsApp sender or Meta Cloud API token is viable for standalone and cloud.

  **Design Doc**:
  *   **Triggers**: Inbound webhook from Meta/Twilio.
  *   **Action**: When a WhatsApp message arrives, the webhook handler verifies the signature and drops it into the AI Job Queue. The `Work Triage` capability categorizes it (e.g., Lead, Support, Order). The `Customer Assistant` drafts a reply based on memory/knowledge and presents it in the owner's feed.
  *   **User Visibility**: Owners see WhatsApp messages natively inside the OHC feed, marked with a WhatsApp icon. They can approve AI-drafted replies or type their own. They never have to leave OHC.
  *   **Integration Points**: External API webhook endpoint `POST /webhooks/whatsapp`. Needs tenant-scoped configuration for WhatsApp credentials in the `integrations` table.

  **Implementation Prompt**:
  Implement a WhatsApp Business API integration that connects a tenant's WhatsApp number to their OHC unified feed.
  1. Add a settings section for the owner to connect their WhatsApp account (using Twilio or Meta Cloud credentials).
  2. Create a secure webhook endpoint to receive inbound messages and route them to the OHC work triage pipeline.
  3. Ensure the Customer Assistant can draft replies to these messages, and allow the owner to send outbound replies directly from the OHC interface.
  4. Display WhatsApp messages distinctly in the work feed so the owner knows the channel origin.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
