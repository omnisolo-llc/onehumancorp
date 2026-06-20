issue_title: "Architecture: AI-Agentic Unified Intake & Triage Gateway"
issue_description: |
  # Mission Queue Protocol: AI-Agentic Unified Intake & Triage Gateway

  ## Problem Statement
  Small business owners like Maya (baker), Carlos (handyman), and Nora (agency principal) receive demand across scattered channels—Instagram DMs, WhatsApp, SMS, web forms, and emails. They currently suffer from "inbox paralysis," losing high-intent leads because they cannot manually monitor, triage, and route these messages while actively performing their work. They need a centralized, AI-driven Intake & Triage Gateway that acts as a true work assistant: reading incoming messages, understanding the intent (e.g., "is this a new cake order, a complaint, or spam?"), extracting context, and routing it to the appropriate downstream agent (Quoting, Scheduling, or Customer Service) without the owner lifting a finger.

  ## Research Report
  - **Market Context**: Platforms like Shopify Inbox, WeCom, and HubSpot attempt unified inboxes, but they force the owner to act as the router. Tencent Workbuddy and Feishu excel at bringing context to chat, but lack autonomous SMB workflow bridging.
  - **Competitor Gaps**: GoDaddy and Wix offer basic form-to-email routing, but zero AI intent classification. Shopify Sidekick is store-bound and doesn't handle off-platform DMs effectively.
  - **User Evidence**: SMBs report a 38% frequency of "Instagram DM Overload" causing missed sales (per `ohc_smb_platform_research_report.md`). They don't just want an inbox; they want the inbox to *do the work*.
  - **Codebase Audit**: We have `src/server/services/omnichannel` and `src/server/services/chat`, but lack a dedicated, high-performance orchestration layer (the "Triage Gateway") that sits in front of these channels, applies an AI intent classifier, and asynchronously queues structured tasks for other departments (like `src/server/services/quoting`).
  - **Target Outcome**: An event-driven gateway that ingests webhooks/messages from all channels, uses a fast LLM pass to categorize intent (Lead, Support, Spam, Booking), and drops a structured event into the `AI Job Queue` (PostgreSQL `SKIP LOCKED`).

  ## Design Doc
  ### Mobile UX Flow (375px)
  1. **Triage Feed**: Owner opens the app to the "Action Feed" (not an inbox). A card reads: *"3 new custom cake requests from IG. Drafts ready."*
  2. **Review & Approve**: Tapping the card shows the original DM, the AI-extracted details (Date: Oct 10, Vegan, Budget: $100), and a drafted response with a payment link.
  3. **One-Tap Action**: Owner taps "Send & Create Quote". The gateway handles the cross-department orchestration.
  4. **Glassmorphism UI**: Uses translucent cards with clear status tokens (e.g., green dot for "High Intent Lead") following OHC Premium Token library.

  ### AI Agent Integration
  - **The Triage Agent (Scout/Front-desk)**: Subscribes to the omnichannel event bus. Uses a strict JSON-schema prompt to classify intent (`intent_type: ENUM`, `confidence_score: FLOAT`, `extracted_entities: JSON`).
  - **Handoff Protocol**: If intent is `booking_request`, Triage Agent drops a job for the `Scheduler Agent`. If `custom_quote`, routes to `Quoting Agent`. All coordination uses Redis distributed locks to prevent double-processing.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Channel as IG/WhatsApp
      participant Gateway as Omnichannel Gateway
      participant Triage as Triage AI Agent
      participant Queue as AI Job Queue (PG)
      participant Dept as Specialist Agent (Quote/Schedule)

      Channel->>Gateway: Incoming Message Webhook
      Gateway->>Triage: Async Classification Job
      Triage-->>Gateway: Returns Intent & Extracted Context
      Gateway->>Queue: Enqueue Task (e.g., Generate Quote)
      Queue->>Dept: Process Job & Draft Response
      Dept->>Owner App: Push Notification / Action Card
  ```

  ### Multi-Tenant & Security Invariants
  - All incoming webhooks authenticated and mapped to `tenant_id`.
  - Postgres queue strictly scoped with `ENABLE ROW LEVEL SECURITY` on `tenant_id`.
  - Redis distributed lock pattern: `ohc:lock:{tenant_id}:triage:{message_id}`.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to build the "Unified Intake & Triage Gateway" capability.
  1. Define the unified ingestion data models in Protobuf/Rust to normalize incoming messages across channels.
  2. Implement the AI Triage worker that consumes from the ingestion stream, calls the LLM provider (using `OHC_LLM_PROVIDER` interface) for intent classification, and enqueues a typed job (using PostgreSQL `SKIP LOCKED`).
  3. Build the primary API endpoint for the Flutter mobile shell to fetch the aggregated "Triage Feed" (cards needing owner action).
  4. Ensure 100% unit test coverage for the intent parsing and routing logic. Provide at least one Playwright E2E test simulating a webhook ingestion that surfaces an actionable card in the UI.

  **Acceptance Criteria:**
  - Ingested message successfully classified by AI.
  - Typed job reliably enqueued.
  - UI correctly displays the pending action card without horizontal scroll on 375px width.
  - Full tenant isolation observed.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, p0]
assignees: []
