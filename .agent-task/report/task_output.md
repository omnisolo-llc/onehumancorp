issue_title: "Architecture: Predictive Agentic Cash Flow & Ledger Automation"
issue_description: |
  # Research Report: Predictive Agentic Cash Flow & Ledger Automation

  ## 1. Track 1: Architectural Gap & Scaling Discovery

  **Problem Statement:**
  Small business owners (SMBs) consistently rank "managing cash flow" as a top survival challenge. Traditional platforms (Shopify, Wix) provide retrospective reporting—dashboards showing what *has* happened. To understand what *will* happen, owners must export data to QuickBooks or Excel, requiring financial literacy and significant manual effort.

  **The OHC Gap:**
  OHC currently possesses transactional primitives (Ledger, Invoices, Subscriptions) but lacks a proactive, predictive layer. The architecture needs a "Finance Agent" capability that constantly evaluates future liabilities against projected revenue and proactively advises the owner *before* a cash flow crunch occurs.

  ## 2. Track 2: Selected Architecture Deep Dive

  **Business Journey Mapping (Nora - Agency Principal & Priya - Boutique Operator):**
  - **Context:** Nora has $5,000 in upcoming contractor payments (liabilities) but several outstanding client invoices (receivables).
  - **Agent Intervention:** Instead of Nora manually cross-referencing these, the Finance Agent runs a daily async job. It detects a projected $2,000 shortfall for next Thursday.
  - **Actionable Insight:** The agent pushes a card to Nora's Mobile Feed: "Cash flow gap of $2,000 projected for next week. Should I send friendly reminders for your 3 overdue invoices?"
  - **Cross-Agent Coordination:** For Priya (Boutique), the agent might suggest: "You have excess inventory of Summer Dresses. Should the Marketing Agent draft a 15% flash sale to cover next week's rent?"

  **Data Model & Invariants:**
  - **`CashFlowProjection` (Entity):** A time-series aggregation storing future predicted balances.
  - **Sources:** Aggregates `LedgerEntry` (current balance), `Invoice` (pending AR), `Subscription` (predictable MRR), and `Booking` deposits.
  - **Multi-Tenancy:** Row-level security (RLS) is strictly enforced via `tenant_id` on all financial tables.
  - **Job Queue:** Projection calculations are heavy. They must run asynchronously using the PostgreSQL `SKIP LOCKED` job queue pattern to avoid blocking the main API thread.

  ## 3. Track 3: Technical Integrity & Mobile-First Review

  **Mobile-First UX Flow (375px):**
  - The feature surfaces as a **"Financial Health Card"** in the Unified Agent Feed.
  - **Visual Design:** Premium Translucent Glass styling. A simple indicator (e.g., Green/Yellow/Red pulse) provides immediate peace of mind or alerts the user.
  - **Interaction:** Tapping the card reveals a plain-English summary (no complex accounting jargon). Below the summary are primary 44x44px touch targets for AI-proposed mitigations: `[Send Reminders]`, `[Draft Flash Sale]`, `[Review Expenses]`.

  **Performance & Zero Trust:**
  - The aggregation output is cached via Redis (`ohc:cache:{tenant_id}:cashflow_projection`) to ensure sub-100ms load times on mobile.
  - All inter-agent requests (e.g., Finance Agent asking Marketing Agent to draft a sale) pass through internal gRPC with SPIFFE/SPIRE identity verification to guarantee cross-tenant boundary security.

  ## 4. Track 4: Strategic Feature Issue Dispatch (Implementation Prompt)

  **Mission:** Build the Predictive Cash Flow Agent Capability

  **User-Facing Outcome:**
  The owner logs into the OHC mobile app and sees a "Financial Health" card in their daily feed. If a future cash flow dip is detected, the card explains the dip in plain English and offers a one-tap AI mitigation (e.g., "Draft invoice reminders").

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Background worker asynchronously calculates the `CashFlowProjection` for a tenant based on pending invoices and current ledger balance.
  2. The Finance Agent LLM analyzes the projection and drafts a plain-language summary.
  3. The user opens the 375px mobile feed and sees the Financial Health Card.
  4. The user taps a mitigation button (e.g., "Remind Clients"), triggering the Customer Success Agent to draft emails for overdue invoices.
  5. **Verification:** E2E Playwright test must seed a tenant with a low ledger balance and high pending invoices, verify the Financial Health Card appears with a warning state, and confirm the mitigation buttons are fully interactive and trigger the appropriate agent fallback.
  6. **Visuals:** Must use the OHC Premium Token library (Translucent Glass, UniFi layout).

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
