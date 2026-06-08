issue_title: "🔍 Scout: Tool Integration Research - WhatsApp Cloud API (Meta)"
issue_description: |
  # WhatsApp Cloud API Integration Report

  ## Problem Statement
  Owners like Maya (Home Baker), Fatima (Food Cart), and Carlos (Field Service) conduct a massive share of their business communication over WhatsApp. Currently, they suffer from fragmented workflows—having to switch between the WhatsApp Business app and their other software tools. They must manually copy-paste customer context, type out repetitive quotes on mobile keyboards, and risk missing important inquiries when they are busy operating. They need their WhatsApp messages to flow directly into OHC’s centralized "Work Triage" feed, where the AI Assistant can immediately draft replies, recognize returning customers, and seamlessly trigger operational workflows (like order prep or scheduling) without the owner ever leaving the OHC interface.

  ## Research Report
  - **Market Context & Need Discovery**: WhatsApp is the dominant communication channel for SMBs across LATAM, EMEA, APAC, and is rapidly growing in the US. Competitors such as Sirena, HubSpot, and Sprout Social offer WhatsApp integration, but these platforms are often overwhelmingly complex and too expensive for micro-operators. Community mining in r/smallbusiness reveals constant complaints about dropping the ball on WhatsApp leads.
  - **Tool Evaluation (Meta WhatsApp Cloud API)**:
    - **Ease of Use for Owners**: Utilizing the Facebook Embedded Signup flow, an owner can link their WhatsApp Business number in a few clicks. No technical knowledge of API keys or webhooks is required from the owner.
    - **Capabilities**: The API supports rich text, images, location sharing, interactive buttons (e.g., "Book Now", "View Catalog"), and product lists. This is critical for OHC’s AI agents to present highly actionable choices directly to the customer in their preferred chat app.
    - **SaaS Viability & Pricing**: Meta provides the first 1,000 service conversations per month for free, which easily covers the inbound volume of our target personas (like Leo's tutoring business or Maya's custom cakes). Beyond that, conversational pricing is highly predictable. The API is cloud-native (webhook-based) and scales perfectly for a multi-tenant PostgreSQL architecture. For standalone/local deployments, webhooks would require a tunneling utility (e.g., Cloudflare Tunnels), which OHC can optionally bundle.

  ## Design Doc
  - **Integration Architecture**:
    - **Authentication**: Integrate the Facebook Embedded Signup UI into OHC's settings. The owner follows an OAuth-like flow to grant OHC access to their WhatsApp Business Account.
    - **Ingestion & Triage**: Configure OHC to receive incoming webhook events from Meta. When a customer messages the owner, OHC receives the payload, associates it with a Customer record (matching the phone number), and pushes it to the Work Triage feed.
    - **AI Assistant Hand-off**: The Work Triage capability routes the conversation to the Customer & Relationship Assistant. The agent reads the conversation history, extracts intent (e.g., "needs a quote for a cake"), and drafts a response. If an action is required, the Operations or Sales Assistant prepares the corresponding link (like a Stripe Payment Link).
    - **Owner Verification**: The OHC mobile UI (375px optimized) shows the incoming message and the AI's drafted reply. The owner simply taps "Approve & Send" or edits the draft. Advanced users can let the AI auto-reply for specific intents.

  ## Implementation Prompt
  - **Frontend (Flutter)**: Implement the Meta Embedded Signup component so the owner can securely link their WhatsApp Business profile. Build the conversational UI within the Work Triage feed to display WhatsApp messages alongside OHC AI drafted replies, ensuring touch targets are 44x44px and no horizontal scroll occurs on mobile.
  - **Backend (Go + PostgreSQL)**: Implement securely authenticated webhook handlers to receive WhatsApp payload events. Map the JSON payloads to OHC's internal Message format and persist them with tenant isolation.
  - **AI Capability Integration**: Connect the parsed WhatsApp context to the `Customer & Relationship Assistant` system prompt to generate culturally aware, concise drafts suitable for mobile chat.
  - **Outbound Dispatch**: Build the egress service to format the owner-approved draft into the required Meta API schema and dispatch the message. Handle asynchronous delivery receipts and update the UI accordingly.
  - **Acceptance Criteria**: The owner (e.g., Maya) connects her WhatsApp account, receives a real WhatsApp message from a potential client, sees it appear in the OHC Triage feed with a drafted response, taps "Approve & Send", and the client receives the reply in WhatsApp—all from an iPhone screen.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
