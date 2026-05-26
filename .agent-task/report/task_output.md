issue_title: "Architect Autonomous Working Capital & Instant Cash Advance Engine"
issue_description: |
  # Research Report: Autonomous Working Capital & Instant Cash Advance Engine

  ## Problem Statement
  Small business owners—like Carlos the handyman and Maya the baker—often struggle with cash flow gaps. Traditional business loans require extensive paperwork, high credit scores, and weeks of waiting. When a large tool breaks or a sudden massive custom order comes in requiring expensive raw materials, they are forced to use personal credit cards with high interest or turn down the job entirely. Competitors like Square (Square Capital) and Shopify (Shopify Capital) offer working capital, but it still requires checking dashboards, filling out acceptances, and managing manual repayment schedules.

  For OneHumanCorp (OHC), the solution must be entirely invisible, integrated into the core platform, and instantly available without applications. We need a system that observes cash flow anomalies and autonomous opportunities, then proactively offers instant capital that repays itself via future sales fractions, ensuring the business never stalls due to temporary liquidity constraints.

  ## Market Analysis
  - **Shopify Capital / Stripe Capital:** Evaluates historical sales data to extend predefined offers. Requires manual acceptance and dashboard tracking. Deducts a fixed percentage from daily sales until repaid.
  - **Square Capital:** Similar to Stripe, heavily relies on processing volume. Excellent for steady businesses but slower to adapt to sudden anomalous spikes (e.g., a viral TikTok post causing a 50x order surge).

  ## OHC Differentiation (The Gap)
  OHC acts as the core operating system, orchestrating every part of the business journey. We have deeper insights than a simple payment gateway. We can predict cash flow needs based on real-time inbox sentiment, calendar bookings, pending quotes, and inventory states.

  **Key Insight:** If OHC sees a signed $5,000 digital quote for Carlos but his business account balance is only $500, the AI Finance Agent knows he likely needs $1,500 for materials *today*. OHC can proactively offer an instant advance of $1,500, repaid dynamically from that specific $5,000 final payment. This zero-touch, context-aware capital allocation is a massive competitive moat.

  ## Design Doc

  ### 1. High-Level Architectural Design
  The architecture requires a multi-tenant `WorkingCapitalEngine` that listens to events from the `Ledger`, `Omnichannel AI Inbox`, and `Universal Capacity Mesh`. It feeds into an AI risk model that determines eligibility and instantly disburses funds via a `CapitalSession`.

  **Core Decisions:**
  - **Proactive Intelligence:** Instead of requiring a user to apply, the system actively scans upcoming approved quotes/invoices and low inventory states to suggest context-aware advances.
  - **Zero-Touch Repayment:** Repayment is automatically configured as a micro-fraction of incoming payments. No manual scheduling or bank transfers required.
  - **Invisible Ledger Abstraction:** Working capital transactions are entirely separated from operational revenue inside the Ledger, preserving clean accounting for the end user.

  ### 2. Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ CAPITAL_SESSION : has
      TENANT ||--o{ PREDICTIVE_OFFER : receives
      CAPITAL_SESSION ||--|{ REPAYMENT_TRANSACTION : tracks
      CAPITAL_SESSION ||--o{ LEDGER_ENTRY : triggers

      TENANT {
          string id
          string name
          float current_balance
      }

      PREDICTIVE_OFFER {
          string id
          string tenant_id
          float amount
          float fee_percentage
          string context_trigger
          boolean accepted
      }

      CAPITAL_SESSION {
          string id
          string tenant_id
          float principal_amount
          float total_repayment_amount
          float current_repaid
          float repayment_rate
          string status
      }

      REPAYMENT_TRANSACTION {
          string id
          string capital_session_id
          string source_payment_id
          float amount
          datetime timestamp
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Business Owner (Mobile)
      participant Sales Dept (AI Agent)
      participant Working Capital Engine
      participant Ledger
      participant Payout Engine

      Business Owner (Mobile)->>Sales Dept (AI Agent): Approves $5000 quote for customer
      Sales Dept (AI Agent)->>Ledger: Registers pending invoice
      Sales Dept (AI Agent)->>Working Capital Engine: Emits QuoteApprovedEvent(amount=$5000)
      Working Capital Engine->>Ledger: Check current liquidity
      Working Capital Engine->>Working Capital Engine: Run AI Risk & Opportunity Model
      Working Capital Engine->>Business Owner (Mobile): Push Notification: "Advance $1500 for materials instantly?"
      Business Owner (Mobile)->>Working Capital Engine: 1-Tap Accept
      Working Capital Engine->>Ledger: Debit Capital Reserve, Credit Tenant Balance ($1500)
      Working Capital Engine->>Payout Engine: Trigger Instant Transfer
      Payout Engine-->>Business Owner (Mobile): Push Notification: "$1500 available via Apple Pay."
  ```

  ### 3. Mobile UX Flow (375px)
  1. **The Contextual Nudge:** Carlos approves a $5,000 quote. A clean, macOS-style Translucent Glass card slides up from the bottom of his screen:
     * **Title:** "Need materials for the Smith job?"
     * **Subtitle:** "Get a $1,500 advance instantly. Repay automatically from the final invoice."
     * **Fee Info (Small Print):** "$50 flat fee. No interest."
  2. **One-Tap Action:** Carlos taps the prominent "[Accept $1,500]" button.
  3. **Instant Availability:** A brief success animation plays. A notification pops up: "$1,500 has been added to your OHC Wallet. Available for tap-to-pay immediately."
  4. **Repayment View:** In the Finances tab, a simple progress bar shows: "Working Capital Advance: $0 / $1,550 repaid. (10% of future sales will automatically apply)."

  ### 4. Technical Integrity & Mobile-First Review
  - **Performance & Offline Targets:** Event ingestion and scoring for the Working Capital Engine must complete within 250ms of the Quote Approved event to enable instant UI updates. The mobile client payload for the nudge must be under 15kb. Acceptance of the offer must function offline by queuing the acceptance payload locally and syncing to the backend once reconnected.
  - **Zero Trust & Security:** Multi-tenant boundaries are strictly enforced. Working Capital transactions leverage SPIFFE/SPIRE for authenticated service-to-service communication between the Working Capital Engine, Ledger, and Payout Engine, ensuring tenant state cannot be spoofed.

  ### 5. AI Agent Integration Points
  - **Finance Dept (Risk Analyst):** Continuously monitors the distributed ledger and upcoming calendar bookings to compute real-time risk profiles for each tenant.
  - **Operations Dept:** If a physical product goes out of stock but pre-orders continue, it flags the Working Capital Engine to offer an advance to bulk-purchase inventory.
  - **Legal/Compliance Dept:** Ensures that capital advance offers comply with regional lending regulations dynamically based on the tenant's registered location.

  ## Implementation Prompt
  **Task:** Implement the Autonomous Working Capital & Instant Cash Advance Engine.

  **Context:** We need a proactive, zero-friction working capital system for small businesses. Instead of waiting for users to apply for loans, the system must detect cash flow bottlenecks (e.g., an accepted high-value quote but low current balance) and generate contextual, 1-tap advance offers.

  **Requirements:**
  1. Create the data models (or equivalent structures) for `PredictiveOffer`, `CapitalSession`, and `RepaymentTransaction`. Ensure strict multi-tenant isolation via SPIFFE/SPIRE.
  2. Implement an event listener in the `WorkingCapitalEngine` that reacts to high-value events (like quote approvals or sudden inventory depletion).
  3. Build the risk assessment logic (stubbed AI risk scoring is fine for V1) that determines eligibility and generates a `PredictiveOffer`.
  4. Implement the 1-tap acceptance workflow (ensuring offline capability): Upon acceptance, the system must interface with the `Ledger` to instantly disburse funds to the tenant and establish the `CapitalSession`.
  5. Implement the auto-repayment hook: Every incoming sale must check for active `CapitalSessions` and siphon the configured percentage toward repayment before the remainder hits the tenant's main balance.

  **Note:** Do not prescribe specific database schemas or lower-level API endpoints. Focus on the core domain logic, multi-tenant safety, and event-driven architecture.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
