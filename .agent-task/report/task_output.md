issue_title: "[research] Autonomous Instant Localized Invoicing & Ledger Engine"
issue_description: |
  # OHC Autonomous Instant Localized Invoicing & Ledger Engine

  ## Problem Statement
  Service-based and agency operators, like **Nora (Agency Principal)** and **Carlos (Field Service Owner)**, suffer from disjointed cash flow operations. Existing tools treat invoicing as a manual, isolated task involving detached PDF generation, clunky data entry, and manual follow-ups for late payments. Often, owners complete the work but delay invoicing due to the friction of generating estimates, converting them to invoices, tracking payments, and reconciling them with their business ledger—especially while operating from a mobile device (375px viewport) in the field. They need an invoicing system that acts proactively: drafting the invoice right after work is completed, automatically requesting payments, offering localized multi-currency support, and seamlessly syncing with a central ledger.

  ## Research Report
  - **Competitor Systems Audit:**
    - **Shopify & Wix:** Primarily designed for upfront physical/digital product sales. B2B and post-service invoicing is an afterthought, often requiring bulky third-party apps with their own separate pricing tiers.
    - **Stripe Invoicing / QuickBooks:** Provide powerful APIs and dashboards, but lack the "assistant-first" and mobile-first approach. Their interfaces require considerable manual configuration, switching apps, and understanding complex financial terminologies.
    - **Square Invoices:** Excellent mobile usability, but locked closely into their specific POS hardware ecosystem without broader autonomous agentic support (e.g., automated AI-drafted follow-ups based on customer relationship context).
  - **Market Gap (OHC Opportunity):**
    - The core gap is the absence of an integrated, autonomous ledger that drives invoicing as a natural extension of completed tasks. For instance, when Carlos completes a "Repair Job" task on his mobile app, an agent should instantly prepare a detailed invoice with a tap-to-pay link. Furthermore, localized requirements (e.g., multiple currencies, regional tax rules) hold SMBs back from expanding. OHC can bridge this by providing a unified ledger model combined with autonomous invoice drafting.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Work/Project Completion Event] --> B{Work Triage Agent}
      B --> C[Finance Agent - The Accountant]
      C --> D[Draft Localized Invoice]
      D --> E[Multi-Tenant Ledger DB]
      D --> F[Mobile Agent Feed UI]
      F -->|1-Tap Approve| G[Stripe/Payment Gateway]
      G --> H[Customer Receives SMS/Email with Payment Link]
      H --> I[Payment Success Webhook]
      I --> J[Reconcile Ledger & Notify Owner]
  ```

  ### Mobile UX Flow (375px First)
  1. **Trigger:** User completes a job or task in the OHC app.
  2. **Agent Feed Card:** The "Finance Agent" immediately pushes a card to the Unified Agent Feed: "Job completed for Smith Residence. Drafted invoice for $450. [Review & Send]".
  3. **Invoice Review (Translucent Glass UI):** Tapping the card opens a streamlined, mobile-friendly invoice preview (no PDFs required on mobile). It highlights the total, line items, and due date.
  4. **Approval:** A large bottom-sheet button (minimum 44x44px target): "Approve & Send via SMS".
  5. **Status Tracking:** The invoice enters a "Pending" state in the finance tab, where the agent will autonomously follow up if unpaid after a configured duration.

  ### AI Agent Integration Points
  - **Work Triage / Operations Agent:** Detects when a project or service booking transitions to a "completed" state and triggers the invoicing intent.
  - **Finance Agent ("The Accountant"):** Generates the invoice line items based on project details, past quotes, and inventory/parts used. Applies relevant localized taxes.
  - **Customer Assistant ("The Ambassador"):** Drafts the personalized SMS/email accompanying the invoice and handles any customer inquiries or requests for installment payments via natural language.

  ### Data Model & System Design Constraints
  - **Entity Boundaries:** `LedgerTransaction`, `Invoice`, `InvoiceLineItem`, `PaymentIntent`.
  - **Multi-Tenancy:** All entities must strongly enforce `tenant_id` via PostgreSQL Row Level Security (RLS) to guarantee complete isolation.
  - **Scalability & Currency:** Use precise decimal types (or integer-based smallest currency units, e.g., cents) to prevent floating-point errors. Support a `currency_code` field natively for localization.

  ## Implementation Prompt
  **Feature Outcome:** Implement the core backend models and mobile-first UI for the Autonomous Invoicing flow. When a service task is marked completed, the system must trigger an event that drafts an Invoice entity. The UI must present this as an actionable card in the Unified Agent Feed, allowing a 1-tap approval to finalize the invoice and generate a mock payment link.

  **Critical User Journey (CUJ):**
  1. Login as Carlos (Field Service Owner) on a simulated 375px mobile viewport.
  2. Mark an existing service booking as "Completed".
  3. Navigate to the Unified Agent Feed.
  4. Observe the newly surfaced action card: "Draft Invoice Ready: $XXX".
  5. Tap "Approve & Send".
  6. Verify the system transitions the invoice state to "Sent" and creates the corresponding Ledger Transaction record in the database.

  **Acceptance Criteria:**
  - Database schema includes strongly isolated (RLS) `Invoice` and `LedgerTransaction` tables.
  - UI fits strictly within a 375px width, utilizing OHC Premium Tokens (translucent glass styling, correct Apple/UniFi radiuses).
  - A full Playwright E2E test covers the complete CUJ without mocking network or database calls.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
