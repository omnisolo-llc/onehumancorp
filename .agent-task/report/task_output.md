issue_title: "Implement AI-Powered Unified Intake & Intelligent Proposal Generation Architecture"
issue_description: |
  # Mission Queue Protocol: AI-Powered Unified Intake & Intelligent Proposal Generation Architecture

  ## Problem Statement
  For service-oriented owners like Nora (Agency Principal) and Carlos (Field Service Owner), managing inbound leads across multiple channels (website forms, emails, DMs) is fragmented and time-consuming. Currently, OHC lacks a unified intake engine that can not only capture demand but also actively interpret client requests and autonomously draft tailored proposals, quotes, and project tasks. Without this, owners have to manually synthesize information, switch contexts, and manually draft estimates, slowing down their time-to-revenue and increasing the cognitive load.

  ## Research Report
  Our competitive analysis indicates that modern platforms (e.g., HubSpot's Breeze, HoneyBook, Dubsado) thrive on seamless lead-to-proposal workflows. AI-native tools like Relevance AI enable autonomous agentic teams to parse inbound data and generate actionable sales outputs. OHC's current architecture has basic message triage but lacks a structured protocol to transition an unstructured inquiry (e.g., an Instagram DM about a kitchen remodel) into a structured proposal/estimate with line items, deposit terms, and associated project tasks. We need an architectural layer where the Customer Assistant and Sales Assistant agents collaborate to transform intake data into an actionable proposal card in the owner's Agent Feed.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant C as Customer (Web/IG/Email)
      participant I as Intake Webhook/API
      participant Q as AI Job Queue (PgSQL)
      participant CA as Customer Assistant Agent
      participant SA as Sales Assistant Agent
      participant DB as Central Ledger (PgSQL)
      participant O as Owner (Mobile UI)

      C->>I: Submits inquiry/DM
      I->>Q: Enqueue Intake Event
      Q->>CA: Process Event & Extract Intent
      CA->>DB: Store Lead & Context (Tenant Scoped)
      CA->>SA: Trigger Proposal Draft Request
      SA->>DB: Read Pricing/Catalog Context
      SA->>DB: Generate & Store Draft Proposal
      SA->>O: Push Action Card to Agent Feed
      O->>SA: Review & Approve Draft
      SA->>C: Send Final Proposal/Payment Link
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification:** The owner receives a push notification and sees a new Action Card at the top of their Agent Feed.
  2. **Action Card:** The card displays a summary: "New Lead: Kitchen Remodel. Draft quote ready for $3,500 based on standard pricing."
  3. **Detail View:** Tapping the card opens a unified view showing the original customer message at the top and the structured draft quote below (line items, deposit, dates).
  4. **Action:** Big, thumb-friendly buttons at the bottom (min 44x44px): `Approve & Send`, `Edit Quote`, `Discard`.
  5. **Editing:** If the owner taps `Edit Quote`, a mobile-optimized form appears allowing them to adjust line items or terms before sending.

  ### AI Agent Integration Points
  - **Customer Assistant (Intake parsing):** Responsible for extracting structured data (budget, timeline, services requested) from unstructured text.
  - **Sales Assistant (Proposal generation):** Uses tenant-scoped pricing data and the extracted intent to construct a realistic quote.
  - **Agent Feed:** Coordinates the handoff, ensuring the owner gets a cohesive, ready-to-approve package instead of scattered notifications.

  ### Key Design Decisions
  - **Asynchronous Processing:** Using the PostgreSQL `SKIP LOCKED` job queue ensures high reliability and handles transient LLM provider failures gracefully.
  - **Tenant Isolation:** All extracted data and generated drafts must enforce strict row-level security based on `tenant_id`.
  - **Owner-in-the-Loop:** Agents draft the proposal, but nothing is sent to the customer without explicit owner approval via the Action Card.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your objective is to implement the backend services, AI agent coordination, and mobile-first UI for the Unified Intake & Intelligent Proposal Generation system.

  **Acceptance Criteria:**
  1. Build the data models for `IntakeRequest` and `ProposalDraft` ensuring strict multi-tenant isolation via RLS.
  2. Implement the AI agent logic where the Customer Assistant parses an intake and passes structured data to the Sales Assistant to draft a proposal.
  3. Develop the mobile-first (375px) UI components for the Action Card and the Proposal Review screen, strictly adhering to the Translucent Glass materials and UniFi-style layouts.
  4. The UI must contain zero mock data; the flow must be driven by real backend states.
  5. Provide at least one comprehensive Playwright E2E test verifying the full flow: an inbound request arrives, the draft is generated and displayed on the UI, and the user can approve it.
  6. Achieve 100% unit test coverage for the new logic and ensure all existing `bazel test //...` checks pass.

  Do not prescribe specific database schemas or API signatures in this brief; design and build the optimal structures to satisfy the criteria above.

  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []