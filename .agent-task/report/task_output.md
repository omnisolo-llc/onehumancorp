issue_title: "Feature: Autonomous Agentic Loyalty & Retention Engine"
issue_description: |
  # Mission Queue Protocol: Autonomous Agentic Loyalty & Retention Engine

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) and Maya (Home Baker) struggle to retain customers and incentivize repeat purchases. Existing loyalty tools (e.g., Smile.io, Yotpo) are standalone apps that require manual configuration of points, tiers, and rewards. These tools do not seamlessly integrate with in-store POS and online checkouts without complex rules, and they require the owner to actively monitor and email customers. OHC needs an invisible, agent-driven loyalty engine that automatically tracks customer lifetime value, issues rewards, and sends personalized win-back campaigns without owner intervention.

  ## Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for loyalty, leading to "App Tax" fatigue (merchants paying $30-$100/mo extra). Square has built-in loyalty but it is rigid and POS-centric.
  - **Competitor Gaps**: Legacy tools require manual points-to-dollars mapping. They do not use LLMs to analyze customer purchasing behavior or autonomously suggest dynamic perks (e.g., "Free local delivery on your next cake").
  - **OHC Opportunity**: Leverage the existing Customer Success Agent ("The Ambassador") to autonomously manage a Unified Loyalty Ledger. The agent will proactively message high-value customers with personalized rewards based on their purchase history across all channels (online and Terminal POS).

  ## Design Doc
  ### Data Model (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ CUSTOMER : has
      CUSTOMER ||--o{ LOYALTY_LEDGER : earns
      LOYALTY_LEDGER {
          string id
          string customer_id
          int points_balance
          int lifetime_points
          string tier
      }
      CUSTOMER ||--o{ REWARD_CLAIM : redeems
      REWARD_CLAIM {
          string id
          string discount_code
          string status
      }
  ```

  ### Architecture & Component Diagram
  - **Loyalty Ledger (PostgreSQL)**: A robust event-sourced ledger tracking points earned and redeemed to prevent race conditions.
  - **The Ambassador Agent (LLM Layer)**: Monitors the `orders` and `pos_terminal_sessions` streams. When a customer crosses a spending threshold, the Agent autonomously drafts an SMS/Email offering a perk and creates a transient discount code.
  - **Unified Inbox UI**: Displays a timeline of loyalty interactions and allows the owner to approve/reject agent-proposed win-back campaigns.

  ### Mobile UX Flow (375px First)
  1. **Owner View (Agent Feed)**: Owner opens OHC app and sees a card: "Agent: 5 customers haven't ordered in 60 days. I've drafted a 15% win-back offer. [Approve & Send]".
  2. **Customer View (Storefront)**: Customer logs in via magic link and sees a glassmorphism "Wallet" card showing points balance and a one-tap button to "Redeem 100pts for Free Shipping".
  3. **POS Checkout (In-Store)**: When Priya taps a customer profile during a Stripe Terminal checkout, a translucent badge appears if the customer has a reward available, allowing one-tap application.

  ### AI Agent Integration Points
  - **Event Trigger**: `OrderCompleted` event published to Redis Pub/Sub.
  - **Processing**: The Ambassador Agent evaluates the purchase against the tenant's generated loyalty rules.
  - **Action**: Updates `points_balance` and potentially enqueues a notification task in `ohc_job_queue`.

  ## Implementation Prompt
  **Implementer Agent Instructions**:
  1. Create the database migrations for `loyalty_ledgers` and `reward_claims` ensuring strict Row Level Security (RLS) on `tenant_id`.
  2. Implement a `LoyaltyService` in Rust (Backend) to handle point accrual and redemption safely with database transactions.
  3. Extend `The Ambassador` agent in `src/server/orchestration/departments/customer_success.rs` to process `POS_SALE_COMPLETED` and online order events, updating the loyalty ledger.
  4. Create a mobile-responsive (375px) React/Next.js widget for the storefront allowing customers to view and redeem points.
  5. Add E2E Playwright tests covering the Critical User Journey: A customer completes an order, earns points, and redeems them on a subsequent order.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
