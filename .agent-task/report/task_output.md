issue_title: "Integrate WhatsApp Cloud API for Unified Customer Messaging"
issue_description: |
  ## Title
  Integrate WhatsApp Cloud API for Unified Customer Messaging

  ## Problem Statement
  For owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart), a vast majority of customer inquiries, orders, and service requests occur over WhatsApp. Currently, these owners have to switch back and forth between their personal/business WhatsApp app and the OHC assistant. This fragmentation leads to missed leads, delayed responses, and lost context. OHC needs to unify these messages directly into the Work Triage feed, allowing the Customer Assistant to draft replies and the Operations Assistant to turn chat inquiries directly into bookings and tasks, all without the owner leaving OHC.

  ## Research Report
  ### Market Context & Competitors
  Competitors like WeCom and DingTalk heavily integrate with regional messaging apps (WeChat, DingTalk chats), whereas Shopify Inbox and HubSpot unify Meta messaging channels. For global small businesses, particularly in LATAM, EU, and Asia, WhatsApp is the dominant communication channel.

  ### Tool Evaluated: WhatsApp Cloud API (Meta)
  - **Usability for Owners:** Non-technical owners do not want to configure webhooks or manage Meta developer accounts. OHC must offer a frictionless "Sign in with Facebook / Connect WhatsApp Business" OAuth flow via Meta's Embedded Signup. Once connected, it just works—messages appear in OHC, and replies from OHC go to the customer's WhatsApp.
  - **Capabilities:** Supports text, images, location (useful for Carlos' field service routes), interactive buttons (useful for Maya's cake options), and template messages (for out-of-session notifications like appointment reminders).
  - **SaaS Viability:** The API is free for the first 1,000 service conversations per month, which easily covers our entry-tier owners. OHC can build a multi-tenant cloud setup. For standalone/local deployments, users can supply their own Meta App credentials.
  - **Reliability:** High SLA from Meta, reliable webhooks.

  ## Design Doc
  ### Triggers & Workflows
  - **Onboarding:** Owner connects WhatsApp via an "Integrations" UI card.
  - **Inbound Message:** Meta sends a webhook payload to OHC. OHC validates the webhook signature, identifies the tenant from the connected WhatsApp Business Account ID, and enqueues an AI Job. The Work Triage agent parses the message, updates the customer's conversation history, and surfaces it in the owner's feed.
  - **Outbound Message (AI Draft/Owner Reply):** The Customer Assistant suggests a reply. If the owner approves, OHC posts the message back to the WhatsApp Cloud API.
  - **Rich Media & Context:** Customers sending images (e.g., cake reference photos) will be downloaded, processed by the OHC Knowledge & Customer agents, and attached to the lead.

  ## Implementation Prompt
  **User-Facing Outcome:**
  The owner should see a new integration card for "WhatsApp". Upon connecting, incoming WhatsApp messages will appear directly in the OHC assistant's unified feed. The owner can read messages, view customer details, and approve AI-drafted replies, which will be sent back to the customer's WhatsApp seamlessly.

  **Acceptance Criteria:**
  1. Create a secure "Connect WhatsApp" configuration UI in the integrations section.
  2. Implement an authenticated Meta webhook handler that reliably associates incoming messages with the correct OHC tenant.
  3. Ensure incoming text and images are added to the unified Work Triage feed.
  4. Enable the owner to reply (and the AI to draft replies) directly from the OHC interface, transmitting them successfully via the WhatsApp Cloud API.
  5. The integration must gracefully handle offline/error states (e.g., API limits or disconnected accounts) with clear, actionable UI alerts.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
