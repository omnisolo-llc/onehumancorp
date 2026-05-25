issue_title: "[Architecture] Invisible AI Payroll & Contractor Management Engine"
issue_description: |
  # Issue Brief: Invisible AI Payroll & Contractor Management Engine

  ## Problem Statement
  Small business owners like Carlos (the handyman) and Fatima (the food cart operator) frequently hire temporary help, subcontractors, or part-time staff. Managing payroll, tax classifications (W-2 vs 1099), timesheets, tip distribution, and payouts is overwhelmingly complex and heavily regulated. Current solutions (Gusto, Deel, QuickBooks) feel like enterprise software and require extensive onboarding. SMB owners need a system that handles payments invisibly, allowing them to say "Pay John $150 for helping out today," while AI handles the compliance, ledger updates, and actual fund transfers in the background.

  ## Research Report
  - **Market Gap:** While Square offers Square Payroll, it still demands formal setup processes that deter micro-businesses. Shopify largely relies on app ecosystems for HR/payroll, which creates friction.
  - **User Needs:** Real-time payout capabilities, automatic tax form generation (like 1099s at year-end), and conversational or 1-tap payout interfaces.
  - **OHC Opportunity:** By leveraging the OHC unified ledger and AI agents, we can abstract away "payroll" into simple "team payments." The platform acts as the intermediary, securely storing contractor payout details and ensuring compliance automatically.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Owner as SMB Owner
      participant UI as OHC Mobile App
      participant AI as AI HR/Finance Agent
      participant Ledger as Unified Ledger
      participant Bank as Payout API (Stripe/Plaid)
      actor Staff as Contractor/Staff

      Owner->>UI: "Pay John $150 for today's shift"
      UI->>AI: Parse intent and extract payment details
      AI->>Ledger: Verify available funds & check tax classification
      Ledger-->>AI: Funds secured, categorized as 1099 contractor expense
      AI->>Bank: Initiate instant transfer to John's saved payout method
      Bank-->>Staff: Funds deposited
      AI-->>UI: "Done! John has been paid $150."
  ```

  ### Data Model & Invariants (Entity-Relationship)
  ```mermaid
  erDiagram
      TENANT ||--o{ TEAM_MEMBER : employs
      TEAM_MEMBER ||--o{ PAYOUT_METHOD : owns
      TEAM_MEMBER ||--o{ PAYOUT_RECORD : receives
      TENANT ||--o{ LEDGER_ENTRY : owns
      PAYOUT_RECORD ||--|| LEDGER_ENTRY : generates

      TENANT {
          uuid id PK
          string business_name
          string current_balance
      }
      TEAM_MEMBER {
          uuid id PK
          uuid tenant_id FK
          string full_name
          string tax_classification "W2 | 1099"
          decimal ytd_earnings
      }
      PAYOUT_METHOD {
          uuid id PK
          uuid team_member_id FK
          string status "verified | pending_setup"
          string encrypted_routing_details
      }
      PAYOUT_RECORD {
          uuid id PK
          uuid team_member_id FK
          decimal amount
          timestamp paid_at
      }
      LEDGER_ENTRY {
          uuid id PK
          uuid tenant_id FK
          string category "Contractor Expense"
          decimal impact
      }
  ```
  **Invariants:**
  - Multi-tenant isolation: No `TEAM_MEMBER` or `PAYOUT_RECORD` query may run without filtering by the authenticated `TENANT` ID.
  - Payout constraints: A `PAYOUT_RECORD` can only be generated if `TENANT.current_balance >= amount`.
  - Zero-trust security: SPIFFE/SPIRE authentication guarantees inter-service calls (AI Agent -> Ledger).

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Screen:** A quick action button labeled "Pay Team" alongside the AI conversational search bar.
  - **Payment Flow:**
    1. **Input:** The user speaks or types into the AI bar: "Pay Sarah $200 for the weekend pop-up."
    2. **Confirmation Card (Glassmorphic):** A clean, translucent card appears confirming the details: "Pay Sarah? Amount: $200. Category: Contractor Expense." with a single bold "Confirm & Pay" button.
    3. **Success State:** A subtle haptic feedback and a small checkmark animation. No dense receipts unless requested.
  - **Advanced Settings (Hidden):** W-9 forms, bank routing details, and year-to-date earnings for each staff member are hidden behind a "Team Details" menu, out of the daily critical path.

  ### Performance, Offline & Payload Targets
  - **Latency Constraints:** Conversational intent parsing and ledger verification must complete within **<300ms (P95)**. Payout initiation to third-party providers should happen asynchronously to prevent blocking the UI.
  - **Payload Size:** The client-side footprint of the "Pay Team" module should be **<50KB** (gzipped) for instantaneous loading on 3G networks.
  - **Offline Capability:** Intent commands ("Pay John $150") can be queued locally via CRDTs if the user is offline (e.g., working in a basement). The transaction syncs and executes immediately upon reconnecting, with an optimistic UI state displaying "Payment Queued."

  ### AI Agent Integration Points
  - **Finance Department Agent:** Intercepts payment commands, verifies ledger balances, and logs the transaction for end-of-year tax reporting.
  - **Legal/Compliance Agent:** Silently monitors transaction volume to a single individual. If payments to a contractor exceed $600 in a year, it automatically triggers a gentle SMS to the contractor to collect W-9 details without bothering the owner.

  ### Key Design Decisions
  - **Zero-Onboarding Payouts:** If a contractor hasn't added bank details, the system sends them a secure SMS link to claim their funds via OHC, functioning as a viral acquisition loop.
  - **Ledger-First Approach:** All payroll actions are fundamentally atomic ledger transfers, ensuring the business's real-time P&L is always accurate.
  - **Abstracted Terminology:** Avoid words like "Payroll," "W-4," or "ACH." Use "Pay Team," "Tax Profile," and "Direct Deposit."

  ## Implementation Prompt
  **For Implementer Agents:**
  Design and implement the underlying multi-tenant data structures and AI agent workflows to support conversational and 1-tap team payouts. The system must integrate with the existing OHC unified ledger.
  **Core User Journey (CUJ):**
  1. A business owner (authenticated) issues a natural language command to pay a known staff member a specific amount.
  2. The system interprets the command, checks the ledger, and processes the payment securely.
  3. If the staff member is unknown or missing payout details, the system gracefully handles collecting that information via secure SMS link.
  **Acceptance Criteria:**
  - Secure, isolated multi-tenant records for team members/contractors.
  - Real-time ledger updates reflecting the payout as an expense.
  - Integration with the AI Finance Agent to parse and execute payment intents.
  - 100% functional on a mobile-first 375px viewport.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
