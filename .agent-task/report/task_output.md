issue_title: "Integration Research: Twilio WhatsApp Business API for Unified Triage"
issue_description: |
  ### Title
  Integrate Twilio WhatsApp Business API for Unified Customer Messaging

  ### Problem Statement
  Small business owners like Carlos (field service) and Maya (home baker) run their businesses on WhatsApp. Customers use it to request quotes, send photos of issues or cake designs, and ask for updates. Currently, managing conversations across personal/business WhatsApp and a separate work management tool creates friction, missed leads, and siloed context. Owners need WhatsApp conversations to land directly in their unified Work Triage feed so the AI assistant can draft replies, extract bookings, and keep the owner organized without context switching.

  ### Research Report
  - **Ecosystem & Community Demand**: WhatsApp is the dominant communication channel for small businesses globally, especially in LATAM, India, and parts of Europe. Small business subreddits and operator forums frequently cite the lack of good WhatsApp integration as a dealbreaker for CRM and work management tools.
  - **Tool Evaluated**: Twilio API for WhatsApp.
  - **Capabilities & Limits**: Twilio abstracts Meta's complex Cloud API setup into a clean interface. It handles rich media (images/videos/documents) flawlessly, which is critical for Carlos receiving photos of broken pipes, or Maya receiving cake reference images.
  - **Pricing & SaaS Viability**: Twilio uses pay-as-you-go pricing (a small markup on Meta's conversation-based pricing). It supports multi-tenant architectures well by allowing subaccounts or distinct sender configurations per tenant. It includes a free tier for testing.
  - **Ease of Use for Owners**: Owners do not need to understand APIs. OHC will guide them through an embedded Meta signup flow or simple sender registration. Once connected, the owner just uses OHC's chat interface—the technology is completely invisible.

  ### Design Doc
  - **Webhooks & Ingestion**: Securely receive incoming WhatsApp messages from Twilio. Route messages based on the recipient's connected phone number to ensure they reach the correct tenant's workspace.
  - **Persistence**: Store messages and handle media payloads (compressing images) so they can be displayed inline in the owner's unified feed.
  - **AI Triage**: Incoming messages trigger the Customer & Relationship Assistant agent to analyze the intent, draft a reply, and bubble it up to the Work Triage feed.
  - **Outbound Execution**: When the owner clicks "Send" or approves a draft in the Flutter shell, the system dispatches the reply back through Twilio using the tenant's credentials.

  ### Implementation Prompt
  - **User-Facing Outcome**: Under Settings > Channels, the owner can connect a WhatsApp Business number. Once connected, inbound WhatsApp messages from customers appear instantly in the unified Work Triage feed. The owner can view text and image messages, and the OHC assistant proactively drafts replies. The owner can edit and send replies directly from the OHC app (mobile or desktop), and the customer receives them natively in WhatsApp.
  - **Acceptance Criteria**:
    1. A webhook endpoint securely receives and persists incoming WhatsApp messages from Twilio to the correct tenant.
    2. Incoming images are securely processed and displayed inline in the OHC Work Triage UI.
    3. The owner can reply from the OHC Flutter application, and the message is successfully delivered to the customer's WhatsApp via Twilio.
    4. The AI assistant automatically drafts a contextual reply for inbound WhatsApp messages.
    5. The integration must function perfectly on the 375px mobile viewport.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
