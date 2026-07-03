issue_title: "Architect & Implement the Omnichannel Agentic Work Triage Feed"
issue_description: |
  # Research Report & Design Doc: Omnichannel Agentic Work Triage Feed

  ## Mission Queue Protocol Brief

  **Problem Statement**
  Business owners like Maya (baker) and Carlos (handyman) are overwhelmed by context switching across Instagram DMs, email inquiries, payment alerts, and booking notifications. They suffer from "App Tax Fatigue" and often miss critical actionable items because their work demand is scattered. Currently, the industry standard (Shopify, Wix) offers dashboards, but not an *assistant-first actionable work feed* that unifies, prioritizes, and drafts actions across all channels.

  **Research Report**
  Competitor analysis of Workbuddy, WeCom, DingTalk, and Notion AI shows a clear trend towards unified intelligence. However, traditional SMB tools remain siloed.
  - **Shopify/Wix:** Rely on scattered apps for bookings and chat.
  - **WeCom:** Great at chat, but weak on commerce integration for small independent operators.
  - **OHC Opportunity:** By leveraging the `Work Triage` AI capability described in our product architecture, OHC can create a single, unified feed where AI agents not only aggregate events but pre-draft the next action (e.g., "Maya, you have 3 cake inquiries. The Customer Assistant drafted replies, and Operations checked delivery dates. Click here to approve.").

  ## Design Doc

  ### 1. High-Level Architecture (Mermaid)
  ```mermaid
  graph TD
      subgraph Ingress
          IG[Instagram DM Webhook] --> IGW[Omnichannel Webhook Handler]
          EM[Email Inbound] --> IGW
          BK[Booking System] --> IGW
          PM[Payment Gateway] --> IGW
      end

      subgraph Core OHC Backend
          IGW --> |Raw Event| Q[(Postgres Job Queue SKIP LOCKED)]
          Q --> |Dequeue| Triage[Work Triage Agent]
          Triage --> |Context Sync| Mem[(Tenant Memory & Lock)]
          Triage --> |Delegate| Sales[Sales Agent]
          Triage --> |Delegate| Ops[Operations Agent]
          Sales --> |Draft Quote| FeedDB[(Unified Feed DB)]
          Ops --> |Check Calendar| FeedDB
      end

      subgraph Mobile Frontend
          FeedDB --> |REST / WebSocket| UI[Flutter 375px Work Feed]
      end
  ```

  ### 2. Mobile UX Flow (375px First)
  - **Screen 1: The Command Center (Home)**
    - Clean, Apple/Ubiquiti-style hierarchy with translucent materials.
    - Top card: **"Requires Action Today (3)"**
    - List items are not just text; they are interactive "Agent Drafts".
  - **Screen 2: Detail & Action**
    - Tap on "Instagram DM from Sarah: Vegan Cake?".
    - See the chat history context.
    - See the AI-drafted response: "Hi Sarah! Yes, we can do vegan cakes. Would you like our standard vanilla bean?"
    - Action Buttons: `[Approve & Send]` or `[Edit]`.

  ### 3. AI Agent Integration Points
  - **Work Triage Agent:** Runs on ingestion. Classifies the inbound message intent.
  - **Customer & Relationship Agent:** Retrieves Sarah's past order history from tenant memory.
  - **Operations Agent:** Checks Maya's delivery calendar for availability.
  - **Handoff:** The Triage Agent synthesizes the findings and creates a pending `FeedItem` record with `status=requires_owner_approval`.

  ### 4. Key Design Decisions
  - **Row-Level Security (RLS):** All feed items strictly enforce `tenant_id` isolation.
  - **Postgres Job Queue:** We use `SKIP LOCKED` for the agent job processing instead of an external queue to reduce infrastructure complexity while maintaining atomicity.
  - **Zero Trust:** Service-to-service calls (e.g., Triage to Sales agent) must be authenticated via SPIFFE/SPIRE.

  ## Implementation Prompt (For Implementer Agent)
  **User Facing Outcome:** As an owner (like Maya), I want to open the OHC app and see a single, prioritized feed of everything that needs my attention today (messages, unconfirmed bookings, pending quotes), complete with AI-suggested next actions, so I don't have to check 5 different apps.

  **Acceptance Criteria:**
  1. Implement the database schema for the Unified Feed, ensuring strict `tenant_id` RLS.
  2. Create a backend service (REST/JSON) that serves feed items to the frontend.
  3. Build a Flutter mobile UI (375px optimized) that displays these feed items using the OHC Premium Token design system (translucent glass, clear status tokens).
  4. Ensure the UI includes an interactive card for an "Agent Draft" where the user can click an `[Approve]` button.
  5. The entire flow must be covered by a Playwright E2E test starting from a successful login and verifying the presence of actionable feed items.

  **Top 5 Codebase Anomalies to Fix (Discovered during Research):**
  1. `src/server/twillio_voice_test.rs` has a typo in the file name or is incomplete (46 bytes).
  2. `src/server/loyalty_test.rs` is practically empty (98 bytes) and missing assertions.
  3. The `chatwoot` integration in `docker-compose.yml` relies on default passwords (`changeme`); needs a secrets management overhaul.
  4. `test_ui.spec.ts` is only 49 bytes and lacks comprehensive Playwright coverage.
  5. Hardcoded paths or outdated README references to `srcs/` instead of `src/`.

  ## Priority & Scope
  **Priority:** P0 (Critical for the Core OHC Promise)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
