issue_title: "[Research] AI Unified Zero-Click Intake & Proposal Generation"
issue_description: |
  # Research Report: AI Unified Zero-Click Intake & Proposal Generation

  ## Problem Statement
  For non-technical owner/operators providing services—specifically Carlos (Handyman) and Nora (Agency Principal)—the intake-to-proposal pipeline is a severe bottleneck. Currently, demand arrives via fragmented channels (Instagram DMs, email, website forms, SMS). The operator must manually read the request, switch to a quoting tool or word processor, calculate estimates based on memory or scattered spreadsheets, draft a proposal, and send it back to the client. This manual overhead causes delayed responses, lost leads, and consumes hours of administrative time that should be spent doing the actual work.

  ## Research Report
  ### Competitive Analysis
  - **Joist & Invoice2go:** Popular with field service workers (like Carlos), but they are purely reactive form-fill tools. The user must manually input line items.
  - **Dubsado & HoneyBook:** Used by agencies (like Nora) for CRM and proposals, but require heavy initial setup (templates, workflows) and are complex on mobile devices.
  - **Shopify/Wix:** Lack robust native service quoting capabilities, forcing users to rely on expensive third-party apps.
  - **Current OHC State:** Missing an autonomous, cross-channel intake parser that natively connects to a dynamic proposal engine.

  ### Findings & Market Gaps
  Operators do not want a better form-builder; they want a virtual assistant that acts on their behalf. A 73% drop-off in lead conversion occurs if a quote is not provided within 24 hours. The optimal solution is an **Invisible AI Automation** that intercepts inbound requests, structures the data, queries the operator's business context (pricing rules, past similar jobs, availability), and drafts a complete proposal for simple 1-tap approval on a 375px screen.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Client
      participant InboundWebhook as Ingestion (Webhook/API)
      participant TriageAgent as Work Triage Agent
      participant SalesAgent as Sales & Revenue Agent
      participant DB as Postgres (Tenant Ledger)
      participant Operator as Mobile App (375px)

      Client->>InboundWebhook: Sends request (DM/Form/Email)
      InboundWebhook->>TriageAgent: Raw text & context
      TriageAgent->>TriageAgent: Classify Intent (Service Request)
      TriageAgent->>SalesAgent: Trigger Proposal Generation
      SalesAgent->>DB: Query Pricing Rules & Past Projects
      DB-->>SalesAgent: Context (Rates, Availability)
      SalesAgent->>DB: Save Draft Proposal (Pending Approval)
      SalesAgent->>Operator: Push Notification: "New Quote Drafted"
      Operator->>Operator: Review on Mobile (375px)
      Operator->>SalesAgent: Tap "Approve & Send"
      SalesAgent->>Client: Deliver PDF/Link Quote
  ```

  ### UI Wireframes & Screen Flow (375px Mobile-First)
  1. **Notification State:** Push notification arrives: "New Lead: Broken Pipe (Estimated $250). Tap to review quote."
  2. **Feed View (`/feed`):** A new Translucent Glass Action Card appears at the top of the owner's feed.
     - **Header:** Client Name & Source (e.g., "From Instagram DM")
     - **Summary:** AI-generated 1-sentence summary of the request.
     - **Draft Quote Preview:** Total price prominently displayed, with key line items visible.
  3. **Edit/Approval View (`/quote/draft/:id`):**
     - Full-screen modal over the feed.
     - Line items are editable with native mobile number pads.
     - Bottom sticky bar with a primary `bg-[#0066FF]` button: "Approve & Send Quote" and a secondary button "Edit Manually".

  ### AI Agent Integration Points
  - **Work Triage Agent:** Intercepts the raw inbound payload. Prompts Gemini Pro to extract `client_name`, `service_requested`, `urgency`, and `contact_info`.
  - **Sales & Revenue Agent:** Takes the structured intent. Uses RAG against the specific `tenant_id` database (looking at `services`, `pricing_rules`, and previous `proposals`) to construct a highly accurate draft quote.

  ### Key Design Decisions
  - **Zero-Trust Multi-Tenancy:** The RAG context and drafted proposals must strictly enforce PostgreSQL row-level security using `tenant_id` to prevent data leakage between operators.
  - **Optimistic UI:** When the operator taps "Approve", the UI must immediately reflect success, queueing the actual delivery job in the background (PostgreSQL SKIP LOCKED queue) for resilience against flaky mobile networks.
  - **Human-in-the-Loop:** AI never sends a quote without explicit operator approval (to protect revenue accuracy), fulfilling the OHC promise of "AI does useful work, owner stays in control."

  ## Implementation Prompt
  **Target Persona:** Carlos (Handyman) and Nora (Agency)
  **Objective:** Implement the "AI Unified Zero-Click Intake & Proposal Generation" backend pipeline and mobile approval UI.
  **Task for Implementer:**
  1. **Backend:** Create the API endpoints and async job processors to receive an unstructured lead text, invoke the AI provider (Gemini/MiniMax) to extract structured fields, and generate a `ProposalDraft` record in the database linked to the `tenant_id`.
  2. **Agentic Logic:** Write the system prompts for the Triage and Sales agents to ensure they accurately estimate line items based on existing tenant pricing data.
  3. **Frontend (Tauri/Flutter):** Build the 375px mobile-first Action Card for the Agent Feed and the Quote Review modal. Ensure it uses the OHC Premium Token library (Translucent Glass materials, 16px borders, 44x44px touch targets).
  4. **Testing:** Write end-to-end Playwright tests that simulate a webhook ingestion, verify the draft appears in the UI, and simulate the owner clicking "Approve & Send".

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
