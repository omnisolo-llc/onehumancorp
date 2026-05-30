issue_title: "[Scale] High-Performance Stripe Payment Routing & Ledger Optimization"
issue_description: |
  # High-Performance Stripe Payment Routing & Ledger Optimization

  ## Problem Statement
  As OneHumanCorp grows, our payment integration via Stripe experiences structural limitations. Currently, small payouts result in large cumulative transaction fees for our users, reducing their profit margin. Non-technical users like Maya (the baker) and Carlos (the handyman) shouldn't have to figure out how to minimize payout fees—the platform should handle it invisibly. Additionally, we need a strong, reliable architecture for handling payout batching and routing across diverse currency contexts to save processing fees.

  ## Research Report
  - **Stripe Fees**: Standard payout fees include a fixed amount (e.g., $0.25). A high frequency of small payments leads to unnecessary accumulated costs.
  - **Competitor Analysis**: Platforms like Shopify bundle payouts by default based on local payout schedules. Wix and Squarespace provide basic options but lack dynamic optimization for minimizing transaction fees based on size and currency.
  - **Current System Gap**: The `PaymentRouter` in `src/server/integrations/stripe/routing.rs` performs simple ACH vs. Credit Card checks but does not integrate deeply with a distributed, multi-tenant payout ledger that guarantees safe batching across node restarts or scaling events (currently uses a basic Redis counter with fallback to memory).

  ## Design Doc
  ### Architecture
  1.  **Durable Tenant Ledger**: Replace the simple Redis counter in `PayoutBatcher` with a robust multi-tenant PostgreSQL ledger implementation. This guarantees atomicity and zero data loss. Redis can be used as an aggressive cache, but PostgreSQL is the source of truth for pending payouts.
  2.  **Dynamic Thresholding**: Adjust batch thresholds based on tenant risk profile and historical volume, powered by an AI Business Advisory agent insight.

  ```mermaid
  graph TD;
      A[Tenant Transaction] --> B[Payment Router];
      B --> C{Amount > Threshold?};
      C -- Yes --> D[Immediate ACH Routing];
      C -- No --> E[Durable Payout Ledger PostgreSQL];
      E --> F[Background Job: Scheduled Payout Batch];
      F --> G[Stripe Payout API];
  ```

  ### AI Integration
  - **Finance & Payments Agent**: This agent will monitor the ledger and adjust payout schedules. It sends plain-language notifications ("We saved you $12 in fees this week by grouping your payments!").

  ### User Journey (Mobile-First)
  - User receives payment -> Dashboard shows "Pending Payout" and "Estimated Fee Saved".
  - Clean UI with translucent glass materials indicating when the next bundled payout occurs.

  ## Implementation Prompt
  Implement a robust `Ledger` service for multi-tenant payout batching backed by PostgreSQL, deprecating the pure Redis counter approach. Ensure the `PaymentRouter` utilizes this durable ledger. Update the Stripe client to correctly dispatch jobs to the `Ledger`. Include 100% unit test coverage for the ledger logic and integrate it with the background worker queues.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
