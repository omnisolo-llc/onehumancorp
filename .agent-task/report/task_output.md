issue_title: "Architectural Deep Dive: Multi-Tenant AI Work Triage & Unified Inbox Engine"
issue_description: |
  ## 1. Live Service UI Gap Audit & Problem Statement
  During our initial discovery and system audit (running the local docker-compose stack and reviewing the UI source code like `src/ui/next/src/app/pos/terminal/page.tsx`), we identified a critical architectural gap. While the platform has integrated Chatwoot for customer support and built strong offline POS capabilities (Terminal), there is no centralized, mobile-first "Assistant Shell" for unified Work Triage.

  Small business owners and operators (like Maya the baker and Carlos the field service owner) are overwhelmed by fragmented inbound channels (Instagram DMs, SMS, WhatsApp). Existing solutions force the owner to manually monitor a shared inbox (which feels like an "admin portal"). There is a critical gap in OHC: a native, multi-tenant unified inbox that doesn't just collect messages, but acts as an **AI Work Triage Engine**—automatically classifying intent, checking inventory/availability, drafting replies, and presenting a prioritized feed of "Next Actions" for the owner.

  ## 2. Research Report
  - **Codebase & Docs Audit**: Review of the OHC repository (`deploy/docker-compose.yml`, `docs/business/market_research/`) shows basic chat support (`chatwoot` integration) but lacks a native event bus for inbound omni-channel work triage handled by the built-in AI agents. The platform lacks a structured Multi-Tenant Inbox schema connected directly to the AI job queue (PostgreSQL `SKIP LOCKED`).
  - **Competitor Systems Audit**: Shopify uses "Shopify Inbox", but it's largely reactive. WeCom and DingTalk provide robust communication but lack autonomous SMB workflow generation (e.g., turning a chat into a quoted invoice automatically).
  - **The Gap**: OHC needs a centralized `WorkTriage` pipeline that ingests external webhooks, normalizes them into a standard `DemandEvent`, and routes them to the AI Assistant for pre-processing before the owner even sees them.

  ## 3. Design Doc
  ### System Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Inbound Channels
          IG[Instagram DMs]
          WA[WhatsApp]
          Web[Web Forms]
      end

      subgraph OHC API Edge
          WebhookHandler[Omnichannel Webhook Gateway]
      end

      subgraph Core Processing
          DemandTable[(Tenant Demand Events)]
          JobQueue[(AI Job Queue - SKIP LOCKED)]
      end

      subgraph AI Capabilities
          TriageAgent[Work Triage Agent]
          SalesAgent[Sales Assistant]
      end

      subgraph OHC Mobile Client
          OwnerFeed[Unified Owner Feed - 375px]
      end

      IG --> WebhookHandler
      WA --> WebhookHandler
      Web --> WebhookHandler

      WebhookHandler -->|Normalize & Persist| DemandTable
      DemandTable -->|Trigger Event| JobQueue

      JobQueue -->|Dequeue Task| TriageAgent
      TriageAgent -->|Classify Intent| SalesAgent

      SalesAgent -->|Draft Reply/Quote| DemandTable
      DemandTable -->|Real-time Sync| OwnerFeed
  ```

  ### Mobile UX Flow (375px First)
  - **Screen 1: The Daily Brief (Home)**: The owner sees an AI-prioritized list on their 375px mobile view: "3 Urgent Booking Requests," "2 Custom Order DMs."
  - **Screen 2: Triage Card Detail**: Maya taps a DM. The screen shows the customer's message ("Do you do vegan cakes for this Saturday?"). Below it, the AI has already drafted a response based on her inventory.
  - **Screen 3: 1-Tap Actions**: At the bottom of the viewport are large (44x44px minimum) touch targets: [Approve & Send], [Edit Draft], [Ignore].
  - **Zero Trust & Security**: Row-level tenant isolation in PostgreSQL (`tenant_id`) ensures Maya never sees Carlos's leads.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Triggered on every new `DemandEvent`. Evaluates context and routes to the appropriate departmental capability.
  - **Memory & RAG**: Agents query tenant-scoped PostgreSQL vector embeddings (`pgvector`) to recall past interactions with the specific customer.
  - **Distributed Locks**: Uses Redis Redlock (`ohc:lock:{tenant_id}:customer:{customer_id}`) to ensure agents don't duplicate processing for rapid-fire inbound messages.

  ## 4. Implementation Prompt
  **Feature Name**: Multi-Tenant AI Work Triage Engine & Unified Owner Feed
  **Target Persona**: Maya (Home Baker) & Carlos (Field Service Owner)

  **Outcome**:
  Implement the backend data models, API endpoints, and mobile UI to ingest omnichannel messages into a unified `DemandEvent` table, trigger the Work Triage AI Agent to draft replies, and expose a mobile-optimized (375px) "Owner Feed" where the user can 1-tap approve AI drafts.

  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1. An inbound message webhook is simulated/received by the backend.
  2. The backend stores the message securely with PostgreSQL RLS (`tenant_id`).
  3. The AI Job Queue processes the message via the `Work Triage Agent`, drafting a contextual reply based on mock tenant inventory/calendar.
  4. The owner opens the mobile web app (375px viewport). The new message appears in the feed with the AI draft visible.
  5. The owner clicks "Approve & Send" (44x44px button). The system marks the event as resolved.
  6. **Zero Mock Data in UI**: The feed must be populated from the real local PostgreSQL database, seeded via documented migration/seed paths.
  7. **Testing Verification**: Must include 100% unit test coverage for new backend code, and a Playwright E2E test (`src/e2e/work_triage.spec.ts`) automating the login, feed rendering, and approval of an AI-drafted message using the real stack.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
