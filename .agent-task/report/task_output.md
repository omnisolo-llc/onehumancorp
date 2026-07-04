issue_title: "Integrate Twilio WhatsApp Business API for Customer Intake and Messaging"
issue_description: |
  ## Problem Statement
  Small-business owners like Maya (Home Baker) and Fatima (Food Cart Operator) receive a significant portion of their customer inquiries, orders, and service requests via WhatsApp. Currently, these messages exist outside of OHC, forcing the owner to constantly context-switch between their personal device and the OHC platform. This leads to missed leads, delayed responses, and fragmented customer context. Non-technical owners need WhatsApp to function as a seamless, integrated channel within OHC, where the AI assistant can auto-draft replies, track order intent, and coordinate tasks without the owner having to open the native WhatsApp app constantly.

  ## Research Report
  ### Market Need & Competitor Analysis
  - **Tencent Workbuddy & WeCom:** Deeply integrate with WeChat to capture conversational commerce. OHC needs a comparable offering for global markets where WhatsApp is the dominant messaging app.
  - **Shopify & Wix:** Both have extensive app marketplaces where WhatsApp integration plugins are consistently top-rated, indicating high demand for conversational commerce capabilities.
  - **Owner Pain Points:** Discussions on r/smallbusiness frequently highlight the difficulty of managing business inquiries via WhatsApp, especially when trying to delegate tasks or maintain a centralized CRM.

  ### Tool Evaluation: Twilio WhatsApp Business API
  - **Usability for Owners:** The complexity of setting up a WhatsApp Business account and Twilio API keys must be completely abstracted from the user. OHC should provide a one-click connection or a guided, simplified onboarding flow.
  - **Capabilities:** Supports rich media, automated templates (for 24h window compliance), and webhooks for real-time inbound message processing.
  - **Pricing:** Twilio offers pay-as-you-go pricing (conversation-based), which is viable for SaaS multi-tenant environments. A free tier or sandbox is available for initial development and testing.
  - **Cloud vs. Standalone:** Primarily a cloud service, but fits well within OHC's multi-tenant architecture using webhooks.

  ## Design Doc
  ### Integration Flow
  1. **Onboarding:** The owner links their WhatsApp Business account via a simplified settings page in OHC (abstracting Twilio credentials where possible, or providing clear, step-by-step instructions for entering Twilio API keys).
  2. **Inbound Webhook:** OHC exposes a webhook endpoint to receive inbound messages from Twilio.
  3. **Work Triage:** The AI Assistant (Work Triage capability) processes inbound messages, identifying intent (e.g., "new order inquiry", "support request").
  4. **Customer Assistant:** The AI drafts a context-aware reply and queues it for owner approval, or sends automated responses based on configured rules (e.g., out-of-office, automated quotes).
  5. **Owner Feed:** The message and suggested actions appear in the owner's unified OHC feed.

  ## Implementation Prompt
  **User-Facing Outcome:**
  As an owner (e.g., Maya), I want to connect my business WhatsApp number to OHC so that customer messages appear directly in my OHC Work Feed. I want the OHC Assistant to automatically draft replies to common inquiries and help me turn WhatsApp conversations into booked orders or tasks.

  **Acceptance Criteria:**
  - Create a settings UI for owners to configure their Twilio WhatsApp credentials or initiate an OAuth-like connection.
  - Ensure inbound WhatsApp messages create a new item in the owner's Work Feed.
  - The AI Customer Assistant must generate draft replies for incoming WhatsApp messages based on the customer's context.
  - Owners must be able to approve, edit, or reject the AI-drafted reply directly from the OHC interface, which then sends the message back via WhatsApp.
  - Validate the integration using the Twilio Sandbox for WhatsApp before rolling out to production numbers.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
