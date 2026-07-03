issue_title: "Integrate WhatsApp Business API for Omnichannel Work Triage"
issue_description: |
  **Title**: Integrate WhatsApp Business API for Omnichannel Work Triage

  **Problem Statement**:
  Many small business operators (like Maya the Home Baker or Carlos the Field Service Owner) receive a significant portion of their customer inquiries, booking requests, and support questions via WhatsApp. Currently, these messages exist outside of OHC, forcing the owner to constantly context-switch between their phone's WhatsApp app and the OHC assistant. This fragmentation leads to missed leads, delayed responses, and a lack of centralized customer memory. They need WhatsApp messages to flow directly into their OHC Work Triage feed alongside emails and web inquiries, where the AI assistant can help draft replies, recognize returning customers, and generate tasks or quotes automatically.

  **Research Report**:
  - **Ecosystem & Market Need**: In many regions (LATAM, EMEA, India) and among certain demographics in the US, WhatsApp is the primary communication channel between businesses and customers. Competitors like Shopify (via Inbox/apps), HubSpot, and Meta Business Suite heavily emphasize WhatsApp integration.
  - **Tool Evaluated**: WhatsApp Business API (specifically via Meta Cloud API or Twilio as a broker).
  - **Ease of Use for Operators**: The actual setup for an owner involves linking their phone number or Meta Business account. Once linked, the owner never has to see the "API" part. They simply use OHC to reply.
  - **Pricing & Reputation**: Meta Cloud API charges per conversation (marketing vs. utility vs. service). The first 1,000 service conversations per month are often free. Twilio adds a small markup per message but simplifies the API and phone number provisioning. For small businesses, the ROI on not missing a booking far outweighs the fraction-of-a-cent message cost.
  - **Cloud vs. Standalone**: Works seamlessly in Cloud (multi-tenant) via webhooks. For Standalone, requires ngrok, Cloudflare Tunnels, or a stable webhook endpoint.

  **Design Doc**:
  - **Integration Point**: The OHC Work Intake and Customer Relationships modules.
  - **Setup Flow**: A new "Channels" section in the OHC UI where the owner clicks "Connect WhatsApp" and goes through the Meta/Twilio OAuth flow to link their business number.
  - **Inbound Flow**: When a customer sends a WhatsApp message, a webhook hits OHC. The AI Job Queue processes it, identifies or creates a customer record, and pushes a new item to the Work Triage feed.
  - **Outbound Flow**: When the owner (or the AI Assistant on their behalf) replies in OHC, OHC sends a request via the WhatsApp Business API back to the customer's phone.
  - **Assistant Capability**: The Customer Assistant agent uses the message context to draft replies or extract structured data (e.g., "I need a cake for Saturday" -> Drafts a quote task).

  **Implementation Prompt**:
  - Create the necessary tenant-scoped configuration tables to store WhatsApp/Twilio credentials and phone numbers securely.
  - Implement a webhook endpoint to receive inbound WhatsApp messages, parse them, and insert them into the owner's Work Triage feed.
  - Add UI in the assistant feed allowing the owner to view the WhatsApp conversation and send replies back through the API.
  - Ensure the AI assistant can read the WhatsApp message text to automatically draft a reply or suggest a follow-up action.
  - **Acceptance Criteria**: An owner can connect a WhatsApp number, receive a message from a customer, see it in their OHC feed, and reply from OHC, with the customer receiving the reply on their WhatsApp app. The AI assistant should automatically suggest a draft reply based on the customer's message.

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
