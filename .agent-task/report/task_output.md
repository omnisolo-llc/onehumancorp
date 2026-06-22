issue_title: "Integrate TaxJar for Automated Sales Tax Compliance"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (Home Baker) and Priya (Boutique Operator) sell products across different states and regions. Calculating and remitting sales tax correctly is complex, time-consuming, and carries significant legal and financial risk if done incorrectly. Owners need a way to automatically calculate the correct sales tax at checkout based on the customer's location and track their tax liabilities without becoming tax experts.

  ## Research Report
  - **Tool Evaluated:** TaxJar (by Stripe)
  - **Market Need:** Competitors like Shopify, WooCommerce, and Square all offer native or easy integrations for automated sales tax calculation. Small business owners frequently cite sales tax compliance as a major administrative burden.
  - **User-First Value Mapping:** TaxJar simplifies sales tax by automatically applying the correct tax rates at checkout and compiling easy-to-read reports for filing. For an owner, this means no more guessing tax rates or manually updating spreadsheets; the assistant handles it transparently.
  - **Capabilities & Limits:** TaxJar provides a robust API for real-time sales tax calculations, nexus tracking, and reporting. It's built specifically for e-commerce and multi-channel sellers.
  - **SaaS Viability:** It integrates well with Stripe (which we already use). It has scalable pricing suitable for small businesses and offers API access for multi-tenant SaaS environments.

  ## Design Doc
  - **Integration Trigger:** A new integration card for "TaxJar" will be added to the Integrations page under a "Finance & Compliance" or "Operations" category.
  - **User Experience:**
    - Owners click "Connect" on the TaxJar integration card.
    - They are guided through an OAuth or API key setup process.
    - Once connected, a background sync enables automatic tax calculations on checkout flows (quotes, invoices, and direct payments).
    - The assistant can provide daily/weekly summaries of collected tax and upcoming filing deadlines.
  - **Checkout Flow Impact:** When a customer views a quote or checkout link, the system will transparently query TaxJar using the customer's shipping address and the seller's nexus information to add the correct tax line item.

  ## Implementation Prompt
  1. Add a "TaxJar" integration option in the `Integrations` UI (similar to Stripe or QuickBooks). The card should explain: "Automatically calculate and track sales tax for your orders."
  2. Implement the backend provider for TaxJar (`src/server/integrations/taxjar/provider.rs` and `client.rs`) following the existing integration pattern (e.g., like Shippo or Square).
  3. Update the Checkout/Quoting APIs to conditionally calculate and append sales tax line items if the TaxJar integration is active for the tenant.
  4. Ensure all changes are verified with comprehensive Playwright E2E tests simulating an owner connecting TaxJar and generating a quote with automated tax applied.

  ## Priority
  P1 (High) - Critical for scaling e-commerce capabilities and reducing legal risk for operators.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
