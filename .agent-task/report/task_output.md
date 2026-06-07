issue_title: "🔍 Scout: Tool Integration Research - WhatsApp Business API via Twilio"
issue_description: |
  ### Title: Integrate WhatsApp Business API via Twilio for Seamless Customer Communication

  ### Problem Statement
  Owners and operators like Maya (Home Baker) and Fatima (Food Cart Operator) rely heavily on direct messaging to capture demand, coordinate orders, and handle customer service. While OHC handles web inquiries well, many customers globally, particularly in LATAM, EMEA, and APAC regions, prefer communicating exclusively via WhatsApp. Currently, owners have to switch context out of OHC to their personal or business WhatsApp app, leading to missed inquiries, fragmented customer history, and inability for OHC's AI assistant to draft replies or triage work automatically. The lack of WhatsApp integration forces non-technical owners to act as manual routers between their phone and OHC.

  ### Research Report
  - **Market Context**: WhatsApp is the dominant messaging platform globally with over 2 billion active users. Competitors like WeCom and DingTalk have built-in deep integrations with dominant local messaging platforms (like WeChat). Independent operators often use WhatsApp Business, but managing high volumes manually is error-prone.
  - **Tool Evaluated**: Twilio API for WhatsApp.
  - **Why Twilio**: Twilio is the industry standard for programmatic SMS and WhatsApp. It abstracts away the complexity of directly integrating with Meta's Graph API, provides robust webhooks, handles template message approvals, and offers both Cloud and Standalone (API-driven) deployment patterns perfectly suited for our backend (Go/Bazel).
  - **Ease of Use for Owners**: Owners do not need to understand APIs. They will simply go through an OAuth-like onboarding flow ("Connect WhatsApp") powered by Twilio's Embedded Signup, instantly bridging their WhatsApp Business number with their OHC Assistant.
  - **Pricing & Viability**: Twilio offers pay-as-you-go pricing with a free tier for testing. WhatsApp conversation-based pricing is affordable and standard for businesses. This is highly viable for a multi-tenant SaaS.
  - **Capabilities & Limits**: Rich media (images, PDFs) are supported, which is essential for sharing quotes and invoices. The 24-hour customer service window applies, meaning OHC must track message timing and fallback to template messages or SMS if replying late.

  ### Design Doc
  - **Trigger**: When a customer sends a message to the owner's connected WhatsApp number, Twilio fires a webhook to OHC's backend.
  - **Action**: The OHC Work Triage capability processes the incoming payload, matches the phone number to an existing Customer record (or creates a new lead), and surfaces it in the owner's feed. The Customer Assistant drafts a suggested reply.
  - **User Experience**: The owner sees the WhatsApp message natively inside the OHC interface, alongside the context (past orders, notes). They click "Send" on the AI-drafted reply, which routes back through Twilio to the customer's WhatsApp.
  - **Onboarding**: A new "Integrations" section in the owner's settings with a "Connect WhatsApp" button that uses Twilio's Embedded Signup flow.

  ### Implementation Prompt
  - **Outcome**: A non-technical owner can connect their WhatsApp Business number to OHC. Incoming WhatsApp messages appear in the OHC Work Feed. The owner can read the message, view an AI-drafted reply, and hit "Send" to reply directly to the customer's WhatsApp without leaving OHC.
  - **Acceptance Criteria**:
    - The UI provides a "Connect WhatsApp" setup flow.
    - Incoming WhatsApp messages via Twilio webhook are ingested and visible in the owner's Work Feed.
    - The AI Assistant correctly parses the message context and provides a drafted reply.
    - Outgoing replies sent from the OHC UI are successfully delivered to the customer via Twilio.
    - The UI must handle the 24-hour response window intelligently (e.g., warning the owner if the window has closed).

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
