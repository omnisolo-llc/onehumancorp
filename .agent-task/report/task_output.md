issue_title: "Invisible AI Fraud Prevention & Dispute Resolution Engine"
issue_description: |
  **Findings:**
  Small business owners often lose money to chargebacks, friendly fraud, and payment disputes due to a lack of time and technical expertise. Existing solutions like Stripe Radar or Shopify Fraud Protect are often complex and require manual evidence compilation for disputes. This presents an opportunity for OneHumanCorp to build an invisible, autonomous layer that intercepts risky transactions and automates dispute compilation and submission.

  **Proposed Next Steps:**
  - Create the backend microservice for the `Fraud Prevention & Dispute Resolution Engine`.
  - Implement the `Interceptor` to evaluate incoming transactions against AutoDream memory and gateway signals.
  - Integrate a `Verification Mesh` that triggers SMS via Teammate Mesh for high-risk but legitimate-looking orders.
  - Build the `Dispute Auto-Compiler` to listen for `chargeback.created` webhooks, fetch order and communication history from the Universal Ledger and AutoDream, and compile/submit evidence.
  - Develop a mobile-first, glassmorphic UI alert card for the merchant's unified inbox to notify them of resolved disputes without requiring manual intervention.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
