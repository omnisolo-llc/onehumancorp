issue_title: "Unified Agentic Quote-to-Cash & Dynamic Deposit Architecture"
issue_description: |
  # Research Report: Unified Agentic Quote-to-Cash & Dynamic Deposit Architecture

  ## 1. Problem Statement
  Service-based and custom-order small businesses (e.g., Nora the Agency Principal, Carlos the Handyman, Maya the Custom Baker) face massive friction in the "Quote-to-Cash" lifecycle. Currently, they use a fragmented stack: one tool to capture the lead (DMs/forms), another to manually build a quote (Word/PDF or basic software), another to request a deposit (Stripe/Venmo), and a final tool to schedule the work and invoice the remainder. Even in modern platforms, these are treated as separate disjointed objects. The lack of an autonomous, connected pipeline leads to delayed responses, dropped leads, and cash flow bottlenecks.

  ## 2. Research Report (Track 1)
  - **Market Context**: Legacy platforms like Shopify focus almost exclusively on "Add to Cart" for physical products, bolting on "Quotes" via clunky third-party apps (e.g., Globo Request a Quote) that break the native checkout flow. Service-focused CRMs like Jobber or HoneyBook provide quote-to-cash workflows but require heavy manual data entry and are not AI-native—they wait for the owner to act.
  - **The OHC Opportunity**: OHC must differentiate by leveraging its AI Agent framework to make Quote-to-Cash an autonomous, zero-touch process. If Carlos receives a text: "Can you fix a leaky pipe on Tuesday?", the system should instantly draft the quote, generate the deposit link, and reserve the calendar slot, requiring only one tap from Carlos to approve.
  - **Competitor Gaps**:
    - *HoneyBook/Jobber*: Manual quote building; high monthly cost; no AI intent extraction from raw messages.
    - *Shopify*: Fundamentally unsuited for dynamic service quoting without expensive apps.
    - *Wix*: Basic invoicing, but disconnected from lead intent and calendar lock.

  ## 3. Design Doc: Quote-to-Cash Architecture (Track 2 & Track 3)

  ### Data Model (PostgreSQL)
  The core architecture unifies Lead, Quote, Deposit, and Calendar into a single state machine, utilizing strict row-level security (`tenant_id`).
  - **`Quote`**: Upgraded to act as a dynamic agreement. It links to a `Customer`, `Service/Product`, and `AvailabilityBlock`.
  - **State Machine**: `DRAFT` -> `PENDING_APPROVAL` (Owner) -> `SENT` (Customer) -> `DEPOSIT_PAID` -> `WORK_COMPLETED` -> `FULLY_PAID`.
  - **Ledger Integration**: When a Quote reaches `DEPOSIT_PAID`, the Finance Agent automatically generates an `Invoice` for the remaining balance, linked to the original Quote ID.

  ### AI Agent Coordination
  - **Sales Agent ("The Closer")**: Ingests DMs or form submissions, extracts the requested service and budget, and autonomously generates a structured `Quote` draft.
  - **Finance Agent ("The Accountant")**: Monitors Stripe webhooks. When the deposit is paid, it automatically transitions the Quote state and drafts the final invoice for the remaining balance.
  - **Operations Agent ("The Manager")**: Automatically places a "tentative" lock on the calendar when the quote is sent, converting to a "confirmed" lock when the deposit clears.

  ### Mobile UX Flow (375px First)
  1. **Owner View (Triage Feed)**: Carlos opens the OHC app. He sees an Agent Card: "New Request from John. I've drafted a $150 quote for Pipe Repair with a $50 deposit. Approve?" -> [Approve & Send]. Touch targets are > 44px.
  2. **Customer View (The Interactive Quote)**: John receives an SMS link. The link opens a mobile-optimized Glassmorphism web page containing:
     - The itemized quote.
     - A dynamic calendar selector (if the date wasn't finalized).
     - A native Stripe Element / Apple Pay button to pay the $50 deposit instantly.

  ## 4. Implementation Prompt
  **Feature Name**: Unified Agentic Quote-to-Cash Pipeline

  **Target Persona**: Carlos the Handyman & Nora the Agency Principal

  **Outcome**: A seamless pipeline where an inbound lead automatically generates a quote, requests a deposit, locks the calendar, and queues the final invoice without the owner ever touching a desktop keyboard.

  **Next Actions for Engineering**:
  1. **Update Data Models**: Enhance the `quotes` table schema to include `linked_invoice_id`, `calendar_event_id`, and `deposit_payment_intent_id`.
  2. **Agent Upgrades**: Upgrade the `Sales Agent` to parse natural language service requests and output the enhanced `Quote` JSON structure. Upgrade the `Finance Agent` to trigger automatically on deposit clearing.
  3. **Interactive Customer UI**: Build the external, customer-facing interactive Quote page (mobile-first, 375px) using the existing Next.js/Tauri framework. It must support 1-tap Apple/Google Pay for the deposit.
  4. **Owner Approval Card**: Implement the "Quote Draft" approval card in the Unified Agent Feed.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []