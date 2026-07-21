issue_title: "Architecture Design: AI-Automated Instant Invoicing & Estimate System"
issue_description: |
  # Research Report: AI-Automated Instant Invoicing & Estimate Architecture

  ## 1. Problem Statement
  Service-based and independent professionals (e.g., Carlos the handyman, Nora the agency principal) need a way to quickly issue estimates and invoices, often while on the move or speaking directly with clients. Currently, existing tools (like Quickbooks, Xero, or standalone invoice apps) require tedious manual data entry and are completely disconnected from the core operational flow (like customer CRM, calendar bookings, and follow-ups). The owner wastes time context-switching and chasing overdue payments.

  ## 2. Research Report
  - **Market Context**: General e-commerce platforms (Shopify, Wix) treat sales as immediate cart checkouts. Service professionals need an asynchronous flow: Request -> Estimate -> Approval -> Invoice -> Payment -> Receipt. Standalone tools like Freshbooks or Wave do this well but lack agentic, deeply integrated intelligence.
  - **The OHC Opportunity**: Integrate quoting and invoicing natively into the OHC platform. By combining this with AI Agents (Finance, Sales, Operations), OHC can automate the generation of line items from unstructured text/voice (e.g., "Invoice John for 3 hours of design work and the custom logo package"), auto-follow-up on unpaid invoices, and reconcile payments seamlessly.
  - **Competitor Gaps**:
    - *Shopify*: Primarily built for physical products and immediate checkout. Invoicing (Draft Orders) exists but is clunky for service estimates.
    - *Quickbooks/Freshbooks*: Excellent accounting but not an "assistant". They don't autonomously draft proposals based on a client conversation history or project scope.
    - *Stripe Invoicing*: Powerful API and dashboard, but highly technical or requires jumping out of the main business management app.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Work Intake: DM/Email/Voice] -->|Parsed by Work Triage Agent| B{Identify Intent: Quote/Invoice}
      B --> C[Finance & Sales Agents]
      C -->|Context Retrieval| D[Customer CRM & Pricing Catalog]
      C -->|Draft Generation| E[Estimate/Invoice Draft Record]
      E --> F[Owner Mobile App]
      F -->|Review & Approve: 375px UX| G[Stripe Invoicing API]
      G -->|Send Link via Email/SMS| H[Customer Payment Portal]
      H -->|Payment Success Webhook| I[OHC Core Backend]
      I -->|Status Update| J[Operations Agent]
      J -->|Notify Owner / Next Steps| F
  ```

  ### Data Model (PostgreSQL)
  - `Invoice`: Core entity (status: draft, pending, paid, overdue, cancelled), linked to a `Customer` and `Tenant`.
  - `InvoiceLineItem`: Individual line items (quantity, unit price, description, optional link to `CatalogItem`).
  - `Estimate`: Pre-invoice proposal. Has a status (draft, sent, accepted, rejected). Can be converted 1:1 into an `Invoice`.
  - `PaymentEvent`: Ledger of payments against an invoice (partial payments, deposits).

  ### AI Agent Coordination
  - **Work Triage / Customer Assistant**: Understands incoming client requests ("Can you send me a quote for fixing the sink?").
  - **Sales / Finance Agent**: Translates the request into a structured `Estimate` draft, pulling pricing from the `Pricing Catalog`. Automatically identifies missing information and asks the owner for clarification. Handles drafting automated, polite follow-up emails for overdue invoices.
  - **Operations Agent**: Once an estimate is approved or an invoice is paid, it schedules the work block on the calendar or updates the project board.

  ### Mobile UX Flow (375px)
  1. **Triage Feed**: Owner sees an agent-generated card: "Drafted an estimate for Carlos (Sink Repair: $150)."
  2. **Review Screen**: Tapping the card opens a clean, thumb-friendly modal showing the estimate details. Owner can tap to edit line items or adjust pricing with a native numpad.
  3. **Action & Send**: "Approve & Send" button triggers the Stripe API and sends an SMS/Email to the client.
  4. **Tracking**: The "Finance" tab shows a simple, visual breakdown of Outstanding vs. Paid invoices, without complex accounting jargon.

  ## 4. Implementation Prompt
  **Feature Name**: Native AI-Assisted Estimating and Invoicing
  - **User Story**: As a service business owner, I want my AI assistant to draft invoices and estimates based on my client conversations so I can review, adjust, and send them directly from my phone in under 30 seconds.
  - **Implementation Requirements**:
    1. Create the `Estimate`, `Invoice`, and `InvoiceLineItem` database tables with tenant isolation.
    2. Implement the gRPC/REST API layer for CRUD operations on estimates and invoices.
    3. Integrate with Stripe Invoicing API for payment collection and link generation.
    4. Build the Mobile-First (375px) Flutter UI for reviewing, editing, and approving AI-drafted invoices.
    5. Ensure the Finance Agent is connected to the Stripe Webhooks to automatically update invoice status to "Paid".
  - **Note**: Do NOT prescribe specific low-level libraries beyond what is standard in the OHC stack (Go, Postgres, Stripe, Flutter).

  ## Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []