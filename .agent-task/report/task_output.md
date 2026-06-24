issue_title: "[research] Autonomous Multi-Currency Invoicing & Tax Ledger"
issue_description: |
  # Research Report: Autonomous Multi-Currency Invoicing & Tax Ledger Architecture

  ## Executive Summary
  This report investigates the current state of invoicing, multi-currency support, and tax calculation for small business owners operating internationally or across state lines. The objective is to design a high-scale, multi-tenant capable architecture for OneHumanCorp (OHC) that autonomously handles currency conversion, localized invoice generation, and tax ledgering using our AI agents, significantly reducing the administrative burden for non-technical users.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  - **Shopify Markets:** Offers powerful multi-currency and localized pricing capabilities but typically requires complex configuration, manual tax nexus setup, and paid apps for B2B invoicing compliance in different regions.
  - **Stripe Invoicing & Tax:** Industry standard for developers, but the raw dashboard is confusing for micro-SMEs (like Maya or Nora). It requires users to understand "Payment Intents," "Webhooks," and "Tax Codes."
  - **The Gap:** Non-technical owners (e.g., Nora the Agency Principal) need to send invoices to clients in different countries. They struggle with calculating the correct VAT/Sales Tax, keeping track of exchange rates, and organizing these for their end-of-year accounting. They need a system that just "does it" in the background.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Nora (Agency Principal) sends proposals and invoices to clients globally. She needs instant invoice generation in the client's local currency, with automated tax compliance, and automatic payment reminders.
  - **The Gap:** Currently, OHC lacks a dedicated, scaleable universal ledger specifically tailored for multi-currency transactions and autonomous tax tracking. Invoices require manual drafting and do not automatically sync with a unified financial memory for the AI.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Sync Protocol
  - **Universal Tax Ledger (PostgreSQL):** A double-entry ledger system where every finalized transaction creates immutable debit/credit entries, tagging the source currency, converted base currency (for the owner), and separated tax amounts.
  - **Edge-Cached Exchange Rates:** Daily or hourly exchange rates fetched and cached via Redis to ensure all AI agents use consistent conversion data when drafting proposals.
  - **Multi-Tenant Isolation:** Row-level security (RLS) on all ledger and invoice tables, utilizing `tenant_id` to guarantee Zero Trust isolation.

  ### AI Agent Coordination
  - **Sales Agent:** When Nora negotiates a deal via the unified inbox, the agent automatically detects the client's location, looks up the current exchange rate, and drafts a proposal/invoice in the client's local currency.
  - **Finance Agent ("The Accountant"):** Continuously monitors the Universal Tax Ledger. It categorizes revenue, calculates accrued tax liabilities (e.g., VAT vs. State Sales Tax), and pushes a monthly summary to the owner's Agent Feed. It also automatically triggers polite payment reminders for overdue invoices.

  ### Mobile-First Implementation
  - **Invoice Review Flow (375px):** A clean card appears in the Agent Feed: "Draft Invoice ready for Client X. Total: €1,500 (approx. $1,620). Send now?". Touch targets for "Approve", "Edit", and "Discard" must be ≥ 44x44px.
  - **Financial Summary:** A simple, plain-language breakdown of pending invoices and estimated tax set-aside, devoid of complex accounting jargon.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Unified Inbox / Sales Negotiation] --> B(Sales Agent)
      B --> C{Detect Client Location & Currency}
      C --> D[Fetch Edge-Cached Exchange Rate]
      D --> E[Generate Multi-Currency Invoice Draft]
      E --> F[Owner Approval via Agent Feed]
      F --> G(Stripe Checkout / Payment Link)
      G -->|Webhook Success| H[Universal Tax Ledger]
      H --> I(Finance Agent: Tax & Revenue Summary)
  ```

  ## 4. Implementation Prompt
  **Feature Name:** Autonomous Multi-Currency Invoicing & Tax Ledger
  **Target Persona:** Nora the Agency Principal
  **Outcome:** Nora can negotiate with international clients, and OHC automatically drafts, converts, and sends localized invoices. The system tracks tax liabilities invisibly and surfaces simple financial summaries on her phone.

  **Next Actions:**
  1. Implement the `UniversalTaxLedger` and `Invoice` data models in PostgreSQL with strict RLS multi-tenant isolation.
  2. Create a background worker to fetch and cache daily exchange rates in Redis.
  3. Develop the Finance Agent capability to parse invoice webhooks, record immutable ledger entries, and calculate tax set-asides.
  4. Implement the Agent Feed mobile UI card (375px optimized) for reviewing and approving AI-drafted invoices.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
