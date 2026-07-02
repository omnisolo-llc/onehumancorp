issue_title: "Architectural Design: Omnichannel Unified Inbox & Agentic Triage"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: Architectural Design: Omnichannel Unified Inbox & Agentic Triage

  **Problem Statement**:
  Small business owners like Maya (baker) and Carlos (handyman) are losing leads because they miss messages scattered across Instagram DMs, WhatsApp, SMS, and Email. They need a single, unified inbox where an AI agent triages incoming messages, drafts context-aware replies, and highlights urgent actions (like a new lead or a complaint) directly on their mobile device. The current system lacks a unified data model to ingest and normalize messages across these disparate channels.

  **Research Report**:
  - **Market Gap**: Legacy platforms like Shopify require 3rd party paid apps for basic omnichannel chat. Wix has a basic unified inbox but lacks proactive AI triage.
  - **User Pain Point**: "I lose leads because I miss Instagram DMs." - 22% frequency in SMB research.
  - **Competitive Advantage**: OHC will provide a native, agent-driven unified inbox that doesn't just aggregate messages but actively drafts replies (Auto-Responder Agent) and suggests next actions, moving from passive CRM to active AI assistant.
  - **Discovery**: Current OHC architecture lacks normalized cross-channel conversational state and a realtime webhook ingestion pipeline to handle high-volume webhook events from Meta (Instagram/WhatsApp) and Twilio (SMS).

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    graph TD;
        MetaWebhooks[Instagram/WhatsApp Webhooks] --> WebhookGateway[API Gateway];
        TwilioWebhooks[Twilio SMS Webhooks] --> WebhookGateway;
        EmailIngest[Email Ingestion] --> WebhookGateway;

        WebhookGateway --> JobQueue[(PostgreSQL Job Queue - SKIP LOCKED)];
        JobQueue --> IngestionWorker[Message Normalization Worker];

        IngestionWorker --> UnifiedConversationDB[(PostgreSQL - tenant isolated)];

        UnifiedConversationDB --> AgentTriage[AI Triage & Draft Agent];
        AgentTriage --> UnifiedConversationDB;

        UnifiedConversationDB --> MobileClient[Flutter Mobile Client 375px];
    ```
  - **Mobile UX Flow (375px)**:
    1. The owner opens the OHC app.
    2. The 'Today' dashboard highlights an "Urgent: 3 unread messages" card (UniFi style layout).
    3. Tapping opens the Unified Inbox: a consolidated list of threads. Each thread shows a small icon (Instagram, WhatsApp, SMS).
    4. Opening a thread shows the customer's message and an AI-drafted reply highlighted in a translucent glass container with "Approve" or "Edit" buttons.
    5. Action is 1-tap, optimized for one-handed thumb use on a 375px screen.

  - **AI Agent Integration**:
    - **Triage Agent**: Listens to the `ConversationUpdated` event, evaluates intent (Lead, Support, Spam), and tags the thread.
    - **Drafting Agent**: Retrieves context from `Customer360` and `InteractionTimeline` to draft a highly personalized, accurate reply for the owner's review.

  - **Key Design Decisions**:
    - Normalize all incoming messages into a single `ConversationMessage` entity with a `channel_type` enum to abstract platform complexity from the UI.
    - Use PostgreSQL `SKIP LOCKED` for processing incoming webhooks reliably to avoid race conditions.

  **Implementation Prompt**:
  *Objective*: Implement the core data model, API endpoints, and mobile-first UI for the Unified Inbox.
  *CUJ*: An owner receives an Instagram DM and an SMS from different customers. They open the app on their phone (375px), see both messages in one unified list, and can approve an AI-drafted reply to the SMS with one tap.
  *Acceptance Criteria*:
  1. Define the `ConversationThread` and `ConversationMessage` schemas with strict multi-tenant RLS in PostgreSQL.
  2. Implement an internal gRPC/REST API to fetch unified threads for a tenant.
  3. Build the Flutter/PWA UI for the unified inbox list and thread detail view, strictly adhering to the 375px mobile-first mandate and translucent glass styling.
  4. Implement at least 5 Playwright E2E tests verifying the inbox flow using real database seeding (ZERO mock data).

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
