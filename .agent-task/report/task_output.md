issue_title: "Multi-Currency Checkout and Localized Invoicing Architecture"
issue_description: |
  ## Problem Statement
  Small business owners and creators often have international customers but struggle with multi-currency pricing and localized invoicing. Existing platforms either require expensive add-ons or force all transactions into a single base currency, causing confusion for buyers and accounting headaches for the seller. A true "owner work assistant" should handle currency conversion, localized invoice generation, and tax implications invisibly.

  ## Research Report
  **Market Context:** E-commerce giants like Shopify offer multi-currency support, but it's often tied to their specific payment gateway (Shopify Payments) and higher-tier plans. Simpler builders like GoDaddy and Wix have rudimentary or non-existent native multi-currency support for smaller merchants.
  **The OHC Opportunity:** By integrating multi-currency natively into the checkout flow and having the Finance AI Agent automatically handle localized invoicing, OHC can empower micro-SMEs to go global without technical or accounting overhead.
  **Competitor Gaps:**
  - *Shopify*: Good multi-currency, but complex setup and tied to their payment ecosystem.
  - *Wix/Squarespace*: Limited native support; often relies on third-party apps for robust localized invoicing.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TenantSettings ||--o{ Currency : enables
      Currency {
          string code PK
          decimal exchange_rate
          datetime last_updated
      }
      Product ||--o{ ProductPrice : has
      ProductPrice {
          uuid product_id FK
          string currency_code FK
          decimal price
      }
      Order ||--|| Invoice : generates
      Invoice {
          uuid id PK
          string currency_code
          decimal converted_base_amount
          jsonb tax_details
      }
  ```

  ### Data Model (PostgreSQL)
  - `Currency`: Supported currencies and exchange rates (updated periodically).
  - `ProductPrice`: Allows defining specific prices for different currencies, overriding automatic conversion.
  - `Invoice`: Stores transaction details, including the original currency, converted base currency amount, and localized tax information.
  - `TenantSettings`: Configures base currency and enabled target currencies.

  ### Key Design Decisions
  - **Dynamic Exchange Rates vs. Manual Overrides**: The system will fetch and update exchange rates daily for automatic conversion, but `ProductPrice` allows merchants to set "clean" manual prices (e.g., €19.99 instead of €21.34) for specific markets.
  - **Base Currency Normalization**: All financial reporting and ledger entries (in `Order` and `PaymentLedger`) will be normalized to the tenant's base currency to ensure accurate revenue tracking. The `Invoice` will maintain a record of the transaction currency and the applied conversion rate.
  - **Agentic Invoicing**: Instead of relying on rigid templates, the Finance Agent will dynamically draft invoices based on the buyer's locale (e.g., formatting dates and currencies correctly, including required tax IDs like VAT).

  ### AI Integration
  - **Finance Agent ("The Accountant")**: Automatically generates localized invoices in the customer's currency, handles exchange rate reconciliation for the owner's dashboard, and flags potential cross-border tax liabilities.
  - **Operations Agent ("The Manager")**: Ensures shipping/fulfillment logic respects international boundaries and currency constraints.

  ### Mobile UX Flow (375px)
  1. **Owner View (Dashboard)**: The owner sets a base price. A simple toggle enables "Global Selling". The interface shows automatic conversions for key markets with options to set manual overrides.
  2. **Customer View**: The storefront automatically detects the user's locale (or allows manual selection) and displays prices in their local currency. The checkout process is seamless, and they receive a localized invoice immediately.

  ## Implementation Prompt
  **Feature Name**: OHC Global Commerce & Localized Invoicing
  - Implement the `Currency` and `ProductPrice` data models to support multi-currency pricing.
  - Integrate with a reliable exchange rate API for automatic conversions.
  - Update the checkout flow to support local currency selection and processing via Stripe.
  - Enhance the Finance Agent to automatically generate and send localized invoices (PDF format) upon successful payment.
  - Create a mobile-first (375px) settings screen for owners to manage their enabled currencies and pricing strategies.
  - Ensure all database queries and AI agent interactions respect strict tenant isolation.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []