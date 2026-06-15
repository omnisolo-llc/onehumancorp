issue_title: "Integrate Twilio WhatsApp Business API for Unified Customer Messaging"
issue_description: |
  ## Title
  Integrate Twilio WhatsApp Business API for Unified Customer Messaging

  ## Problem Statement
  Owners and operators like Maya (Home Baker) and Carlos (Field Service Owner) rely heavily on WhatsApp to communicate with their customers, receive inquiries, and negotiate services. Currently, these interactions are siloed on their personal or separate business phones, making it difficult to maintain a unified workspace. They have to switch contexts constantly between OHC and WhatsApp to copy-paste order details, send payment links, or remember customer preferences. This manual synchronization causes dropped leads, delayed responses, and lost revenue. They need their WhatsApp conversations integrated directly into OHC so the assistant can draft replies, automatically track inquiries as Work Intake, and let them manage all interactions in one place without technical configuration.

  ## Research Report
  **Market Context & Competitor Analysis:**
  WhatsApp is the primary communication channel for small businesses in LATAM, EMEA, and APAC. Competitors like HubSpot, Zoho, and specialized CRMs heavily feature native WhatsApp integrations. WeCom and DingTalk provide seamless chat integrations for their respective markets. A direct WhatsApp integration is a table-stakes feature for global operations.

  **Tool Evaluated: Twilio WhatsApp Business API**
  Twilio is a market leader for programmable messaging, offering a robust and scalable WhatsApp Business API.

  *   **Ease of Use for Non-Technical Users:** The Twilio API handles the complex underlying WhatsApp Graph API. For the OHC user, the integration will simply ask them to authenticate or connect their WhatsApp Business number through an embedded signup flow, making it completely invisible to them as an infrastructure tool.
  *   **Pricing:** Twilio operates on a pay-as-you-go model with conversation-based pricing. The first 1,000 service conversations per month are typically free or very low cost, making it highly viable for OHC's small business personas. It supports both Cloud and Standalone environments securely.
  *   **Reputation & Reliability:** Twilio is enterprise-grade, highly reliable, and offers strong webhook delivery guarantees, which is critical for real-time chat scenarios.

  ## Design Doc
  **Integration Flow:**
  1.  **Connection:** In the OHC "Integrations" or "Channels" settings, the user selects "Connect WhatsApp". They follow a guided flow to link their WhatsApp Business account (powered by Twilio under the hood).
  2.  **Inbound Work Intake:** When a customer sends a message to the connected WhatsApp number, Twilio fires a webhook to OHC. OHC's Work Triage processes the message and adds it to the unified inbox. If it's a new inquiry, the Customer Assistant creates a new lead profile automatically.
  3.  **Assistant Drafting & Outbound:** The OHC Assistant can automatically draft suggested replies for the owner based on context (e.g., offering available dates for a booking or a payment link). The owner approves or edits the draft in the OHC UI, and OHC sends the message back via the Twilio API.
  4.  **Actionable Insights:** OHC automatically extracts tasks, order details, and requested dates from WhatsApp conversations and turns them into actionable items in the owner's feed.

  ## Implementation Prompt
  **User-Facing Outcome:**
  Owners can connect their WhatsApp Business account to OHC with a few clicks. Once connected, incoming WhatsApp messages appear in their OHC unified inbox. Owners can read messages, see AI-suggested replies based on their business context, and reply directly from OHC. The assistant automatically identifies inquiries, requests for quotes, and bookings, linking them to customer profiles.

  **Acceptance Criteria:**
  - An owner can successfully link their WhatsApp Business number to their OHC workspace.
  - Incoming WhatsApp messages create or update conversations in the OHC unified inbox in real-time.
  - The AI assistant successfully drafts relevant replies to incoming WhatsApp messages for the owner to review and send.
  - Sent messages from OHC are reliably delivered to the customer's WhatsApp.
  - Disconnecting the WhatsApp integration stops message sync immediately and gracefully handles errors.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
