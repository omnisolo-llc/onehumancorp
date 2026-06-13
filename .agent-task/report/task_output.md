issue_title: "Unified Agentic Inbox System"
issue_description: |
  # Unified Agentic Inbox System

  ## Mission Queue Protocol
  This brief adheres to the Mission Queue Protocol for OneHumanCorp.

  ## Problem Statement
  Small business owners and operators (like Maya the Baker or Fatima the Food Cart Operator) are missing leads and critical communications because their workflow is scattered across Instagram DMs, WhatsApp, SMS, and email. The current platform landscape either requires expensive third-party integrations (Shopify + plugins) or lacks depth entirely (GoDaddy). As noted in our internal research (Pain Point #2), 22% of SMBs report losing leads due to missed Instagram DMs.

  ## Research Report
  - **Competitor Analysis:** Shopify relies heavily on the "Shopify Tax" where users must install and configure third-party apps for unified communication and auto-replies, increasing cost and complexity. Wix and Squarespace offer rudimentary tools, while GoDaddy's Airo focuses mostly on basic branding setup.
  - **Persona Fit:** Maya (Baker) and Carlos (Handyman) require an omnichannel inbox that works flawlessly on a 375px mobile screen. They cannot spend hours configuring API keys or designing email workflows.
  - **The Gap:** OHC lacks a unified, event-driven intake mechanism that centralizes multi-channel communications into a single interface and allows the `Customer Success Agent` to autonomously draft and send replies.

  ## Design Doc
  ### Mobile UX Flow (375px view)
  1. **Triage Feed:** The owner opens the OHC mobile app and sees a "Work Triage" feed. This feed aggregates messages from all channels (IG, WhatsApp, Email).
  2. **Agent-Drafted Replies:** For common inquiries ("What are your hours?", "Do you do vegan cakes?"), the message card already contains a suggested reply drafted by the Customer Success Agent, based on tenant context.
  3. **One-Tap Action:** The owner reviews the drafted reply and taps a single "Approve & Send" button.
  4. **Manual Override:** The owner can easily tap into the text field to edit the reply before sending.

  ### AI Agent Integration Points
  - **Event Bus:** Webhook ingestion services (for IG, WhatsApp) publish events to an internal message bus.
  - **Work Triage:** A centralized service that subscribes to the event bus, persists the message, and triggers the `Customer Success Agent`.
  - **Customer Success Agent:** Evaluates the message against the tenant's memory and knowledge base to draft an immediate, context-aware reply, placing it in a "Pending Approval" state.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram / WhatsApp Webhooks] -->|Ingest| B[Webhook Ingestion Service]
      B -->|Publish Event| C[Event Bus / Queue]
      C -->|Subscribe| D[Work Triage Service]
      D -->|Trigger| E[Customer Success Agent]
      E -->|Fetch Context| F[Tenant Knowledge Base]
      E -->|Draft Reply| D
      D -->|Sync| G[Mobile PWA / Flutter App]
      G -->|Owner Approves| D
      D -->|Send| H[External Communication API]
  ```

  ## Implementation Prompt
  **Goal:** Build the backend infrastructure and mobile-first UI for the Unified Agentic Inbox.
  **CUJ (Critical User Journey):**
  1. Maya receives a DM on Instagram asking about vegan options.
  2. The webhook is received by OHC and appears in Maya's "Work Triage" feed.
  3. The `Customer Success Agent` drafts a reply based on Maya's menu.
  4. Maya opens the OHC app (375px viewport), sees the pending reply, and taps "Approve". The message is sent back to the customer via Instagram.
  **Acceptance Criteria:**
  - Implement a generic webhook ingestion endpoint that standardizes messages.
  - Create the `Work Triage` data model (ensure strict multi-tenant isolation).
  - Implement the UI in the Tauri app with a mobile-first (375px) approach.
  - The `Customer Success Agent` must correctly trigger and generate a draft reply.
  - Full E2E Playwright test covering the user approving a drafted reply.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
