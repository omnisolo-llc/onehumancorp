issue_title: "Instant Localized Invoicing & Multi-Currency Architecture"
issue_description: |
  # Instant Localized Invoicing & Multi-Currency Architecture

  ## Problem Statement
  Small business owners and independent professionals (like Nora the agency principal or Leo the music tutor) increasingly serve a global client base. Generating invoices manually in different currencies with correct local tax logic is tedious, error-prone, and distracting from core work. Existing tools require complex setup for multi-currency handling and often lock critical localization features behind expensive enterprise tiers. When a client requests an invoice in EUR while the owner operates in USD, the owner has to manually calculate exchange rates and format the invoice correctly.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Stripe Invoicing / Stripe Billing:** Powerful but often too technical for a non-technical SMB to configure perfectly across different locales without relying on third-party integrations. It supports multi-currency but requires manual selection and configuration for each customer.
  - **QuickBooks / Xero:** Provide multi-currency support but are heavyweight accounting tools rather than simple, unified work assistants. They pull the owner away from their main workflow.
  - **FreshBooks / Wave:** Good for simple invoicing, but lack the deeply integrated, agentic workflow of OHC where the same assistant that drafts a project proposal can instantly generate the localized invoice without breaking context.
  - **OHC Opportunity:** Leverage AI agents (specifically Finance/Accountant and Customer Success) to automatically detect client locales from CRM data or email context, dynamically generate invoices in the correct local currency, apply appropriate tax rules, and handle instant currency conversion logic at the edge. The owner only needs to tap "Approve Invoice."

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Request/Proposal Approved] -->|Event| B(The Ambassador Agent)
      B -->|Extracts Client Locale & Amount| C[Event Mesh]
      C --> D(The Accountant Agent)
      D -->|Query Exchange Rate API| E[Currency Conversion Service]
      E --> D
      D -->|Generate Invoice Payload| F[Stripe Billing/Invoicing API]
      F --> G[Unified Ledger DB - PostgreSQL]
      D -->|Draft Invoice Ready| H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|Owner Taps Approve| J[Omnichannel Dispatcher]
      J -->|Send PDF/Link| K[Client Email/SMS]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** A prominent card appears in the daily triage feed: "Draft Invoice: €1,500 for Project Alpha (Nora's Client in Berlin)".
  - **Interaction:** Tapping the card opens a detailed view of the drafted invoice. It clearly shows the original amount (e.g., $1,600 USD) and the converted localized amount (€1,500 EUR based on current rates). It highlights any applied local VAT/Taxes.
  - **Action Buttons:** Primary: "Approve & Send". Secondary: "Edit Details", "Change Currency".
  - **Visual Design:** Follows OHC Premium Token library. Clean, translucent materials. The invoice preview is a scalable, readable mobile component, not just a pinch-to-zoom PDF.

  ### AI Agent Integration Points
  - **Finance Assistant (The Accountant):** Automatically triggered when a proposal is approved or a milestone is reached. Queries real-time exchange rates, applies tenant-specific tax configurations for the target locale, and formats the invoice payload.
  - **Customer Success Assistant (The Ambassador):** Drafts the personalized email/message accompanying the invoice in the client's preferred language.

  ### Key Design Decisions
  - **Auto-Detection:** The system uses client context (address, past payments, communication language) to default to the optimal currency, removing the manual selection step for the owner.
  - **Unified Ledger Consistency:** All transactions are recorded in the tenant's base currency in the PostgreSQL `Ledger` table, with the foreign currency amount and exchange rate stored for accurate reporting and reconciliation.

  ## Implementation Prompt
  **User-Facing Outcome:** As an agency principal, when I finish a project for a client in France, my OHC assistant automatically drafts an invoice in Euros with the correct VAT applied. I review the card on my phone, tap "Approve," and it's sent instantly.
  **CUJ & Acceptance Criteria:**
  1. A project milestone is marked as complete, triggering an event in the mesh.
  2. The Accountant agent intercepts the event, identifies the client's locale as "fr-FR", and queries the exchange rate from the base currency (e.g., USD) to EUR.
  3. The agent generates a draft invoice in EUR and places it in the `ActionRequiredQueue`.
  4. The owner views the draft invoice on the mobile UI (375px), seeing both the EUR total and the estimated USD equivalent.
  5. The owner approves the invoice, which calls the Stripe API to finalize the invoice and dispatches it via email.
  6. Provide Playwright E2E tests: A user logs in, completes a mock milestone, sees the drafted localized invoice card in the feed, taps "Approve", and the system records the transaction correctly in the database.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
