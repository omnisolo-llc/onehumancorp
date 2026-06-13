issue_title: "Research: Mobile-First Quote Generation and Acceptance Flow"
issue_description: |
  # Research Report: Agentic Mobile-First Quote Generation and Acceptance

  ## Core Problem
  Small business owners like Carlos (Handyman) and Nora (Agency Principal) spend too much time creating, formatting, and tracking quotes for service requests. Existing tools are often disjointed from the core operations or overly complex, lacking an integrated, mobile-first, one-click quoting solution powered by AI.

  ## Market & Competitor Analysis
  - **Traditional Methods**: Many SMBs use Word/Excel templates or dedicated tools like Joist or Invoice2go, adding friction and "App Tax".
  - **Platform Offerings**: Shopify focuses heavily on products; while service plugins exist, they feel bolted-on. QuickBooks/Xero offer quoting but are accounting-first, lacking operational CRM integration.

  ## OHC Solution Strategy: The Sales Agent
  The OHC platform must provide an integrated "Sales Agent" capability. When an inquiry is received (e.g., via Work Triage), the Sales Agent should be able to instantly draft a quote, which the owner can review and approve directly from their 375px mobile feed.

  ## Architectural & Design Decisions
  - **Data Model**: Extend `quotes` and `quote_line_items` schema to support dynamic variant pricing, deposit requirements, and direct integration with the `invoices` and `bookings` tables.
  - **AI Integration**: The Sales Agent must parse incoming requests (e.g., "Need a drywall repair next Tuesday"), check the ledger/catalog for standard pricing, and generate a structured Quote intent.
  - **Mobile UX Flow**:
    1. **Intake**: A new quote request appears in the Agent Feed.
    2. **Draft Presentation**: The feed presents a summary card with the drafted quote line items and a total.
    3. **Action**: The owner can hit "Approve & Send" or "Edit".
    4. **Customer View**: Customers receive a mobile-optimized web link (via SMS/email) to review and accept the quote, triggering an automatic invoice generation or booking confirmation.

  ## Implementation Prompt (For Implementer Agent)
  Implement the end-to-end mobile-first quote generation and acceptance flow.
  - Enhance the backend API to support generating quotes via LLM intents, utilizing the Sales Assistant agent.
  - Build a responsive (375px optimized) mobile UI component for the owner feed that displays draft quotes clearly.
  - Develop the customer-facing quote acceptance page, ensuring it handles the transition from "Quote Accepted" to "Invoice Generated" or "Deposit Required" seamlessly.
  - Guarantee robust multi-tenant data isolation and Zero-Trust security across all API endpoints.

  ## Priority & Scope
  **Priority**: P1 (High)
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
