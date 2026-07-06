issue_title: "Integrate WhatsApp Business API (via Twilio) for Work Triage"
issue_description: |
  ### Track 1: Dynamic Integration & Market Need Discovery
  **Ecosystem Scraping:** An audit of competitor platforms (WeCom, Shopify Inbox, HubSpot) reveals that omnichannel messaging—especially WhatsApp—is the most installed and requested integration for small businesses globally. In LATAM, EMEA, and parts of APAC, WhatsApp *is* the internet for small businesses.

  **Community Mining:** Subreddits like r/smallbusiness and r/ecommerce show owners constantly struggling to separate personal WhatsApp messages from business inquiries. This results in lost leads, forgotten custom orders, and an inability to delegate inbox management to staff or AI agents without handing over a personal phone.

  **Integration Target:** Twilio WhatsApp Business API. It provides a robust, scalable webhook-based approach that can seamlessly feed into OHC's "Work Triage" agent.

  ### Track 2: Selected Tool Deep-Dive Evaluation
  **User-First Value Mapping:**
  - **Persona Focus:** Maya (Home Baker) & Carlos (Field Service Owner).
  - **The Benefit:** Maya currently receives custom cake orders via WhatsApp DMs on her personal phone, often missing messages when baking. With this integration, Maya connects her business number to OHC. Customers message her on WhatsApp, but Maya reads and replies within the OHC Assistant feed. The OHC Customer Assistant can auto-draft replies for pricing, ask for deposit links, and create "tasks" out of cake orders—all without Maya touching her phone's native WhatsApp app.

  **Capabilities & Limits:** Twilio’s API abstracts Meta's complex onboarding. It provides reliable webhooks for incoming messages, media support (images of cakes/repairs), and template messages for 24hr+ follow-ups. Webhook latency is minimal. One limit is the 24-hour customer service window imposed by Meta, requiring pre-approved templates for outgoing messages after 24 hours.

  **SaaS Viability:** Pricing is per-conversation (utility-based), which is highly scalable for Cloud multi-tenant. Twilio supports Subaccounts, which maps perfectly to OHC's `tenant_id` model. Standalone deployments can simply provide their own Twilio Account SID and Auth Token.

  ### Track 3: Strategic Integration Dispatch (Issue Brief)

  **Title**: Integrate Twilio WhatsApp API for Unified Inbox and Agent Triage

  **Problem Statement**:
  Owners like Maya and Carlos run their businesses via WhatsApp, but managing everything from a mobile app leads to dropped leads, forgotten follow-ups, and an inability to collaborate. They need their WhatsApp messages to flow directly into OHC's Work Triage, where AI agents can draft replies and extract actionable tasks (quotes, bookings).

  **Research Report**:
  - WhatsApp is the primary communication channel for SMBs outside the US, and growing rapidly within it.
  - Native WhatsApp Business app lacks team collaboration and API access for AI agents.
  - Twilio provides a stable REST API and Webhooks to send/receive WhatsApp messages.
  - Required features: Inbound message handling (text/images), outbound replies within the 24-hour session window, and mapping phone numbers to OHC customers.

  **Design Doc**:
  - **Trigger/Input**: A Twilio webhook receives an incoming WhatsApp message and routes it to an OHC webhook endpoint.
  - **Processing**: The system identifies the `tenant` based on the Twilio Subaccount or configured phone number, creates/updates a Customer record based on the sender's phone number, and posts the message to the tenant's Work Triage feed.
  - **Agent Action**: The Customer Assistant is triggered to summarize the context and generate a drafted reply or suggest an action (e.g., "Create a quote for this cake request").
  - **User Output**: The owner sees the message in their OHC command center and can tap "Send Draft" to push the reply back to the customer's WhatsApp via the Twilio API.

  **Implementation Prompt**:
  Implement the Twilio WhatsApp webhook receiver and outbound sender. The non-technical owner should be able to connect their Twilio account (or use a platform-provided one), see incoming WhatsApp messages appear instantly in their OHC Work Triage feed, and send replies. The OHC AI should automatically draft suggested replies for these incoming WhatsApp messages. Ensure the UI handles basic text and image attachments gracefully, both displaying incoming media and allowing the owner to attach photos.

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
