issue_title: "🔍 Scout: Tool Integration Research - WhatsApp Business API"
issue_description: |
  # Research Report: WhatsApp Business API Integration

  ## Problem Statement
  Small business owners, especially those outside of North America or dealing with high-touch sales (like custom cakes or personal services), use WhatsApp as their primary communication tool. Currently, managing incoming inquiries, booking requests, and support via personal WhatsApp accounts leads to missed messages, manual entry of orders into OHC, and an inability to use AI agents to handle routine questions. Business owners need a unified way to handle WhatsApp messages directly from the OHC platform and have AI agents automatically reply to common questions or capture leads.

  ## Research Report
  *   **Target User Benefit:** Maya the Baker and Fatima the Food Cart Operator directly benefit by moving their customer inquiries from a chaotic personal WhatsApp app to a structured inbox within OHC, where the "Customer Success" AI agent can intercept basic queries (e.g., "Do you do vegan cakes?") while the owner sleeps.
  *   **Competitor Analysis:** Shopify and Wix have large app ecosystems for WhatsApp integration (e.g., "WhatsApp Chat + Abandoned Cart" plugins). However, these are often bolt-on solutions that require separate subscriptions. By embedding WhatsApp into OHC natively, we deliver on the promise of an all-in-one platform without extra configurations.
  *   **Tool Deep-Dive (WhatsApp Business API via Twilio or Meta Direct):**
      *   **Ease of Use:** For the end-user (business owner), it must be seamless. They should only need to click "Connect WhatsApp" and go through an OAuth flow.
      *   **Pricing:** Meta charges per conversation. We can integrate via Twilio (easier developer experience, scalable) or directly with Meta (lower cost). Both support multi-tenant SaaS environments.
      *   **Capabilities:** Rich media messages, automated templates (for order updates), and webhooks for real-time AI replies.
  *   **Cloud vs. Standalone Viability:** Highly viable for Cloud via secure webhooks.

  ## Design Doc
  *   **Trigger:** A customer messages the business owner's connected WhatsApp number.
  *   **Action:** The WhatsApp API sends a webhook to the OHC backend. The message is routed to the business's unified inbox. The "Customer Success" AI agent is triggered to evaluate if it can auto-reply based on the business's knowledge base.
  *   **User Interface:** The business owner sees the conversation in their OHC mobile app inbox. They can seamlessly take over the conversation from the AI if needed. A "Connect WhatsApp" settings page will handle the OAuth setup.

  ## Implementation Prompt
  Integrate the WhatsApp Business API to allow small businesses on OHC to receive and reply to WhatsApp messages directly within their unified OHC inbox. The implementation must support the routing of incoming messages to the AI "Customer Success" agent for initial handling and allow the business owner to seamlessly take over the chat. Create the necessary UI settings for owners to authenticate their WhatsApp Business accounts.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
