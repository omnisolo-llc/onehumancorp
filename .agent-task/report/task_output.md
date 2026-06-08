issue_title: "Instant Localized Invoicing & Autonomous Tax Compliance Architecture"
issue_description: |
  # Instant Localized Invoicing & Autonomous Tax Compliance Architecture

  ## Problem Statement
  Small business owners and operators like Nora (Agency Principal) and Carlos (Field Service Owner) face significant friction when generating invoices, managing deposits, and ensuring local tax compliance. Current platforms either require complex manual setup for tax rates or push users to expensive third-party accounting integrations. For non-technical owners, "Sales tax is confusing" is a high-anxiety area. They need an invoicing system that instantly generates localized, compliant invoices and payment links directly from a mobile device, with an AI agent handling the background tax calculations and follow-ups.

  ## Research Report
  Our audit of competitor systems (Shopify, Wix, Stripe Invoicing) reveals that while Stripe offers powerful tax and invoicing APIs, integrating them into a unified, mobile-first experience for SMBs is often left to the user.
  - **Shopify/Wix:** Rely heavily on plugins for advanced invoicing and local tax compliance.
  - **Stripe Invoicing:** Powerful backend, but the dashboard is too complex for a mobile-first user like Carlos in the field.
  - **OHC Opportunity:** By leveraging the "Finance & Decision Assistant" agent, OHC can automatically compute local taxes based on the service location, generate a premium PDF/web invoice, and handle SMS/email payment reminders autonomously. This eliminates the need for external accounting apps and reduces setup paralysis.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - 375px] -->|Creates Job/Quote| B(API Gateway)
      B --> C{Finance Agent}
      C -->|Location/Service Data| D[Tax Calculation Engine]
      D -->|Tax Rates| C
      C --> E[Invoice Generator]
      E --> F[(PostgreSQL: Invoices)]
      E --> G[Stripe Payment Links API]
      G --> H[Customer Notification via SMS/Email]
      H --> I[Payment Webhook Handler]
      I --> J[Ledger & Analytics]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Quote Creation:** Carlos finishes a repair. On his phone (375px viewport), he taps "Create Invoice". He enters the service details ("Pipe repair, $150").
  2. **AI Tax & Localization:** The Finance Agent automatically detects the job location (e.g., Austin, TX) and applies the correct 8.25% sales tax. A transparent glassmorphism card appears: "Texas Sales Tax (8.25%) applied automatically."
  3. **One-Tap Send:** Carlos reviews the generated total. He taps a full-width (44px height) primary button: "Send Payment Link".
  4. **Customer Experience:** The customer receives an SMS with a web link to a mobile-optimized Stripe Checkout page.
  5. **Auto-Reconciliation:** Once paid, the invoice status turns green (`#34C759`) in Carlos's feed.

  ### AI Agent Integration Points
  - **Finance Assistant:** Calculates localized tax, formats the invoice, and categorizes the revenue.
  - **Customer Assistant:** Drafts the SMS/Email message containing the payment link and automatically schedules a polite follow-up if unpaid after 3 days.

  ### Key Design Decisions
  - **No Manual Tax Setup:** Abstract tax logic entirely behind the Finance Agent to eliminate user anxiety.
  - **Mobile-First Invoicing:** Ensure the invoice creation flow requires typing only the essential job details; everything else (customer info, tax, formatting) is contextually pulled by the AI.
  - **Asynchronous Payment Webhooks:** Rely on Stripe webhooks to update the local PostgreSQL ledger to ensure eventual consistency even if the mobile client loses connectivity.

  ## Implementation Prompt
  **Feature:** Autonomous Localized Invoicing Flow
  **Persona:** Carlos (Handyman) & Nora (Agency Principal)
  **CUJ:** From the mobile dashboard, the user creates a new invoice for a customer. The system autonomously calculates the local tax based on the customer's profile/job location, generates a Stripe Payment Link, and drafts an SMS/Email to the customer. Upon payment, the invoice is marked as paid in the database and the user's feed is updated.
  **Acceptance Criteria:**
  - Build a responsive 375px UI for invoice generation.
  - Integrate the Finance Agent to dynamically apply mock/real tax rates based on context.
  - Generate a secure payment link and store the invoice record in the multi-tenant PostgreSQL database.
  - Ensure 100% unit test coverage for the invoice generation logic.
  - Provide a Playwright E2E test verifying the invoice creation and status update flow.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
