issue_title: "🔍 Scout: Tool Integration Research - WhatsApp Cloud API"
issue_description: |
  ## Title: 🔍 Scout: Tool Integration Research - WhatsApp Cloud API

  ## Problem Statement
  Small business owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart Operator) receive a huge portion of their customer inquiries, orders, and service requests through WhatsApp. Currently, owners must manually juggle their personal or business WhatsApp app, constantly switching between the app and OHC to capture demand, draft replies, check availability, and send payment links. This manual context-switching causes missed leads, delayed responses, and fragmented customer records. They need their OHC assistant to natively ingest WhatsApp messages and draft replies so they can manage all communication in one place.

  ## Research Report
  - **Tool Evaluated**: WhatsApp Cloud API (by Meta)
  - **Market Need**: WhatsApp is the dominant messaging platform in many global regions (LATAM, EU, India) and increasingly popular for local services in the US. Competitors like HubSpot, Zoho, and specialized local CRMs all feature WhatsApp Business integration as a core, often premium, feature.
  - **Usability for Non-Technical Owners**: Owners only need to authenticate their WhatsApp Business Account via Meta's embedded signup flow (OAuth-like). Once connected, OHC handles everything. The owner just sees messages pop up in their OHC Work Triage feed.
  - **Pricing & Viability**:
    - **SaaS (Cloud)**: Meta charges per conversation (marketing, utility, service). The first 1,000 service conversations per month are typically free, making it highly viable for small operators.
    - **Standalone**: Meta Cloud API is hosted by Meta, but the webhook and API calls can easily be routed to a local OHC instance if the owner exposes a secure webhook URL (e.g., via ngrok/Cloudflare Tunnels).
  - **Capabilities & Limits**:
    - **Pros**: Rich media support, structured message templates (useful for quotes/booking confirmations), and high deliverability.
    - **Cons**: Strict 24-hour customer service window for free-form replies. Requires approved templates for outbound notifications after 24 hours.

  ## Design Doc
  - **Trigger / Flow**:
    1. **Onboarding**: Owner links their Meta Business account in OHC Settings. OHC stores the access token and registers a webhook.
    2. **Inbound**: Customer sends a WhatsApp message. Meta sends a webhook to OHC's API layer. OHC routes this to the specific tenant and drops it into the `Work Triage` queue.
    3. **AI Action**: The `Customer & Relationship Assistant` automatically drafts a reply based on the message intent (e.g., asking about cake prices) and presents it to the owner in the feed.
    4. **Outbound**: Owner taps "Approve & Send" on the draft. OHC calls the WhatsApp Cloud API to send the message.
  - **User Interface**: No technical jargon. Just a "Connect WhatsApp" button in settings. Messages appear seamlessly in the existing unified inbox / daily feed alongside Instagram DMs and emails.

  ## Implementation Prompt
  - Create a "Connect WhatsApp" button in the Settings > Integrations UI that initiates the Meta Embedded Signup flow.
  - Implement a multi-tenant webhook handler in the Go backend to securely receive and parse WhatsApp Cloud API inbound messages.
  - Route parsed messages into the unified Work Triage feed, displaying the WhatsApp icon next to the customer's name.
  - Wire up the Customer Assistant LLM capability to generate suggested replies specifically formatted for chat (short, friendly).
  - Add the ability for the owner to approve and send the draft reply back to the customer via the WhatsApp API.
  - **Acceptance Criteria**: Owner can link account, receive a test message from a customer in the OHC feed, see an AI-drafted reply, and click "Send" to successfully deliver the message back to the customer's WhatsApp.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []