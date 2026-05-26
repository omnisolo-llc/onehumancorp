issue_title: "Invisible Real-Time Bookkeeping & Tax Engine"
issue_description: |
  # [Architecture] Invisible Real-Time Bookkeeping & Tax Engine

  ## 1. Title
  **Invisible Real-Time Bookkeeping & Tax Engine: Zero-Friction Accounting for the Autonomous SMB**

  ## 2. Problem Statement
  For OneHumanCorp (OHC)’s core personas—especially **Maya (baker, 28)** and **Carlos (handyman, 42)**—accounting is the most dreaded aspect of running a business. At the end of the year, they scramble to organize receipts, categorize expenses, and calculate estimated taxes, often using disconnected tools like QuickBooks or a shoebox of receipts. This causes massive "Financial Fog," which is one of the top 10 SMB pain points, leading to cash flow issues and unexpected tax liabilities.
  Current platforms (Shopify, Wix) offload this critical responsibility to third-party integrations, fundamentally breaking the "Zero Manual Work" promise. Small business owners do not want a complex P&L dashboard; they just want to know: "How much money can I safely spend, and how much is reserved for taxes?"

  ## 3. Research Report
  ### Competitive Landscape
  *   **Shopify:** Relies heavily on QuickBooks Online or Xero integrations. No native, real-time tax withholding or expense categorization agent.
  *   **Wix:** Basic financial reporting, but forces users into manual categorization and external accounting software.
  *   **Stripe Tax / QuickBooks:** Powerful but requires the user to understand accounting terminology (e.g., Chart of Accounts, Reconciliation).

  ### Market Data
  *   **35%** of small business owners cite "Financial Fog" and tax anxiety as a top source of stress.
  *   **Solopreneurs** spend an average of 15 hours a month on bookkeeping tasks.
  *   The IRS estimates millions of dollars in penalties are paid annually by SMBs due to miscalculated or missed quarterly tax payments.

  ### Opportunity
  We bypass the entire "Accounting Software" category by embedding an invisible, real-time bookkeeping engine directly into the core OHC ledger. By leveraging AI to categorize transactions at the moment they occur and automatically reserving estimated tax liabilities into virtual accounts, we eliminate the need for end-of-year scrambling.

  ## 4. Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant User as Carlos (Mobile 375px)
      participant Edge as Edge Gateway
      participant Ledger as Universal Multi-Tenant Ledger
      participant FinanceAgent as The Treasurer (Finance AI)
      participant ActionFeed as OHC Action Feed
      participant Banking as Virtual Tax Account (Banking API)

      User->>Edge: Completes a $500 job (Payment Received)
      Edge->>Ledger: Record Transaction (Multi-Tenant, Immutable)
      Ledger->>FinanceAgent: Trigger Analysis on Transaction
      FinanceAgent->>FinanceAgent: Identify category, calculate 25% tax liability ($125)
      FinanceAgent->>Banking: Auto-transfer $125 to "Tax Reserve" sub-account
      FinanceAgent->>ActionFeed: Update Daily Briefing: "$375 safe to spend. $125 saved for taxes."
      User->>ActionFeed: Reviews plain-language daily summary
  ```

  ### UI Wireframes (375px Mobile-First) & Mobile UX Flow
  **Screen 1: Financial Summary Card (Dashboard)**
  *   Clean, macOS-style Translucent Glass card on the main dashboard.
  *   No complex P&L charts by default.
  *   **Key Metric 1:** "Safe to Spend" (Available Cash - Tax Reserve).
  *   **Key Metric 2:** "Tax Reserve" (Automatically accumulated).
  *   **Action Button:** `[ View Plain-Language Report ]`

  **Screen 2: The Plain-Language Report**
  *   Conversational interface replacing the traditional ledger view.
  *   "You made $2,000 this week. We saved $500 for quarterly taxes. Your biggest expense was materials for the Smith job."

  **Screen 3: Expense Capture (Camera Integration)**
  *   User takes a photo of a receipt.
  *   Vision AI automatically reads the merchant, amount, and date, classifying it as a business expense and adjusting the tax liability downward automatically.

  ### AI Agent Integration Points
  *   **The Treasurer (Finance AI):** Constantly monitors the immutable ledger. When a transaction occurs (inbound or outbound), it classifies the transaction against standard tax categories without user input. It also calculates dynamic tax liabilities based on the user's jurisdiction and revenue run-rate.
  *   **The Visualizer (Vision AI):** Integrates with the Treasurer to process physical receipts captured via the mobile camera, turning unstructured images into structured ledger entries.

  ### Key Design Decisions and Why
  *   **Virtual Sub-Accounts over Dashboards:** Instead of just telling the user they owe tax, we physically (or virtually via banking APIs) move the money into a locked "Tax Reserve" account. This prevents the most common SMB failure: spending tax money.
  *   **Immutable Ledger Foundation:** All transactions are written to a strictly isolated, multi-tenant immutable ledger to guarantee auditability and security (Zero Trust).
  *   **Plain Language Default:** "Chart of Accounts," "Debits/Credits," and "Reconciliation" are banned terms in the default UI. They exist only behind the "Advanced Accountant Settings" switch for CPA access.

  ## 5. Implementation Prompt
  **To the Implementer:**
  Your task is to build the "Invisible Real-Time Bookkeeping & Tax Engine." The Core User Journey (CUJ) is as follows:
  When a transaction is recorded in the system (e.g., an invoice paid or a receipt scanned), the system must autonomously classify the transaction and update a real-time "Safe to Spend" and "Tax Reserve" balance, presenting this to the user in a plain-language mobile UI.

  **Acceptance Criteria:**
  *   **Data Model:** Define the entities for the Multi-Tenant Immutable Ledger and Virtual Accounts. Ensure strict Zero Trust isolation.
  *   **AI Integration:** The system must trigger the Finance AI agent upon new ledger entries to perform categorization and tax liability calculation automatically.
  *   **Mobile-First UX:** The data must be surfaced via a 375px-optimized UniFi-style card showing only plain-language metrics ("Safe to Spend", "Reserved for Taxes").
  *   **Grandmother Test:** A user must never see traditional accounting terms (P&L, General Ledger) unless they explicitly toggle an "Advanced" mode.

  *(Note: Describe WHAT the system should do and WHY, ensuring you map the data structures and agent flows, but do NOT prescribe specific SQL DDL, API endpoints, or lower-level library implementations.)*

  ## 6. Priority
  `P1` (High - Critical for the Retention and Revenue phases of the business journey)

  ## 7. Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []