issue_title: "Implement the Autonomous Cashflow & Micro-Capital Engine"
issue_description: |
  # [Architecture] Autonomous Cashflow & Micro-Capital Engine

  ## Problem Statement
  Small business owners—like Carlos the handyman and Maya the baker—often suffer from "Financial Fog". They have money coming in and out across different channels, making it difficult to understand real-time profitability, upcoming tax liabilities, and cash flow crunches. Furthermore, securing micro-capital (e.g., a $500 advance to buy inventory for a large order) through traditional banking or legacy platforms takes too long, requires dense paperwork, and adds immense friction. They need an invisible, plain-language financial partner that proactive monitors cash flow and offers instant micro-capital when a verified order requires it.

  ## Research Report
  **Findings:**
  - Based on our top 10 SMB pain points audit, "Financial Fog" affects at least 35% of users. It represents the inability to see real profit vs. revenue without exporting to spreadsheets.
  - **Competitor Analysis:**
    - **Shopify:** Offers Shopify Capital, but it is reactive, tied to complex historical performance algorithms, and targets larger financing needs rather than instant, order-based micro-advances. The dashboard remains complex.
    - **Wix:** Basic reporting capabilities; financing options are third-party driven and disjointed from daily operations.
    - **Square:** Offers Square Loans, which is close, but still feels like a separate banking product rather than an integrated, conversational cashflow agent.
  - **OHC Opportunity:** Leverage our AI Departments (Finance & Operations) to proactively identify when a user needs capital based on verified incoming orders (e.g., Fatima receives a massive catering pre-order but needs cash for ingredients). Offer plain-language insights ("You have $1,200 coming in next week, but owe $400 for supplies") and 1-tap micro-loans tied directly to specific transactions.

  ## Design Doc

  ### Architecture Diagram (Data Model & Relationships)
  ```mermaid
  erDiagram
      MERCHANT ||--o{ LEDGER_ENTRY : has
      MERCHANT ||--o{ ORDER : receives
      ORDER ||--o{ MICRO_LOAN : triggers
      LEDGER_ENTRY ||--o| MICRO_LOAN : funds

      MERCHANT {
          string id
          string name
          float current_balance
          float pending_balance
      }

      ORDER {
          string id
          float total_amount
          string status
          date expected_payout
      }

      LEDGER_ENTRY {
          string id
          string type
          float amount
          date timestamp
      }

      MICRO_LOAN {
          string id
          float advance_amount
          float fee
          string status
          date repayment_due
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **The Insight Notification:** Maya wakes up to a lock-screen notification from OHC Finance Agent: *"You have a new $800 custom cake order! Do you need a $200 advance for supplies?"*
  2. **The Dashboard Card:** On the home dashboard, a translucent glass card under "Cashflow" reads:
     - **Incoming this week:** $1,200
     - **Upcoming expenses:** $300
     - **Safe to spend:** $900
  3. **1-Tap Advance:** Tapping the notification opens a modal with a simple slider. "Advance $200 now for a $5 flat fee." The button reads **"Get $200 Instantly"**.
  4. **Zero-Jargon Confirmation:** A success screen confirms the money is available in the OHC Wallet/Tap-to-Pay pool immediately.

  ### AI Agent Integration Points
  - **The Finance Department (Agent):** Monitors the `LEDGER_ENTRY` and `ORDER` tables continuously in the background. Analyzes predictive cash flow based on historical seasonality and current pending orders.
  - **The Communications Department:** Drafts the plain-language SMS/Push notifications so the user never sees terms like "APR," "Underwriting," or "Amortization."

  ### Key Design Decisions
  - **Event-Driven Analysis:** Cashflow calculations and loan offers are triggered asynchronously via the background job queue whenever a new high-value order arrives, ensuring no impact on checkout latency.
  - **Multi-Tenant Isolation:** Ledger entries and loan states are strictly partitioned by Merchant ID using our Zero-Trust (SPIFFE/SPIRE) architecture.
  - **Mobile-Only Optimized UI:** All complex charts are replaced with plain-text summaries (e.g., "Safe to spend") ensuring it passes the "grandmother test." Developer terms remain strictly hidden.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Build the backend data models and the background orchestration logic for the Autonomous Cashflow & Micro-Capital Engine.
  1. Create the entities necessary to represent a Merchant's unified ledger, pending orders, and micro-loan advances. Ensure strict multi-tenant isolation.
  2. Implement the background worker (using our event-mesh) that listens for `order.created` events. When an order exceeds a merchant's typical average order value by 50%, evaluate eligibility for a micro-advance.
  3. Design the API endpoints to serve the "Safe to Spend" metric and the 1-Tap loan acceptance flow to the mobile frontend.
  *Acceptance Criteria:* The engine must process order events asynchronously, store ledger updates atomically, and expose a sub-50ms endpoint for the mobile app to fetch the daily plain-language cashflow briefing. Do not prescribe specific SQL schema details—design the system to integrate seamlessly with the existing OHC data layer and agent departments.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
