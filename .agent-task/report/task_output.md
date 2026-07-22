issue_title: "Implement Unified AI Triage Feed Architecture"
issue_description: |
  # Research Report: Unified AI Triage Feed Architecture

  ## 1. Problem Statement
  Small business owners like Maya (Home Baker) and Carlos (Field Service Owner) face a daily deluge of fragmented information: Instagram DMs, SMS service inquiries, Stripe payment notifications, new calendar bookings, and AI agent drafts. Existing platforms (Shopify, Wix) treat these as separate silos or passive notification bells, causing the owner to lose track of what needs immediate attention. The core promise of OHC is to guide users from "unclear work -> clear next action in minutes." Currently, our platform lacks a centralized, intelligent triage feed that unifies, prioritizes, and proposes the next logical action for these disparate events.

  ## 2. Research Report & Gap Analysis
  - **Competitor Analysis:**
    - **Tencent Workbuddy / WeCom:** Successfully unify communications and operational tasks, but they lean heavily on enterprise configurations.
    - **Shopify / Wix:** Notifications are passive alerts (e.g., "New Order #123"). They do not suggest actions or draft replies automatically in a unified inbox.
    - **Notion AI / Microsoft Copilot:** Great for knowledge work, but disconnected from POS, inventory, and physical service bookings.
  - **The OHC Gap:** The current OHC system has separate ledger and job queue structures (`ohc_universal_ledger`, `ohc_job_queue`) but lacks a materialized, AI-curated "Work Feed" that the Work Triage agent can manage. Without this, the owner must manually navigate multiple screens on their 375px mobile device to figure out what to do.

  ## 3. Design Doc: Architecture & Flow
  ### Data Model & Entity Relationship (PostgreSQL)
  We need a new table structure to represent the unified feed, distinct from the raw job queue or ledger.
  - `ohc_triage_feed`:
    - `id`, `tenant_id`, `source_type` (e.g., 'dm', 'booking', 'alert').
    - `priority_score` (calculated by the AI).
    - `content_summary` (plain-language summary for the owner).
    - `suggested_action` (e.g., 'Approve Draft', 'Send Quote', 'Dismiss').
    - `status` (pending, acted_upon, dismissed).
  - Multi-tenant isolation using RLS on `tenant_id`.

  ```mermaid
  erDiagram
      TENANT ||--o{ OHC_UNIVERSAL_LEDGER : tracks
      TENANT ||--o{ OHC_TRIAGE_FEED : manages
      OHC_UNIVERSAL_LEDGER ||--o{ OHC_TRIAGE_FEED : generates

      OHC_TRIAGE_FEED {
          string id PK
          string tenant_id FK
          string source_type
          int priority_score
          string content_summary
          string suggested_action
          string status
          timestamp created_at
      }
  ```

  ### Architecture Flow
  ```mermaid
  sequenceDiagram
      participant External as Social Media / Payment API
      participant Ledger as OHC Universal Ledger
      participant WorkTriage as Work Triage Agent (Gemini)
      participant Feed as OHC Triage Feed
      participant UI as OHC Mobile UI (375px)

      External->>Ledger: New Event (DM/Payment)
      Ledger->>WorkTriage: Trigger Webhook / Event
      WorkTriage->>WorkTriage: Evaluate Context & Priority
      WorkTriage->>Feed: Insert Row with Drafted Reply
      UI->>Feed: Poll / Stream for Tenant Updates
      Feed-->>UI: Display Actionable Card
      UI->>Feed: Owner Taps "Approve"
  ```

  ### AI Agent Integration ("Work Triage" Agent)
  - **Trigger:** Webhooks, new row inserts in the ledger, or scheduled polling.
  - **Action:** The Work Triage agent (using Gemini Pro) consumes the raw event, evaluates it against the owner's context (e.g., business hours, inventory levels), and inserts a prioritized row into `ohc_triage_feed` with a drafted response or suggested action.

  ### Mobile UX Flow (375px)
  - **First Screen Experience:** The "Command Center" feed.
  - **Cards:** Each feed item is a translucent, Apple-style card.
    - *Example Card:* "Cake Inquiry from Sarah. (Instagram DM)". Below it, a drafted reply: "Yes, we can do vegan! $50 deposit required."
    - *Buttons:* Large (44x44px minimum) touch targets: [Approve & Send], [Edit].
  - **Offline/Resilience:** Feed items are cached locally using the PWA/Flutter client for low-data mode accessibility.

  ## 4. Implementation Prompt
  **Feature Name:** Unified AI Triage Command Center
  **Target Personas:** Maya (Home Baker), Nora (Agency Principal)

  **User Outcome:** When Maya opens the OHC app, she sees a single feed of prioritized tasks (3 new DMs, 1 missed deposit). Each task already has a suggested action or drafted reply ready for a 1-tap approval.

  **Priority:** P0
  **Estimated Scope:** Large

  **Critical User Journey (CUJ):**
  1. A new DM webhook arrives. The Work Triage agent processes it, queries the inventory, drafts a reply, and creates a high-priority item in the `ohc_triage_feed`.
  2. Maya opens the OHC mobile app (375px viewport).
  3. She sees the new feed card at the top: "New custom cake request. Drafted reply ready."
  4. She taps "Approve & Send". The UI optimistically removes the card from the pending feed and dispatches the action to the backend.

  **Acceptance Criteria for Implementer:**
  - Create the `ohc_triage_feed` table with strict Row Level Security.
  - Implement the gRPC/REST endpoints to fetch the feed for the current tenant.
  - Build the Flutter/Tauri UI for the feed using OHC Premium Tokens (translucent materials, clean hierarchy) tailored for a 375px screen.
  - Include Playwright E2E tests verifying that a generated feed item can be viewed and "acted upon" in the UI. Ensure ZERO mock data is used in the UI layer.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []