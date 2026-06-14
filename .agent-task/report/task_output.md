issue_title: "Implement WhatsApp Business Cloud API Integration for Work Triage"
issue_description: |
  **Title**: Implement WhatsApp Business Cloud API Integration for Work Triage and Customer Replies

  **Problem Statement**:
  Many of our owner/operator personas (like Maya, the home baker, and Carlos, the field service owner) conduct a significant portion of their business conversations, intake, and customer support via WhatsApp. Currently, these conversations exist entirely outside of OHC, leading to fragmented context, missed orders, and unlogged customer histories. Owners have to constantly context-switch between OHC and their WhatsApp mobile app. This breaks the "One assistant" promise, preventing OHC from drafting replies, coordinating bookings, or tracking lead conversions from one of the most widely used messaging channels globally.

  **Research Report**:
  *   **Market Need & Discovery**: Competitors like WeCom, Zoho Bigin, and HubSpot offer direct WhatsApp integrations, which are consistently among their most-installed marketplace apps. Research in r/smallbusiness and SMB forums highlights that WhatsApp is the primary communication medium for local services and commerce outside the US (and increasingly inside the US).
  *   **Tool Candidate**: Meta's WhatsApp Business Cloud API (direct).
      *   **Direct Meta API**: Provides lower latency and better pricing than middleware like Twilio. Requires setting up a Meta app, webhooks, and business verification. Ideal for a SaaS platform (OHC as a Tech Provider).
      *   **Usability for Non-Technical Users**: From the owner's perspective, the setup involves a simple OAuth-like "Log in with Facebook" flow to link their WhatsApp Business number. Once linked, messages flow directly into OHC's "Work Triage" feed.
      *   **Pricing**: Cloud API hosted by Meta is free for the API usage itself. Business-initiated conversations have regional pricing, but user-initiated service conversations (the primary use case for Maya/Fatima) are free up to 1,000/month. This fits perfectly with our owner profiles.
      *   **Reliability**: Meta's Cloud API provides webhooks for incoming messages, delivery receipts, and read receipts. It supports multi-tenant architecture natively.

  **Design Doc**:
  *   **Integration Points**:
      *   **WhatsApp Setup Flow**: A "Connect WhatsApp" button in the OHC Settings area triggers an embedded Meta setup flow (Embedded Signup for Tech Providers).
      *   **Webhook Ingestion**: OHC exposes a single endpoint to receive Meta Webhooks, which are parsed, matched to an OHC `tenant_id` via the destination phone number/WABA ID, and dispatched to the AI Job Queue.
      *   **Work Triage Rendering**: Incoming messages create or update an OHC conversation thread. If action is needed (e.g., an order request), the Work Triage UI surfaces it as a prioritized task.
      *   **Assistant Action**: The "Customer & Relationship Assistant" reads the message content, formulates a draft response based on the owner's knowledge base (e.g., pricing, availability), and presents it in the UI for the owner to "Send" or "Edit".
      *   **Outgoing Delivery**: OHC sends the approved message via the Meta Cloud API `/messages` endpoint.

  **Implementation Prompt**:
  *   Implement the embedded signup flow for Meta WhatsApp Business accounts so owners can connect their number simply by authenticating.
  *   Create a robust webhook receiver that handles incoming WhatsApp messages (text, image, audio), verifies the Meta signature, and routes them to the correct tenant's Work Triage feed.
  *   Extend the Work Triage UI to display WhatsApp messages alongside other channels, allowing the AI Assistant to draft inline replies.
  *   Build the outgoing API client to send owner-approved text and media replies back to the customer's WhatsApp.
  *   **Acceptance Criteria**: A non-technical owner can link their WhatsApp number. When a customer sends a WhatsApp message, it appears in OHC. The AI assistant successfully drafts a context-aware reply, and clicking "Send" delivers the message back to the customer. All database updates and network calls must be covered by E2E tests using simulated webhook payloads.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
