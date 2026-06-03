issue_title: "Build Universal Omnichannel Gift Card & Store Credit Engine"
issue_description: |
  Small business owners like Priya (boutique) and Maya (baker) rely heavily on gift cards for holiday sales and customer retention. Currently, OHC lacks a unified system for selling, issuing, and redeeming digital gift cards or managing store credit. When a customer wants a refund for an in-store purchase or wants to buy a gift card for a friend online, the business owner must use external, disconnected tools or write it down on paper. This breaks the multi-channel experience and requires manual reconciliation, leading to lost revenue and bad customer experiences.

  **Research Report:**
  **Market Analysis & Competitor Benchmarks:**
  - **Shopify:** Offers robust gift card capabilities, but requires higher-tier plans for advanced features. Their API allows omnichannel usage, but the UI is still backend-heavy.
  - **Square:** Dominates in-person gift cards. Seamless integration with their POS, but less flexible for cross-platform online/offline blending without complex API work.
  - **Wix/Squarespace:** Gift card features exist but often feel bolted on, with separate flows for physical vs. digital.

  **OHC Opportunity:**
  By building a native, multi-tenant Ledger for Gift Cards and Store Credit, OHC can instantly enable business owners to offer Apple Wallet/Google Wallet compatible digital gift cards. AI agents can autonomously handle the entire lifecycle: sending the gift card via SMS/email, tracking usage, and issuing automatic store credit for returns via the omnichannel inbox, requiring zero manual configuration by the business owner.

  **Implementation Prompt:**
  Context: Implement the backend ledger and mobile-first UI for the Universal Gift Card & Store Credit Engine.
  Outcome: A business owner can sell, issue, and redeem gift cards across both online checkout and mobile Tap-to-Pay POS. Customers can receive and store these gift cards in their digital wallets. Returns can be processed instantly to store credit.
  Acceptance Criteria:
  1. Append-only ledger data model is implemented with strict multi-tenant (tenant_id) isolation.
  2. Endpoints exist to create, redeem, and check the balance of a gift card.
  3. Concurrent redemption attempts on the same gift card must safely resolve without negative balances (prevent double-spend).
  4. The merchant UI uses the defined macOS-style translucent glass components on a 375px viewport.
  5. All developer terms are hidden; the UI simply says "Gift Cards & Store Credit" - passes the "grandmother test".
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
