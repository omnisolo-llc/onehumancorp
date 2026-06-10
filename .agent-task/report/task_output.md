issue_title: "Implement Mobile-First Agentic Unified Inbox & Work Triage"
issue_description: |
  ## Title
  Implement Mobile-First Agentic Unified Inbox & Work Triage

  ## Problem Statement
  Operators like Maya (baker) and Carlos (handyman) receive customer inquiries across fragmented channels: Instagram DMs, SMS, WhatsApp, and website forms. Managing these scattered messages on a 375px phone screen is overwhelming. There is no unified view, and manual triage takes time away from actual service work. Messages are disconnected from bookings, invoices, and historical customer context. They need an AI work assistant that automatically centralizes, groups, tags, and drafts replies to all inquiries, prioritizing urgent actionable requests immediately.

  ## Research Report
  - **Competitive Analysis:** Solutions like WeCom and DingTalk provide robust enterprise messaging but lack small business operational context. Shopify Inbox focuses primarily on web chat and email, lacking deep Instagram/WhatsApp operational orchestration. Wix inbox is passive, requiring manual categorization.
  - **Platform Gap:** Currently, OHC lacks a unified, multi-channel intake pipeline that ties directly into the AI Job Queue and Agent capabilities. Messages exist in silos or are missed entirely.
  - **Need:** An architecture that ingests webhooks from external platforms (Meta for Instagram/WhatsApp, Twilio for SMS), normalizes them into a single isolated entity with strict `tenant_id` isolation, and immediately dispatches an AI Agent to triage, extract intent (e.g., "quote request", "booking change"), and draft a suggested reply.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Webhooks: IG, WA, SMS] -->|Ingest| B[API Layer - gRPC/REST]
      B --> C[(PostgreSQL: normalized messages)]
      C --> D[AI Job Queue - SKIP LOCKED]
      D --> E[Triage Agent Worker]
      E --> F[Extract Intent & Draft Reply]
      F --> G[Redis Pub/Sub]
      G -->|WebSocket| H[Flutter Mobile Client 375px]
  ```

  ### UI Wireframes & Mobile UX Flow (375px target)
  - **Navigation:** Bottom tab bar featuring Home, Inbox, Calendar, Sales.
  - **Inbox Tab:** A single feed of unified message cards. Each card displays the customer avatar, channel icon (IG, WA), a snippet of the latest message, and an AI-generated priority tag (e.g., "Urgent", "Quote Request").
  - **Thread View:** Standard chat interface. At the bottom, a translucent glass panel (using OHC Premium Tokens) displays the "AI Suggested Reply" alongside a prominent 44x44px "Approve & Send" button.
  - **Flow Example:**
    1. Push notification: "New IG DM from Sarah (Cake order)".
    2. Owner taps the notification, opening the OHC app directly to the Thread View.
    3. The thread displays Sarah's message. Below it, the AI assistant has already drafted: *"Hi Sarah! I'd love to bake a vegan cake for you. We have availability on the 14th. It will be $50. Shall I send a deposit link?"*
    4. Owner taps the "Approve & Send" button. The message is dispatched back through the API to the external network platform.

  ### AI Agent Integration Points
  - **Triage Agent:** Triggered asynchronously on new message creation. Uses Gemini Pro (or fallback provider) to classify intent, extract entities (dates, products), and generate a context-aware draft reply.
  - **Context Memory:** The agent fetches past interactions and orders for the customer using a unified identifier via the tenant-scoped memory layer.

  ### Key Design Decisions
  - **Unified Data Model:** A single abstract entity model ensures the frontend UI does not need channel-specific logic to render the inbox.
  - **Async AI Processing:** AI triage must happen asynchronously via the Job Queue (`SKIP LOCKED` pattern) to ensure webhook ingestion remains lightning fast and resilient.
  - **Visual Distinction:** The AI suggestion box must float above the standard chat input using macOS-style Translucent Glass styling. This clearly separates AI-generated drafts from human input and passes the "grandmother test".

  ## Implementation Prompt
  Implement the Agentic Unified Inbox core capabilities:
  1. Define the persistence entity representing a normalized cross-channel message with strict Row-Level Security (RLS) on `tenant_id`.
  2. Create the webhook ingestion endpoints in the Go API server for generic message payloads.
  3. Integrate the AI Job Queue to trigger a triage classification task when a new message is saved.
  4. Build the Flutter UI (or Web PWA equivalent) for the Inbox feed and Thread View, rigorously optimized for 375px mobile screens. Include the AI Suggested Reply translucent card with an "Approve & Send" button.
  **Acceptance Criteria:** A message POSTed to the ingestion API appears instantly in the mobile UI. The AI job queue picks it up, generating a draft reply visible in the thread. The owner can click "Approve & Send" to finalize and clear the notification.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
