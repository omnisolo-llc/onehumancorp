issue_title: "Autonomous Working Capital & Micro-Lending Engine"
issue_description: |
  # Autonomous Working Capital & Micro-Lending Engine

  ## Problem Statement
  Small business owners frequently hit growth ceilings because they lack access to working capital. Traditional bank loans require extensive paperwork, credit checks, and take weeks to approve. Existing SMB platforms like Shopify Capital or Square Loans offer cash advances, but they are often buried in dashboards and require manual application steps. For non-technical users, understanding interest rates, compounding, and repayment terms is a source of severe "Financial Fog." They need an invisible, proactive system that automatically offers them the exact amount of capital they need, right when they need it, with simple, plain-language repayment terms that adjust dynamically to their daily sales.

  ## Research Report
  - **Market Gap:** Shopify Capital and Square Loans provide Merchant Cash Advances (MCAs), but they are static and not integrated with predictive inventory or booking signals. Stripe Capital offers embedded lending APIs but requires platform implementation.
  - **Proposed Solution:** Leverage the OHC Unified Ledger and the Finance AI Agent to continuously monitor cash flow, upcoming bookings, and inventory constraints. The system dynamically generates pre-approved Capital Offers. Repayment is handled invisibly via an automatic, zero-thought daily sweep (e.g., 10% of daily sales) until the principal plus a flat fee is repaid. No interest rates, no complex terms.

  ## Design Doc
  - **Architecture:** `MERCHANT` receives `CAPITAL_OFFER`, converts to `CAPITAL_ADVANCE`, which intercepts `TRANSACTION_LEDGER` credits to create `REPAYMENT_TRANSACTION`s.
  - **Mobile UX:** Proactive push notifications for capital offers based on business signals. Plain-language offer cards (e.g., "We give you $2,000. You pay a one-time fee of $150. We take 10% of daily sales until repaid").
  - **AI Integration:** The Finance Agent acts as the underwriter, monitoring the ledger for risk. The Operations Agent signals when capital is needed based on inventory bottlenecks.

  ## Implementation Prompt
  Implement the CapitalEngine microservice, backend data models, and mobile UI components for the Autonomous Working Capital feature.
  - **CUJ:** A merchant receives a capital offer, accepts it via FaceID, funds are instantly deposited into their OHC Wallet, and subsequent sales are automatically swept for repayment.
  - **Acceptance Criteria:** Create necessary schemas with tenant isolation. Implement the daily sweep event listener. Build the mobile UI offer card using the translucent glass design system.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
