issue_title: "Twilio WhatsApp Business API Integration"
issue_description: |
  # Title: Twilio WhatsApp Business API Integration

  ## Problem Statement
  For non-technical owners like **Maya** (Home Baker) and **Carlos** (Field Service Owner), communicating with customers on their preferred channel is paramount. WhatsApp is one of the most widely used messaging apps globally, especially for small businesses handling order inquiries, support, and notifications. Currently, owners have to switch context out of the OHC assistant to reply to customers on their personal or business WhatsApp apps, leading to fragmented work triage, lost context, and delayed responses. They need the OHC AI assistant to natively ingest WhatsApp messages into the unified priority feed, draft replies, and allow one-click sending, all while keeping the customer's context linked to their CRM and operational data.

  ## Research Report
  - **Tool Evaluated**: Twilio WhatsApp Business API
  - **Target Audience Relevance**: Direct fit for direct-to-consumer and service-based operators who heavily rely on mobile chat. Twilio provides robust abstraction over the underlying Meta API.
  - **Usability for Non-Technical Owners**: From the owner's perspective, they don't need to know it's Twilio under the hood. They simply authenticate or link their business phone number once (using Twilio's ISV Tech Provider Program or direct self sign-up flow). After that, the OHC Assistant treats WhatsApp as just another channel in the "Work Intake" feed.
  - **Key Capabilities**:
    - **One-way messaging/notifications**: Sending templates (e.g., appointment reminders, OTPs, delivery updates).
    - **Two-way conversational messaging**: Enabling back-and-forth chats seamlessly inside OHC via webhooks and the Programmable Messaging/Conversations APIs.
    - **Rich Media**: Support for images (e.g., cake photos, repair receipts), locations, and buttons, which are critical for our personas.
  - **SaaS Viability & Pricing**: Twilio's per-conversation pricing model is scalable. The webhook-based architecture is highly reliable and well-suited for our AI Job Queue (PostgreSQL SKIP LOCKED) to process incoming messages asynchronously without dropping payloads.
  - **Architecture Fit**: Perfectly suits our cloud multi-tenant architecture (via Twilio Messaging Services / WhatsApp Senders mapped to `tenant_id`) as well as standalone configurations where users can supply their own API keys.

  ## Design Doc
  - **Setup**: Owners visit a "Channels & Integrations" screen in the OHC UI and click "Connect WhatsApp". They go through a simple guided linking process.
  - **Ingestion**: Twilio webhook endpoints in the OHC backend receive incoming messages. These are mapped to a specific `tenant_id` and pushed into the PostgreSQL AI Job Queue.
  - **Work Triage**: The OHC Assistant picks up the job, retrieves the customer profile, generates a summary/context, drafts a reply, and places a unified card in the owner's daily feed (e.g., "Carlos, new repair inquiry from John on WhatsApp").
  - **Action**: The owner reviews the AI-drafted reply and clicks "Send", or types their own response. The backend calls the Twilio `messages.create` API to dispatch the message back to the customer's WhatsApp.
  - **Media Handling**: Incoming images are downloaded from Twilio and stored in the tenant's GCS/MinIO bucket.

  ## Implementation Prompt
  Implement the Twilio WhatsApp Business API integration so that owners can receive and send WhatsApp messages directly from the OHC Assistant feed.

  **Acceptance Criteria:**
  1. Provide an authentication/linking flow in the UI for an owner to connect their WhatsApp Business number.
  2. Implement an inbound webhook endpoint that securely receives Twilio payloads, verifies the Twilio signature, and enqueues the message into the tenant's AI Job Queue.
  3. Ensure the Work Triage agent processes these queued messages and surfaces them in the owner's feed with customer context.
  4. Provide a backend service and UI action for the owner to send responses back to the customer via Twilio.
  5. Include E2E Playwright tests that simulate an inbound message and verify it appears in the owner feed.
  6. The implementation must follow our strict Mobile-First and Owner Clarity principles—no complex API jargon should be visible to the user.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
