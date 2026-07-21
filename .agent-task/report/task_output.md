issue_title: "[Architecture] AI-Driven Omnichannel Gift Card & Store Credit System"
issue_description: |
  ## Title: AI-Driven Omnichannel Gift Card & Store Credit System

  ## Problem Statement
  For retail and service-based small businesses (like Priya the Boutique Operator and Maya the Home Baker), gift cards and store credit are essential for driving customer acquisition, retention, and handling returns gracefully. However, traditional systems often silo online gift cards (e-commerce) from in-store physical gift cards (POS). This forces the business owner to use third-party apps or manual reconciliation. Furthermore, non-technical owners lack the time and expertise to actively promote gift cards or intelligently issue store credit during customer service interactions. They need a unified system where the AI actively manages balances, issues credit for returns autonomously, and campaigns for gift card sales during peak seasons.

  ## Research Report
  - **Competitor Systems Audit**:
    - *Shopify*: Offers omnichannel gift cards, but their setup is complex and managing physical vs. digital requires careful synchronization and often upgraded POS tiers. AI does not proactively manage this.
    - *Wix*: Basic digital gift card support, but disconnected from in-person tap-to-pay experiences.
    - *Square*: Strong in-person gift card support (physical cards), but less seamless in integrated AI workflows for online customer recovery.
  - **Identify Gaps**: OHC currently handles online orders and Stripe Terminal POS payments but lacks a native, unified ledger for stored value (Gift Cards and Store Credit). Priya needs to sell a gift card online, and have the customer redeem it in-store via Tap-to-Pay, with the balance updating instantly. Maya needs the AI Customer Assistant to automatically offer a $10 store credit to a disappointed customer, without her needing to open a dashboard.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      Purchase[Online/In-Store Gift Card Purchase] -->|Creates/Updates| LedgerDB[(PostgreSQL: Stored Value Ledger)];
      CSAgent[AI Customer Success Agent] -->|Issues Store Credit| LedgerDB;
      LedgerDB -->|Strict Row-Level Security| TenantIsolation[Tenant Isolation Layer];
      Checkout[Checkout Flow / POS] -->|Queries Balance| LedgerDB;
      Checkout -->|Applies Balance| PaymentIntent[Stripe Payment Intent];
      PaymentIntent -->|Captures Remainder| Stripe[Stripe API];
      OperationsAgent[AI Operations Agent] -->|Monitors Liability| Dashboard[Owner Mobile Dashboard - 375px];
  ```

  ### Mobile UX Flow (375px)
  1. **Selling a Gift Card (In-Store)**: Priya taps "Sell Gift Card" in the OHC mobile POS, enters $50, and the customer taps their card. The system generates a digital code (or pairs with a physical QR/NFC card) and emails the recipient.
  2. **Automated Store Credit**: A customer DMs Maya complaining about a delayed cake delivery. The AI Customer Assistant drafts a reply: "I'm so sorry! I've issued a $15 store credit to your account for next time," and a button prompts Maya: "Approve & Issue Credit". Maya taps once; the ledger is updated instantly.
  3. **Redemption**: During checkout (online or mobile POS), the system automatically surfaces available store credit or prompts for a gift card code, dynamically updating the final Stripe charge amount.
  4. **Liability Dashboard**: The OHC app provides a plain-language summary of outstanding gift card liabilities so the owner knows exactly how much unused credit is circulating.

  ### AI Agent Integration Points
  - **AI Customer Success Agent**: Has the authority (subject to owner approval via mobile push) to issue store credit from the ledger to resolve disputes or reward loyal customers.
  - **AI Sales & Marketing Agent**: Detects upcoming holidays (e.g., Mother's Day) and drafts a promotional campaign to sell gift cards, suggesting an email/social blast.
  - **AI Finance Agent**: Tracks outstanding stored-value liabilities and incorporates them into the daily/weekly revenue summaries, ensuring the owner understands their financial position.

  ### Key Design Decisions
  - **Unified Ledger**: Gift Cards and Store Credit use the same underlying `stored_value_ledger` table in PostgreSQL, differentiated by `type`. This ensures atomic transactions and prevents double-spending across online and in-store channels.
  - **Stripe Integration**: The system deducts the stored value first, then creates a Stripe PaymentIntent for the remaining balance. This minimizes transaction fees and cleanly separates OHC-managed credit from external bank capture.
  - **Zero Trust**: Strict row-level security (`tenant_id`) and transactional locks (`SELECT ... FOR UPDATE`) are required to prevent race conditions during redemption, especially if a customer tries to use the same code online and in-store simultaneously.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the backend architecture for the Omnichannel Gift Card & Store Credit System.
  1. Design the PostgreSQL schema for the `stored_value_ledger` and `gift_cards` tables, incorporating `tenant_id` for RLS and strong transactional guarantees.
  2. Create the gRPC/REST APIs to issue, redeem, and check balances for stored value.
  3. Integrate the checkout flow (both online and Stripe Terminal POS) to apply stored value before calculating the final Stripe PaymentIntent amount.
  4. Extend the AI Customer Success Agent context to allow it to draft store credit issuances.
  5. Ensure 100% unit test coverage for ledger operations, explicitly testing concurrency (double-spend attempts).
  Do NOT build the mobile UI in this phase. Focus on the core ledger logic, payment routing, and agent capabilities.

  ## Priority
  P2

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
