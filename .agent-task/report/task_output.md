issue_title: "Integrate WhatsApp Cloud API for Unified Inbox and AI Auto-Replies"
issue_description: |
  ## Mission Queue Protocol: WhatsApp Cloud API Integration

  ### Problem Statement
  Personas like Maya (Home Baker), Fatima (Food Cart), and Carlos (Field Service) communicate with their customers primarily through WhatsApp, especially in global and mobile-first markets. Currently, they have to constantly switch between OHC and the WhatsApp app on their phones to answer inquiries, send quotes, accept deposits, and confirm orders. This causes missed leads, delayed responses, and fragmented customer context. They need OHC to capture WhatsApp messages natively, allow the AI assistant to draft replies automatically, and enable them to manage WhatsApp conversations directly within the unified OHC Work Triage feed.

  ### Research Report
  - **Tool Evaluated**: Meta WhatsApp Cloud API.
  - **Ecosystem & Competitor Analysis**: WeChat (WeCom) integration is the foundational piece of Tencent Workbuddy. For the rest of the world, WhatsApp is the equivalent. Every major CRM (HubSpot) and commerce platform (Shopify) has highly-rated apps to bring WhatsApp into the shared inbox.
  - **Usability for Non-Technical Owners**: Owners will not see API keys or webhooks. They will connect their existing WhatsApp Business account via Meta's Embedded Signup flow (a standard OAuth-like popup). Once connected, WhatsApp messages simply become tasks/chats in their OHC inbox.
  - **Pricing & SaaS Viability**: Meta offers the first 1,000 user-initiated service conversations per month for free. This generous free tier easily covers the initial needs of small operators like Maya, Fatima, and Leo. Beyond that, per-conversation pricing (e.g., ~$0.015 for utility/service messages) can be passed through to the owner or bundled into OHC premium tiers.
  - **Cloud vs. Standalone**: In a multi-tenant cloud setup, OHC will expose a unified webhook endpoint for Meta, routing messages to tenants based on the destination phone number. For standalone/local deployments, a reverse proxy (like ngrok or Cloudflare Tunnels) would be required to receive webhooks.

  ### Design Doc
  - **Trigger**: A webhook is received from the WhatsApp Cloud API containing a text, image, or audio message from a customer.
  - **Action**: The OHC Work Triage system ingests the webhook, identifying the tenant via the receiver's phone number. The OHC Customer Assistant agent processes the message context, tags it (e.g., "New Lead", "Support"), and drafts a suggested reply based on the tenant's knowledge base and past interactions.
  - **Owner View**: The owner sees a new item in their OHC Work Triage feed with a distinct WhatsApp icon. They can read the customer's message, review the AI's drafted reply, edit it if necessary, and tap "Send". The owner never leaves the OHC app.
  - **Background Agent**: The Sales & Revenue Assistant can also step in if the message is identified as an order, drafting a Stripe Payment Link or Quote to send back via WhatsApp.

  ### Implementation Prompt
  - **User-Facing Outcome**: An owner can link their WhatsApp Business number to OHC. Incoming WhatsApp messages appear in the OHC Work Triage feed. The AI Assistant drafts responses. The owner can send replies directly from the OHC mobile or web app, which are delivered securely to the customer's WhatsApp.
  - **Acceptance Criteria**:
    1. A settings UI allows the owner to connect their WhatsApp Business account via Meta Embedded Signup.
    2. A secure webhook listener processes incoming WhatsApp messages, parsing text/images, and correctly routing them to the relevant tenant's Work Triage feed.
    3. The Customer Assistant agent automatically reads the message context and drafts a suggested reply, visible in the feed.
    4. The owner can review, edit, and send the reply from OHC, which successfully delivers to the customer via the WhatsApp Cloud API.
    5. The UI displays appropriate status tokens (Pending, Sent, Delivered, Read) and WhatsApp branding to differentiate from SMS or Email.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
