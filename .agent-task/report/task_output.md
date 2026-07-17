issue_title: "Implement 'The Ambassador' Agent - Native Social Inbox Auto-Responder"
issue_description: |
  # The Ambassador Agent Implementation Brief

  ## Problem Statement
  Based on our market research (`docs/business/market_research/[research]_ohc_smb_market_dynamics_agentic_workflows.md`), a massive pain point for micro-SMB owners (like "Maya the Baker") is manual management of customer inquiries across social media channels (e.g., Instagram DMs). These operators spend countless hours responding to repetitive queries about pricing, availability, and policies instead of actively producing or fulfilling orders. Existing solutions force them to either buy expensive third-party apps or switch out of their primary business workflows, defeating the purpose of a unified work assistant.

  ## Research Report
  - **Competitor Analysis:** Shopify relies heavily on its App Store for this functionality, resulting in an "app tax" for basic inbox features. Shopify's "Sidekick" is a merchant-facing reactive chatbot, rather than a proactive customer-facing agent. Wix and Squarespace offer basic contact form integrations but lack autonomous social media chat handling.
  - **User Needs:** SMB operators need an AI assistant that acts as a 24/7 customer success representative. This "invisible staff member" must ingest multi-channel messages, retrieve business-specific context (inventory, FAQs, operating hours), and draft ready-to-send replies.
  - **Gap Identification:** OHC currently lacks a proactive, unified message triage feed that auto-drafts customer replies based on the tenant's actual operating data.

  ## Design Doc
  ### Architecture
  The Ambassador Agent will operate as a Modular Capability Plugin connected to the OHC Core Orchestrator.

  1. **Ingestion Layer:** Webhook endpoints for Meta Graph API (Instagram/WhatsApp). Inbound messages normalize into a standard `WorkIntakeEvent`.
  2. **Orchestrator Routing:** The Hub delegates the event to the Ambassador Capability Plugin.
  3. **Context Hydration (RAG):** The agent fetches real-time tenant context from PostgreSQL via the MCP Gateway (e.g., checking if vegan cakes are in stock).
  4. **Draft Generation:** The LLM (Gemini Pro) evaluates the user's intent and generates a conversational draft reply.
  5. **Review & Dispatch:** The draft is saved to PostgreSQL (`MessageDraft` table) and published to the mesh. The mobile UI updates instantly.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant MetaAPI as Social API (Meta)
      participant Webhook as OHC Backend (Webhook)
      participant Hub as Hub (Orchestrator)
      participant Agent as Ambassador Agent
      participant RAG as DB/VectorStore (RAG)
      participant Mobile as OHC Frontend (Mobile)
      participant Maya as Owner (Maya)

      Customer->>MetaAPI: Sends DM ("Vegan cake available?")
      MetaAPI->>Webhook: POST /webhooks/social
      Webhook->>Hub: Publish WorkIntakeEvent
      Hub->>Agent: Delegate Task (Draft Reply)
      Agent->>RAG: Query Context (Inventory, FAQs)
      RAG-->>Agent: Context Results (In Stock)
      Agent->>Agent: Generate Draft
      Agent->>Hub: Persist Draft & Notify Mesh
      Hub->>Mobile: WebSocket Push (New Draft)
      Mobile->>Maya: Push Notification & Triage Feed Update
      Maya->>Mobile: Review Draft -> Tap "Approve & Send"
      Mobile->>Webhook: PUT /api/messages/approve
      Webhook->>MetaAPI: Deliver Message
      MetaAPI->>Customer: Message Received
  ```

  ### Mobile UX Flow (375px First)
  - **Work Triage Dashboard:** The first screen after login shows "Requires Attention". A high-priority card displays: "Drafted reply for @customer (Instagram)".
  - **Review Screen:** Tapping the card transitions to a clean, glassmorphic review pane.
    - Top segment: The customer's original message bubble.
    - Middle segment: The AI's drafted response in a translucent textarea box, editable if needed.
    - Bottom segment: A large (minimum 44x44px) primary "Approve & Send" button, and a secondary "Edit" button.
  - **Post-Action State:** Tapping "Approve" triggers a fluid transition. The card disappears from the Triage feed with a success toast.

  ### AI Agent Integration Points
  - **System Prompt:** Instructs the LLM to adopt a polite, concise business assistant persona scoped strictly by `tenant_id`.
  - **Memory:** Session-based short-term memory for active multi-turn chats, with long-term memory for returning customers.

  ## Implementation Prompt
  Implement the full-stack flow for the Ambassador Agent's draft approval UI and backend integration.

  **Persona:** Maya the Baker (Custom cake shop).

  **Critical User Journey (CUJ):**
  1. Maya logs into the OHC web app on her smartphone (viewport 375px).
  2. She views the Work Triage feed and sees a notification: "Drafted reply for @customer".
  3. She taps the notification. The review screen shows the customer asked about vegan options.
  4. She sees the AI's drafted response confirming availability based on her actual inventory.
  5. She taps "Approve & Send". The UI shows a success state, and the task clears from her feed.

  **Acceptance Criteria:**
  1. **Frontend UI:** Build the mobile-first (375px) Work Triage feed and Draft Review screen using the OHC Premium Design System (glassmorphism tokens, Tailwind).
  2. **Backend API:** Implement the REST/gRPC endpoints to fetch pending drafts and approve them (`GET /api/messages/drafts`, `PUT /api/messages/{id}/approve`).
  3. **Data Integrity:** UI must use the real API flow. No hardcoded mock data in the frontend code. Include a migration/seed script to generate a pending draft for verification.
  4. **E2E Testing:** Create a complete Playwright E2E test (`src/e2e/ambassador_agent.spec.ts`) that verifies the exact CUJ above using the running local stack. The test must pass locally via `bazel test //...`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
