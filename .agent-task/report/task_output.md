issue_title: "Implement Autonomous AI Work Triage and Daily Work Generation"
issue_description: |
  # Research Report: AI Autonomous Work Triage & Daily Work Generation for SMBs

  ## 1. Problem Statement
  Small business owners (Maya the Baker, Carlos the Handyman, Fatima the Food Cart Operator) are overwhelmed by incoming demand scattered across Instagram DMs, forms, WhatsApp, and emails. They lack a unified view of what needs attention "today" and struggle to translate informal communication into actionable tasks, bookings, or sales. Legacy platforms like Shopify or Wix are purely reactive dashboards—they wait for the user to manually enter data or complete a rigid checkout flow. OHC needs an AI-native engine that actively triages unstructured inbound signals and automatically synthesizes a prioritized, actionable "Daily Work" feed.

  ## 2. Research & Market Findings
  - **The "Dashboard Paralysis"**: Competitors like Shopify or Square provide analytics and notification lists, but the owner still must figure out the *meaning* of those notifications. "You have 3 new messages" does not help the owner decide *what to do next*.
  - **The Context Gap**: Existing CRMs (HubSpot, Salesforce) require heavy data entry. SMB owners do not have time to tag leads or move Kanban cards.
  - **The Agentic Opportunity**: Leading AI-native platforms (e.g., Lindy.ai, 11x.ai) succeed because they operate autonomously in the background. OHC's unique advantage will be an invisible **Work Triage Agent** that runs continuously, transforming raw inputs (e.g., an Instagram DM saying "Do you do vegan cakes for Saturday?") into structured OHC objects (a pending Quote, a calendar block, and a drafted reply) and bubbling them up into a single, mobile-first Daily Plan view.

  ## 3. Architecture & Design Doc

  ### 3.1. Architecture Diagram
  ```mermaid
  graph TD
      subgraph External Inputs
          IG[Instagram DMs]
          Email[Emails]
          Forms[Web Forms]
      end

      subgraph OHC Backend
          External Inputs -->|Webhook/Polling| InboundRouter
          InboundRouter -->|Raw Payload| TriageQueue[(Redis Queue)]
          TriageQueue --> Worker[AI Work Triage Agent]

          Worker -->|LLM Extraction| EntityEngine

          EntityEngine -->|Create/Update| DB[(PostgreSQL)]
          DB --> |Tenant Data| Tasks
          DB --> |Tenant Data| Messages
          DB --> |Tenant Data| Customers
      end

      subgraph Frontend App
          DB --> |API| MobileApp[Mobile App: Daily Work Feed]
      end
  ```

  ### 3.2. Mobile UX Flow (375px First)
  1. **The "Morning Summary" (Home Screen):**
     - A clean, translucent card layout showing "Today's Focus".
     - E.g., "3 urgent messages, 2 quotes to approve, 1 delivery."
  2. **Actionable Cards:**
     - Tapping a summary item opens a card with context and AI-drafted actions.
     - E.g., A card shows Maya a DM from a customer. Below it, an AI-drafted reply and a button "Approve & Send Quote".
  3. **Zero Data Entry:**
     - The AI has already linked the customer profile and tentatively blocked the calendar. The owner just clicks "Approve" or "Edit".

  ### 3.3. AI Agent Integration Points
  - **Triage Trigger:** A background job (using PostgreSQL SKIP LOCKED or Redis) that processes new inbound communication.
  - **Prompt Strategy (Work Triage):** The LLM receives the raw message and current business context. It outputs structured JSON: `{ "intent": "inquiry", "customer_info": {...}, "suggested_actions": [...] }`.
  - **Handoff:** The Triage Agent creates a `DailyWorkItem` record linked to the tenant and the underlying entity (e.g., a Message or a Booking).

  ## 4. Implementation Prompt
  **To the Implementer Agent:**
  Your mission is to build the core backend infrastructure and mobile-first UI for the "Daily Work Feed" powered by an AI Triage Agent.

  **Key Deliverables:**
  1.  **Data Model:** Design and implement the database schema (PostgreSQL) for `DailyWorkItem` and `InboundSignal`. Ensure strict row-level tenant isolation (`tenant_id`).
  2.  **Triage Engine (Backend):** Implement the background worker that simulates or processes incoming signals (e.g., a mock webhook endpoint for testing), uses the configured LLM to analyze the intent, and generates structured `DailyWorkItem`s.
  3.  **API Layer:** Build the REST/gRPC endpoints to fetch the active Daily Work Feed for a tenant and to execute actions (e.g., "approve drafted reply").
  4.  **Mobile-First UI:** Create a premium, translucent glass-styled "Home" screen in the Flutter/PWA shell that displays the prioritized feed. It must work flawlessly at 375px. Use truthful empty states, not fake data.
  5.  **Verification:** Implement Playwright E2E tests covering the complete journey: injecting a raw signal -> processing by backend -> surfacing in the UI -> owner clicking "Approve". Do not mock internal API calls in the E2E test.

  **Acceptance Criteria:**
  - The UI runs on 375px screens with no horizontal scrolling.
  - The Triage Engine successfully parses a test input and creates a structured work item in the database.
  - `bazel test //...` and the specific E2E tests pass 100%.

  ## 5. Priority & Scope
  - **Priority:** P0 (Critical foundational feature)
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
