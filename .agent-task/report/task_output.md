issue_title: "Integrate WhatsApp Business API (via Twilio) for Conversational Commerce and Operations"
issue_description: |
  # Research Report: WhatsApp Business API Integration for OHC

  ## Problem Statement
  For many owner/operators (especially in international markets, but increasingly in the US), WhatsApp is the defacto operating system for their business. Customers expect to be able to text a business to place an order, book a service, or ask a question. Currently, owners like Maya (Home Baker) and Fatima (Food Cart Operator) have to constantly switch between their personal/business WhatsApp app and whatever tools they use to manage orders. This leads to missed messages, lost context, and disjointed operations. They need a way for WhatsApp messages to feed directly into OHC's Work Triage, so the AI Assistant can draft replies, capture orders, and track context seamlessly without the owner having to leave the OHC command center.

  ## Research Report
  - **Tool Evaluated:** Twilio API for WhatsApp (as a proxy/provider for the WhatsApp Business API).
  - **Relevance:** Competitors like WeCom and local CRM tools heavily integrate with WeChat or WhatsApp to centralize customer communications.
  - **Why Twilio?** Direct WhatsApp Business API access can be cumbersome for a SaaS to manage on behalf of multi-tenant users. Twilio provides a unified API, robust webhook delivery, and handles the complexity of WhatsApp template approvals and sender onboarding.
  - **SaaS Viability:** Twilio supports multi-tenancy easily via subaccounts or unified messaging services. It scales well for Cloud environments and can be configured per-tenant. Pricing is pay-per-conversation, which aligns with business usage.
  - **Owner/Operator Benefit:** No technical jargon. The owner simply connects their WhatsApp number, and suddenly their OHC Assistant is reading and drafting replies to their WhatsApp messages. It fits the "Open OHC and immediately know what needs attention today" promise perfectly.

  ## Design Doc
  - **Trigger:** An incoming WhatsApp message hits a webhook hosted by OHC.
  - **Action:** The webhook payload is parsed, matched to the `tenant_id` based on the receiving number, and routed to the OHC AI Job Queue.
  - **Integration Point:** The Work Triage system processes the message. If it's an existing customer, context is attached. The Customer & Relationship Assistant drafts a reply or suggests a next action (e.g., "Draft a quote for this cake order").
  - **User Experience:** The owner sees the WhatsApp message appear in their unified OHC feed. They can review the AI-drafted response, edit it, and click "Send". The message goes back out via the Twilio API to the customer's WhatsApp.

  ## Implementation Prompt
  1. Add a new settings section in the OHC UI (under "Channels" or "Integrations") where an owner can connect their WhatsApp number (this will likely involve an OAuth-like flow or inputting Twilio credentials depending on our partner model, but keep the UI simple: "Connect WhatsApp").
  2. Implement an incoming webhook endpoint `POST /api/webhooks/twilio/whatsapp` that receives Twilio's payload.
  3. Map the incoming message to a unified "Message" entity in the database, linked to the correct Tenant and Customer.
  4. Emit an event to trigger the AI Assistant to evaluate the message and generate a draft response.
  5. Display the message and the draft response in the owner's primary Work Triage feed in the Flutter app.
  6. Implement the outbound path: when the owner clicks "Send" in the UI, make a request to the Twilio API to send the WhatsApp message, and mark the draft as sent.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
