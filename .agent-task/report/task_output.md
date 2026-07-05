issue_title: "Integrate Twilio for WhatsApp Business API to Unify Customer Work Intake"
issue_description: |
  **Title**: Integrate Twilio for WhatsApp Business API to Unify Customer Work Intake

  **Problem Statement**:
  Our owner/operator personas, such as Maya the Home Baker and Carlos the Field Service Owner, receive a massive amount of customer inquiries, pre-orders, and service requests via WhatsApp. Currently, these messages are siloed on their personal devices or separate business accounts, forcing them to manually triage, respond, and copy data into their management tools. This leads to missed leads, delayed responses, and fragmented work context. They need OHC to unify WhatsApp conversations directly into their work feed, allowing the AI assistant to draft replies, create bookings, and track customer history seamlessly.

  **Research Report**:
  - **Tool Evaluated**: Twilio for WhatsApp Business API
  - **Market Need & Competitive Context**: Competitors like WeCom and DingTalk heavily integrate with local messaging ecosystems (e.g., WeChat) to capture demand at the source. In LATAM, parts of Europe, and many US service sectors, WhatsApp is the primary business communication channel. Owners explicitly seek "WhatsApp integration" as a top requirement in CRMs and operational tools (e.g., HubSpot, Shopify App Store reviews).
  - **Usability for Non-Technical Owners**: Via Twilio, the complex API structure is abstracted away. The owner would only need to authenticate or link their business number once. From then on, WhatsApp messages simply appear in the OHC feed like any other work item.
  - **Capabilities & Limits**: Twilio provides robust APIs, reliable webhooks for incoming messages, and excellent uptime. It supports rich media (images, PDFs) essential for tasks like Maya receiving cake inspiration photos. Limits include a 24-hour customer service window constraint imposed by Meta, which OHC will need to handle gracefully (e.g., prompting the owner to use a pre-approved template outside the window).
  - **SaaS Viability**: Twilio operates primarily in the cloud. It offers a scalable pay-as-you-go pricing model (approx. $0.005 per message + Meta's conversation pricing), which is highly viable for a multi-tenant SaaS.

  **Design Doc**:
  - **Integration Point**: OHC backend will expose a secure webhook endpoint to receive incoming WhatsApp messages from Twilio.
  - **Work Triage & Assistant Action**:
    - Incoming messages trigger the Work Triage agent.
    - If the phone number matches an existing customer, the conversation is appended to their history. If new, a lead is created.
    - The Customer & Relationship Assistant analyzes the message (e.g., "Can I order a cake for Saturday?") and drafts a contextual reply for the owner's review.
    - If rich media (images) is received, it is stored in OHC's file storage (GCS/MinIO) and linked to the conversation.
  - **User Experience**: The owner sees a new item in their OHC command center feed: "New WhatsApp Inquiry from [Customer Name]". The item shows the message and the AI's drafted response with a single "Send & Create Task" button. The 24-hour response window is visually indicated if approaching.

  **Implementation Prompt**:
  - Implement a webhook receiver in the backend to process incoming messages from Twilio's WhatsApp API.
  - Connect the receiver to the Work Triage AI queue so that incoming messages generate actionable feed items in the OHC UI.
  - Ensure the Customer & Relationship Assistant can generate drafted replies specifically formatted for WhatsApp (concise, conversational).
  - Add UI support for displaying WhatsApp messages in the feed, including handling image attachments, and provide a clear indicator for Meta's 24-hour response window.
  - Provide a simple "Link WhatsApp Account" flow in the settings for the owner.

  **Priority**: P1 (High)
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
