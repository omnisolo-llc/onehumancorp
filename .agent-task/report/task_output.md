issue_title: "Implement Autonomous Cashflow & Instant Treasury Engine"
issue_description: |
  # Title: Autonomous Cashflow & Instant Treasury Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) suffer from "Financial Fog" and constant cash flow anxiety. Maya receives payments via Instagram DMs, OHC Tap-to-Pay, and web orders, but she doesn't know in real-time how much of her bank balance is actual "profit" versus what needs to be set aside for taxes, ingredient restocks, or her own salary. She manually transfers money between bank accounts at the end of every week, a tedious and error-prone process. They need an invisible treasury that automatically routes incoming funds into appropriate "envelopes" (tax, operations, profit) instantly, providing real-time clarity and instant liquidity without manual bookkeeping.

  ## Research Report
  *   **Current Architecture Limits:** Current OHC integrations with Stripe or native POS process the transaction but dump the funds into a single, unified merchant account balance. There is no automated capability to allocate or reserve funds for specific liabilities.
  *   **Competitor Analysis:**
      *   *Shopify Balance:* Provides fast payouts and basic categorization, but it requires active manual management and setup.
      *   *Wix Payments:* Simple payment processing, but lacks proactive treasury features.
      *   *Novo/Relay (SMB Banking):* Offer envelope budgeting (reserves), but they don't natively integrate with the platform's order-level data to automatically predict how much to set aside based on the specific items sold.
  *   **Discovery:** OHC must provide an Instant Treasury Engine that intercepts all incoming settled payments, uses AI to predict liabilities (e.g., this cake order requires 30% for ingredients, 15% for tax), and auto-splits the funds into logical ledgers ("Buckets"). It should also offer instant access to these funds via OHC-issued virtual cards for immediate operational spend.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      PAYMENT-GATEWAY ||--o{ INGESTION-ROUTER : "Settled Funds Event"
      INGESTION-ROUTER ||--o{ TREASURY-AI-AGENT : "Request Split Rules"
      TREASURY-AI-AGENT }|--|| HISTORICAL-CASHFLOW-DB : "Analyze Needs"
      TREASURY-AI-AGENT ||--o{ CORE-LEDGER : "Dispatch Split Txs"
      CORE-LEDGER ||--o{ TAX-BUCKET : "Credit (Multi-tenant)"
      CORE-LEDGER ||--o{ OPS-BUCKET : "Credit (Multi-tenant)"
      CORE-LEDGER ||--o{ PROFIT-BUCKET : "Credit (Multi-tenant)"
      OPS-BUCKET ||--o{ VIRTUAL-CARD-ISSUING : "Fund Card Authorization"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Dashboard View (375px):**
      *   A prominent macOS-style translucent glass card at the top: **"Safe to Spend: $1,240.00"** (This is the Profit bucket).
      *   Below it, smaller unified UI modular cards:
          *   "Reserved for Taxes: $450.00"
          *   "Reserved for Supplies: $300.00"
      *   **Action:** User taps "Safe to Spend".
      *   **Detail Screen:** Shows a one-tap button: "Transfer to Personal Bank" or "Spend via Apple Pay (OHC Card)".
      *   No complex accounting ledgers are visible to the user unless they toggle "Advanced Settings".

  ### Key Design Decisions
  *   **Double-Entry Core, Envelope UI:** The backend must use a rigorous, multi-tenant double-entry accounting ledger to track all fractional cents securely. However, the frontend strictly abstracts this into simple "Buckets" or "Envelopes", passing the "grandmother test".
  *   **Event-Driven Ingestion:** The engine must asynchronously consume settlement events to ensure high throughput and not block the main checkout flow.
  *   **Zero-Trust Isolation:** Every ledger bucket is cryptographically isolated using SPIFFE/SPIRE multi-tenant identities to guarantee funds are never commingled across merchants.

  ### AI Agent Integration Points
  *   **Finance Agent (Treasury AI):** Automatically analyzes incoming orders. If Maya sells a custom wedding cake, the agent knows (based on past inventory costs) to route 30% of the revenue to the "Supplies" bucket automatically.
  *   **Operations Agent:** Monitors the "Supplies" bucket. If it falls below projected operational needs for the week, it sends a plain-language notification ("Maya, you might need an extra $100 for supplies this week based on upcoming orders. Want me to shift it from Profit?").

  ## Implementation Prompt
  Implement the Autonomous Cashflow & Instant Treasury Engine. Build the event-driven ingestion pipeline that listens for `PaymentSettled` events across all OHC channels. Design the integration with the Finance AI Agent to calculate dynamic fund splits based on the merchant's historical margins and upcoming liabilities. Implement the robust Core Ledger updates to route the funds into isolated, multi-tenant "Buckets" (Taxes, Operations, Profit) using double-entry principles. Ensure strict Zero-Trust boundaries using tenant IDs. Expose secure GraphQL APIs for the mobile client to read real-time bucket balances. Do not prescribe the specific underlying SQL schema, but guarantee ACID compliance and idempotency for all ledger transactions.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
