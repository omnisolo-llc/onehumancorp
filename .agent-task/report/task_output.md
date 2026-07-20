issue_title: "Architectural Design: Unified Omnichannel AI Inbox & Triage Engine"
issue_description: |
  # Unified Omnichannel AI Inbox & Triage Engine

  ## Problem Statement
  Owners like Maya (custom baker using Instagram DMs) and Carlos (handyman using texts and emails) face fragmented communication channels. Messages, leads, and customer requests arrive across multiple platforms (WhatsApp, IG DMs, email, website chat), creating a chaotic, unprioritized feed. Without an intelligent system to triage these inputs, owners miss opportunities, lose track of customer context, and spend excessive time manually parsing intent (e.g., "Is this a lead, a complaint, or a spam message?"). They need a unified inbox where all messages are centralized, automatically triaged by an AI assistant, and presented with actionable drafts and context.

  ## Research Report
  - **Tencent WeCom:** Leads the market with deep WeChat integration, allowing enterprise-grade communication to feel seamless for consumer interactions. However, it requires significant setup and is ecosystem-locked.
  - **Zendesk & Intercom:** Highly effective omnichannel support, but they are built for support teams (ticketing, complex routing, dashboards). They feel too administrative and "corporate" for small business operators like Maya or Carlos, who need an assistant, not a CRM suite.
  - **Shopify Inbox:** Provides a good unified view for commerce but lacks true, autonomous AI triage and drafting that handles multi-step operational tasks (like scheduling a service or recovering a deposit).
  - **Opportunity:** OHC can differentiate by delivering an assistant-first inbox. Instead of just listing messages, the engine must use LLMs (Gemini Pro) to triage intent, retrieve tenant-specific memory (past orders, preferences), and draft contextual responses or suggest operational actions (e.g., "Send quote link").

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant External as Webhooks (IG, WA, Email)
      participant Ingest as OHC Ingestion Layer
      participant Queue as TaskDB (pg)
      participant KAIROS as KAIROS Orchestrator
      participant Agent as Triage Agent (Gemini)
      participant Vector as AutoDream (pgvector)
      participant Mesh as Teammate Mesh (Redis)
      participant UI as Owner UI (Mobile)

      External->>Ingest: Receive raw message
      Ingest->>Queue: Enqueue normalized `message_received` task
      Queue->>KAIROS: Dispatch task
      KAIROS->>Agent: Triage message intent
      Agent->>Vector: Retrieve customer history & context
      Vector-->>Agent: Return context
      Agent->>Agent: Generate draft response & suggest next action (e.g., Quote)
      Agent->>Queue: Save triaged thread & draft
      Queue->>Mesh: Publish `thread_updated` event
      Mesh->>UI: Realtime update via WebSocket
  ```

  ### Multi-Tenant Isolation
  - All webhooks hit tenant-isolated ingestion endpoints.
  - Redis Pub/Sub channels for Teammate Mesh are strictly partitioned by `tenant_id` (`ohc:mesh:{tenant_id}:inbox`).
  - `pgvector` lookups for customer memory use Row-Level Security (RLS).

  ### Mobile-First UI Flow (375px)
  1. **Triage Feed:** The default view. Messages are not sorted strictly chronologically but by *AI Priority* (Urgent Leads > Active Conversations > Spam).
  2. **Conversation View:** Tapping a thread shows the message history. A prominent, translucent bottom sheet displays the AI's suggested draft or action (e.g., a "Create Quote" button).
  3. **Action:** The owner taps "Send Draft" or edits it using the native mobile keyboard. The UI must be fully functional without horizontal scrolling.

  ### Visual Mandate (Translucent Glass)
  The Inbox UI components MUST adhere to the premium visual mandate:
  ```css
  .inbox-card {
    backdrop-filter: blur(20px) saturate(200%);
    background: rgba(255, 255, 255, 0.03);
    font-family: 'Outfit', 'Inter', sans-serif;
    border: 1px solid rgba(255, 255, 255, 0.08);
  }
  ```

  ## Implementation Prompt
  Implement the backend API endpoints, the KAIROS triage agent integration, and the Flutter/React frontend components for the Unified Omnichannel AI Inbox.

  **Acceptance Criteria:**
  1. Create a webhook ingestion service that standardizes incoming payloads from at least two mock channels (e.g., "SMS" and "IG_DM").
  2. Implement an AI Agent worker that processes new messages, classifies intent (Lead, Support, General), and generates a draft response using tenant context.
  3. Build the mobile-first (375px) Inbox UI adopting the Translucent Glass standard. It must consume the real-time Teammate Mesh events to update the feed without reloading.
  4. Ensure all database interactions utilize Row-Level Security by `tenant_id`.
  5. Include E2E Playwright tests verifying a message flows from ingestion to the UI feed, and that the AI draft is visible and actionable by the owner.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
