issue_title: "Scout: Tool Integration Research - Afterpay"
issue_description: |
  **Title**: Implement Afterpay (Buy Now, Pay Later) Integration to Increase Small Business Sales

  **Problem Statement**:
  Small business owners, especially those selling physical products or higher-ticket services (like custom jewelry, boutique clothing, or photography packages), often struggle with abandoned carts and price objections. Customers may hesitate to spend $200 upfront. While enterprise retailers offer flexible financing, our non-technical small business owners lack a simple, one-click way to offer installment payments without taking on credit risk or managing complex underwriting processes.

  **Research Report**:
  *   **Market Need & Discovery**: Analysis of competitor ecosystems (Shopify, Wix, Square) reveals that BNPL (Buy Now, Pay Later) integrations are among the most installed and requested features. Subreddits like r/smallbusiness and r/ecommerce frequently highlight BNPL as a critical driver for increasing Average Order Value (AOV) and conversion rates.
  *   **Tool Selected**: Afterpay (by Block/Square).
  *   **Value Proposition**: Afterpay allows end-consumers to split their purchases into 4 interest-free payments over 6 weeks. The small business gets paid the full amount upfront (minus fees) and assumes zero fraud or credit risk.
  *   **Ease of Use for Non-Technical Users**: Exceptional. SMBs simply toggle "Enable Afterpay" in their dashboard. The integration handles the checkout flow redirection and return. No coding or complex API keys are required if we use an OAuth/Partner integration.
  *   **Pricing & Viability**: Afterpay typically charges a per-transaction fee (around 6% + $0.30), which is higher than standard credit cards, but justified by the ~20-30% increase in conversion and up to 40% higher AOV. It has robust APIs and webhooks, functioning perfectly in Cloud (multi-tenant) via a Master Merchant model, and Standalone environments.

  **Design Doc**:
  *   **Trigger**: End-customer reaches the checkout step on a small business's OHC storefront. If the SMB has enabled Afterpay and the cart value is within limits (e.g., $35 - $1000), an "Afterpay" payment option appears.
  *   **Action**: Selecting Afterpay redirects the user to the Afterpay hosted checkout to log in and approve the payment schedule. Upon approval, they are redirected back to the OHC order confirmation page. OHC's backend captures the authorization via webhook/API to complete the order.
  *   **User Interface (SMB Dashboard)**: A simple settings card under "Payments": "Offer Afterpay: Let customers pay in 4 installments. You get paid upfront." A toggle switch and a dynamic fee disclosure.

  **Implementation Prompt**:
  We need to add Afterpay as a turnkey payment method for our merchants.
  Acceptance Criteria:
  1.  Merchants can enable Afterpay with a single toggle in their Settings -> Payments dashboard.
  2.  During checkout, eligible orders display the "Pay in 4 interest-free payments" widget near the total.
  3.  Selecting the option successfully routes the customer through the Afterpay flow and returns them to an order success page.
  4.  The merchant dashboard clearly reflects the full payout amount upfront, deducting the Afterpay processing fee, so there is no confusion about when the merchant gets their money.
  5.  If an order is cancelled or refunded, the integration automatically issues a refund to the Afterpay ledger.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
