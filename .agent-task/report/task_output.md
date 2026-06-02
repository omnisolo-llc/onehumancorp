issue_title: "[Architecture] Invisible Multi-Party Split Payments & Consignment Ledger"
issue_description: |
  # Problem Statement
  For small business owners like Priya (boutique owner) who sells consignment items for local artists, and Carlos (handyman) who frequently sub-contracts specialized tasks (like electrical work) on larger jobs, managing multi-party payments is an administrative nightmare.

  Currently, they must take the full payment from the customer, manually calculate the split percentage, keep track of who owes what on a notepad or complex spreadsheet, and remember to manually send Venmo/Zelle payouts at the end of the week. This causes tax confusion (inflated revenue for the primary owner), delayed payouts, and endless friction. They need a system where they can just tag a partner, and the money splits magically in the background.

  # Research Report
  ## Competitive Landscape
  *   **Shopify:** Has basic split payment capabilities, but it requires installing complex third-party apps (like CollabPay). These are desktop-first, involve complex multi-step setups, and the apps take an extra fee.
  *   **Wix & Squarespace:** No native, easy-to-use split payment capability. Users are forced to handle sub-contractor payouts completely offline.
  *   **Stripe Connect:** Powerful developer API, but requires writing code. Too technical for Carlos or Priya to set up on their own.

  ## Opportunity
  OneHumanCorp (OHC) can completely eliminate this friction by offering an invisible, zero-config split payment engine. When Priya adds a consignment product or Carlos drafts an invoice, they simply tag the sub-contractor. The AI Agents automatically handle the ledger splits, issue localized receipts, and route funds directly to the partner's account, removing the primary business owner entirely from the payout burden.

  # Design Doc
  ## Architecture Diagram
  The architecture orchestrates payment captures, split ratio application via the Finance AI Agent, ledger commits, and sub-merchant payouts using the Stripe Connect framework. It utilizes the PostgreSQL `SKIP LOCKED` job queue for asynchronous processing.

  ```mermaid
  erDiagram
      INVOICE ||--o{ PAYMENT_INTENT : "initiates"
      PAYMENT_INTENT ||--|{ TRANSFER_JOB : "triggers async"
      TRANSFER_JOB }|--|| FINANCE_AGENT : "processed by"
      FINANCE_AGENT ||--o{ LEDGER_ENTRY : "records splits"
      FINANCE_AGENT ||--o{ STRIPE_CONNECT : "initiates payout"

      INVOICE {
          uuid id
          float total_amount
          json split_config
      }
      TRANSFER_JOB {
          uuid id
          string status "pending | processing | completed"
      }
      LEDGER_ENTRY {
          uuid tenant_id
          uuid sub_merchant_id
          float amount
      }
  ```

  ## Mobile UX Flow (375px First)
  **Screen 1: Add/Edit Product (or Service Invoice)**
  *   Clean, macOS-style Translucent Glass dashboard card.
  *   Standard product/service fields (Image, Title, Price).
  *   A new, highly visible toggle: `[ Split this payment ]`.

  **Screen 2: Split Configurator (Bottom Sheet Modal)**
  *   Triggered by the toggle. Slides up smoothly.
  *   "Who gets a cut?" search bar (auto-suggests existing contacts, or allows adding a new phone number/email).
  *   A simple, playful visual slider or numeric input (e.g., "70% to Sarah (Artist)").
  *   Real-time preview text: *"If this sells for $100, Sarah gets $70, you get $30."*
  *   Primary Button: `[ Confirm Split ]`.

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to build the "Invisible Multi-Party Split Payments & Consignment Ledger" capability.

  **Core User Journey (CUJ):**
  Priya opens the OHC mobile app, creates a new product listing, taps "Split this payment", and allocates 70% to a contact ("Sarah"). When a customer buys the product, the system automatically splits the funds at the ledger level, notifying both Priya and Sarah of their respective cuts, without Priya doing any manual math or transfers.

  **Acceptance Criteria:**
  *   **Mobile-First UX:** The Split Configurator must be a smooth, native-feeling bottom-sheet interaction on a 375px screen.
  *   **Zero Jargon:** No "Stripe Connect", "Escrow", or "Settlement" terminology in the UI. Keep it conversational.
  *   **Multi-Tenant Safety:** The underlying data model must securely and immutably record the split between two separate identities, adhering to strict Zero-Trust boundaries.
  *   **Agent Orchestration:** The Finance AI must handle the ledger entries asynchronously via the `SKIP LOCKED` job queue. Do not block the primary checkout API request.

  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
