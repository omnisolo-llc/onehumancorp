issue_title: "[Architectural Gap] Implement Redis-backed Agent Draft Queue with Mobile-First Action Cards"
issue_description: |
  ## Title
  Implement Redis-backed Agent Draft Queue with Mobile-First Action Cards

  ## Problem Statement
  Owners like Maya (baker) and Carlos (handyman) receive messages across channels (Instagram, SMS, WhatsApp). Currently, the Agent Feed architecture lacks a unified, resilient queue to hold AI-drafted responses pending owner approval. Without this, drafted messages are lost if the mobile client disconnects, or if multiple agents try to draft simultaneously, resulting in a confusing, uncoordinated experience that forces the owner to manually review multiple tabs instead of a single prioritized feed.

  ## Research Report
  - **Shopify/Wix Inbox**: Consolidate messages but lack proactive AI drafting capabilities. They aggregate but do not automate context-aware replies.
  - **Zendesk/Intercom**: Too complex for single-owner operations, focusing on agent-to-agent ticketing rather than AI-to-owner approval flows.
  - **OHC Gap**: OHC's vision promises an "Approve, Edit, or Discard" flow on a 375px mobile screen. We lack the backend infrastructure (Redis queue) to persist these drafts and the frontend components (Action Cards) to display them reliably.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Source as Webhook (IG/SMS)
      participant Ingest as Event Ingestion
      participant LLM as Agent (Gemini/GPT-4o)
      participant Queue as Redis Draft Queue
      participant Mobile as OHC App (375px)

      Source->>Ingest: New Customer Message
      Ingest->>LLM: Trigger Draft Generation
      LLM->>Queue: Push Draft (tenant_id, context, drafted_reply)
      Queue-->>Mobile: Push Notification / WebSocket
      Mobile->>Queue: Fetch Pending Drafts
      Mobile-->>Mobile: Display Action Card
      Mobile->>Queue: Approve/Edit/Discard Action
      Queue->>Source: Dispatch Final Message
  ```

  ### UI Wireframes / Mobile UX Flow
  1. **Home Feed (375px)**: Unified list of pending action cards.
  2. **Action Card Component**:
     - **Header**: Customer Name & Channel (e.g., "Instagram DM").
     - **Body**: AI-drafted reply (e.g., "Yes, vegan cakes are available!").
     - **Actions**: "Approve" (Primary, Translucent green), "Edit" (Secondary), "Discard" (Destructive).
  3. **Interaction**: Swiping right approves, swiping left discards. Tapping opens an edit modal.

  ### AI Agent Integration
  - Agents (e.g., The Ambassador) use the Draft Generation prompt.
  - They push outputs to `ohc:drafts:{tenant_id}:{draft_id}` in Redis.
  - The Action Required Queue strictly enforces tenant boundaries.

  ## Implementation Prompt
  **Goal:** Build the Redis-backed Draft Queue and the mobile-first Action Card UI component.
  **CUJ:**
  1. As an owner, I open the OHC mobile app.
  2. I see a prioritized list of AI-drafted replies in my feed.
  3. I tap "Approve" on a draft.
  4. The draft is removed from the queue and dispatched.
  **Acceptance Criteria:**
  - Implement Redis queue for drafted messages with tenant isolation.
  - Build `ActionCard` Flutter/React component adhering to OHC Premium Token library (macOS glass + UniFi layout).
  - Ensure 100% usability on a 375px viewport (no horizontal scrolling, 44x44px touch targets).
  - Add Playwright E2E tests verifying the queueing and approval flow.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
