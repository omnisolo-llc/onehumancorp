issue_title: "[architecture] Automated Omnichannel Loyalty & Rewards Engine"
issue_description: |
  # Architecture Report: Automated Omnichannel Loyalty & Rewards Engine

  ## Problem Statement
  Small business owners (like Priya the Boutique Owner or Maya the Baker) struggle to retain customers and incentivize repeat purchases. Current platform loyalty programs (e.g., Smile.io on Shopify) are complex to configure, require a separate app subscription, and are often disconnected from in-person sales or social commerce (Instagram DMs). They require manual intervention to adjust points or issue rewards, creating friction for both the owner and the customer. The owner needs a native, zero-touch loyalty system that automatically tracks points across all channels and agentically prompts customers with rewards to drive re-engagement.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Requires expensive third-party apps (Smile.io, Yotpo). These apps have complex rules engines that overwhelm non-technical users and often do not sync seamlessly with Shopify POS without higher-tier plans.
  - **Wix/Squarespace:** Offer basic native loyalty, but they are passive—relying on the customer to remember they have points and log in to use them.
  - **OHC Opportunity:** A native, omnichannel loyalty ledger where points are automatically accrued for every purchase (online, in-store via POS, or via Agent conversational checkout). The crucial differentiator is the **Agentic Re-engagement**: the Marketing Agent (The Promoter) proactively monitors the loyalty ledger and automatically drafts/sends personalized reward offers (e.g., "You have enough points for a free pastry!") via SMS, Email, or DM to dormant customers.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Checkout] -->|Payment Event| E(Loyalty Event Mesh)
      B[POS / Terminal] -->|Payment Event| E
      C[Conversational Checkout] -->|Payment Event| E
      E --> F{Omnichannel Identity Resolution}
      F --> G[Unified Loyalty Ledger DB]
      G --> H[The Promoter Agent]
      H -->|Query Dormant Balances| G
      H -->|Draft Reward Offer| I[Action Required Queue / Auto-Dispatch]
      I --> J[Mobile App Feed 375px]
      J -->|Approve| K[Omnichannel Dispatcher]
      K -->|SMS/Email/DM| L[Customer]
  ```

  ### Mobile UX Flow (375px First)
  1.  **Configuration (Owner):** A simple toggle in settings: "Enable Automatic Loyalty Program (1 point per $1)". No complex rules.
  2.  **Dashboard Feed (Owner):** The unified feed surfaces cards like: "3 customers just earned a free coffee reward. Agent drafted notifications. [Approve All]".
  3.  **Customer Experience (Checkout):** During checkout, if the customer's recognized identity has enough points, an automatic "Apply Reward" toggle appears. No codes to copy/paste.
  4.  **Customer Experience (In-Store POS):** The POS UI displays the customer's point balance upon identifying them (via email/phone or tap-to-pay profile), allowing the cashier to apply it with one tap.

  ### AI Agent Integration Points
  -   **The Promoter (Marketing):** Runs a scheduled job (e.g., weekly) to scan the `loyalty_ledger`. It identifies customers close to a reward tier or those who haven't purchased recently but have a high balance. It drafts personalized outreach campaigns offering to redeem their points.
  -   **The Ambassador (Customer Success):** Can answer natural language queries in DMs: "How many points do I have?" -> Queries ledger -> "You have 450 points! Enough for $5 off your next cake."

  ### Key Design Decisions
  -   **Immutable Ledger:** Points are managed via a strict append-only ledger (`loyalty_ledger` table with `credit` and `debit` events) for perfect auditability, completely isolated by `tenant_id`.
  -   **Identity First:** Loyalty is tied to the `Customer` entity, not a separate account, relying on the Omnichannel Identity Resolution engine to merge online and offline profiles.
  -   **Proactive, not Passive:** The system pushes the reward to the customer via Agents, rather than waiting for them to discover it.

  ## Implementation Prompt
  **User-Facing Outcome:** Priya turns on the Loyalty feature with one tap. Two weeks later, her OHC mobile feed shows: "Sarah hasn't been in the store for a month but has $10 in loyalty credit. Send her an SMS reminder? [Approve]". Sarah receives the text, taps the link, and buys a $50 dress using her credit.

  **CUJ & Acceptance Criteria:**
  1.  Create the `loyalty_ledger` database schema with Row Level Security enforcing tenant isolation.
  2.  Implement an event listener on successful payment intents (Stripe webhooks/internal events) that automatically credits the customer's loyalty balance based on a simple tenant setting (e.g., 1 point per $1).
  3.  Implement a scheduled task (simulated via API for testing) for The Promoter Agent that queries the ledger for users with unspent points > X and drafts a notification in the `ActionRequiredQueue`.
  4.  Provide Playwright E2E tests: Simulate a completed order, assert the ledger is updated. Log in as the owner on mobile view, view the drafted reward notification card, approve it, and assert the system dispatches the message.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
