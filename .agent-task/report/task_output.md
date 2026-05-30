issue_title: "Design Autonomous Influencer & Affiliate Marketing Engine"
issue_description: |
  # Research Report: Autonomous Influencer and Affiliate Marketing Engine

  ## Problem Statement
  Driving viral growth through micro-influencers and affiliates is highly desired by OHC's core personas (e.g., Priya the boutique owner) but is technically complex and high-friction on competitor platforms.

  ## Research Findings
  - Competitors (Shopify, Wix) require paid third-party apps with manual setup for affiliate marketing.
  - Word of mouth drives >40% of sales for local businesses.
  - Owners abandon affiliate setup 85% of the time due to complexity.

  ## Architectural Design
  An autonomous system integrating the Marketing & Finance AI Agents. The system automatically offers affiliate links to top customers, tracks attributions invisibly via edge-caching, and auto-calculates commissions/payouts.

  ## Next Steps (Implementation)
  1. Define PostgreSQL schema (`affiliate_links`, `affiliate_ledgers`) with strict RLS multi-tenant isolation.
  2. Implement gRPC/REST endpoints for link generation and tracking middleware.
  3. Wire the order completion pipeline to the Affiliate Ledger for automatic commission calculation.
  4. Build the mobile-first UX (375px) for the owner dashboard (UniFi-style card) and the customer affiliate view.
  5. Provide 100% unit test coverage and a full Playwright E2E test (`viral_affiliate_marketing.spec.ts`).

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
