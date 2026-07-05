issue_title: "Integrate WhatsApp Cloud API for Unified Customer Messaging"
issue_description: |
  ### Title
  Integrate WhatsApp Cloud API for Unified Customer Messaging

  ### Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) run their businesses largely through chat. Currently, they have to constantly check their phones, switch between the OHC app and WhatsApp, and manually copy order details or customer preferences. This manual triage causes missed messages, delayed responses, and lost revenue. They need their WhatsApp messages to flow directly into their OHC Work Triage feed so the assistant can automatically draft replies, track order details, and save customer context without the owner ever having to open the WhatsApp app separately.

  ### Research Report
  - **Tool Evaluated**: WhatsApp Cloud API (hosted by Meta).
  - **Market Need**: Messaging is the primary work intake channel for small businesses globally (particularly in LATAM, EMEA, and APAC). Competitors like WeCom and WhatsApp Business app handle basic chat, but lack deep business workflow integration (quoting, task creation).
  - **Pricing & Viability**: The API is free to access and offers 1,000 free service (inbound-initiated) conversations per month, which easily covers the volume for small operators like Fatima and Maya. After the free tier, utility and service conversations cost fractions of a cent per message.
  - **Capabilities**: It provides reliable webhooks for incoming messages, media support (audio notes, images for cake references or repair issues), and structured interactive messages (buttons for "Approve Quote" or "Schedule Pickup").
  - **Usability for Non-Technical Users**: While the underlying API is complex (requires Meta Business Manager setup), OHC can abstract this via an embedded signup flow (OAuth), meaning the owner just clicks "Connect WhatsApp" and logs in.

  ### Design Doc
  - **Integration Trigger**: Users connect their WhatsApp Business number via an embedded OHC settings page.
  - **Inbound Flow**: When a customer sends a WhatsApp message, Meta sends a webhook to OHC. OHC routes this to the specific tenant's Work Triage feed. The Customer & Relationship Assistant analyzes the message, matches the customer, and drafts a reply or identifies it as an action item (e.g., "Wants to order a cake").
  - **Outbound Flow**: The owner reviews the AI-drafted reply in OHC and clicks "Send." OHC calls the WhatsApp Cloud API to deliver the message.
  - **User Experience**: The owner interacts purely within the OHC unified feed. They see WhatsApp messages alongside tasks and alerts. They don't need to manage Meta API tokens; the OHC integration handles the authorization state transparently.

  ### Implementation Prompt
  **User-Facing Outcome**: Provide a "Connect WhatsApp" button in the OHC settings. Once connected, all incoming WhatsApp messages for that number must appear in the OHC Work Triage feed. The AI assistant should automatically attach these messages to the correct customer profile and propose draft responses. The owner can edit and send replies directly from OHC, and the messages will appear in the customer's WhatsApp app.

  **Acceptance Criteria**:
  1. A tenant can successfully authorize their WhatsApp Business account via the OHC settings UI.
  2. Incoming WhatsApp messages create a new item in the Work Triage feed.
  3. The OHC assistant automatically drafts a contextual reply for the owner to review.
  4. The owner can send a reply from OHC, which is successfully delivered to the customer on WhatsApp.
  5. The UI gracefully handles message sending errors (e.g., if the user is offline or the 24-hour service window has expired).

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
