issue_title: "Feature: Autonomous AI-Driven Quote Generation Engine for Service Owners"
issue_description: |
  ## Problem Statement
  Service-based small business owners (like Carlos the handyman or Nora the agency principal) often lose leads because they cannot respond to quote requests quickly enough while working in the field. Current platforms like Shopify or Wix are built around fixed-price products, not dynamic service estimation. Existing quoting tools (like Jobber or HoneyBook) require owners to manually build every estimate, resulting in friction, delayed responses, and lost revenue. We need an assistant that autonomously captures service intent, drafts an accurate quote using historical pricing/context, and presents it to the owner for one-tap approval.

  ## Research Report
  - **Competitor Analysis:**
    - **HoneyBook/Jobber:** Highly functional but manual. They act as blank canvases where the operator must build everything. They lack AI-driven drafting from unstructured data (e.g., a DM saying "need my gutters cleaned on a 2-story house").
    - **Shopify/Wix:** Terrible for service businesses. They force service intent into product variants, frustrating customers and operators alike.
    - **OHC Gap:** OHC currently lacks a dedicated engine to parse unstructured service requests into structured quotes. Implementing this directly addresses Carlos and Nora's core workflows, turning chaotic DMs into actionable revenue.
  - **Persona Focus (Carlos - Handyman):** Carlos receives a text while on a roof. He can't open a laptop to draft an invoice. He needs OHC to read the text, recognize it as a quote request, draft a $150 quote for "Gutter Cleaning (2-story)", and show him an action card to approve it.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request (DM, Form)] --> B[Work Triage Agent]
      B --> C{Is this a service request?}
      C -- Yes --> D[Quote Engine Agent]
      D --> E[Query Tenant Pricing History & Service Catalog]
      E --> F[Generate Structured Draft Quote]
      F --> G[Agent Feed Action Card]
      G --> H[Owner Approval (1-Tap)]
      H --> I[Dispatch Quote via Communication Channel]
      I --> J[Wait for Customer Acceptance & Deposit]
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification:** Owner receives push notification: "New Quote Draft: Gutter Cleaning for Sarah."
  2. **Agent Feed Action Card (Home Screen):** A UniFi-style translucent card displays:
     - Customer intent summary.
     - Line items (e.g., Service: $150, Materials: $20).
     - Total: $170.
  3. **Actions:** Three large touch targets (44x44px minimum): `Approve & Send`, `Edit`, `Discard`.
  4. **Edit Screen:** If tapped, opens a full-screen mobile-optimized form to quickly adjust numbers using native numeric keyboards.

  ### AI Agent Integration Points
  - **Triage Department:** Listens to incoming webhook events and classifies intent (`intent_type: quote_request`).
  - **Sales/Revenue Department (Quote Engine):** Uses RAG against the `TenantServiceCatalog` and `HistoricalInvoices` to predict line items and pricing based on the natural language request.
  - **Operations Department:** Blocks off tentative time on the calendar if the quote implies urgency.

  ### Key Design Decisions
  - **Schema:** Store quotes as a distinct entity linked to a `Customer` and `Opportunity`, separate from finalized `Invoices`.
  - **Approval Gate:** The AI *drafts* the quote but *never* sends it without explicit human approval (Zero-trust for outgoing pricing).
  - **Tenant Isolation:** RAG must strictly scope queries to the specific `tenant_id` to prevent pricing leaks between businesses.

  ## Implementation Prompt
  **Goal:** Build the Autonomous Quote Generation Engine.
  **CUJ:** Carlos receives an Instagram DM: "Can you fix my leaky sink? It's under the kitchen counter." The system must capture this, draft a quote for standard plumbing repair based on his past rates, and surface an Action Card in his Agent Feed. Carlos taps "Approve & Send," and the quote is sent back via DM.
  **Requirements:**
  1. Create the structured data model for `QuoteDrafts`.
  2. Implement the AI prompt architecture for the Quote Engine to parse intent and suggest pricing.
  3. Build the mobile-first (375px) Agent Feed UI Action Card for Quote Approval.
  4. Integrate the approval action with the outbound messaging service.
  5. **Acceptance Criteria:** E2E Playwright test proving the flow from incoming unstructured text to an approved, sent quote.

  ## Top 5 "Does Not Make Sense" Repo Findings (For future optimization)
  1. **Missing Unified Frontend Entry Point:** The Flutter app configuration is deeply nested and lacks a clear, single `main.dart` entrypoint documented in the root for quick onboarding.
  2. **Inconsistent Agent Protocol:** `ohc-builtin-agent` and standard backend services seem to have overlapping responsibilities regarding intent parsing.
  3. **Stale Docker Images:** The `docker-compose.yml` points to some local images that frequently fail to build without undocumented Bazel targets.
  4. **Test Environment Leakage:** Some E2E tests appear to rely on specific external network states rather than robust local adapters.
  5. **Lack of DB Migrations Script:** The local development setup relies on `seed_discovery.sql` but lacks a clear, sequential migration tool (like golang-migrate) for schema evolution.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []