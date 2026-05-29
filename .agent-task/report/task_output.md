issue_title: "Autonomous Multi-Currency & Cross-Border Localization Engine"
issue_description: |
  ## Problem Statement
  Currently, OneHumanCorp (OHC) lacks native support for cross-border commerce and multi-currency transactions. When a business owner like Maya (the baker) or Priya (the boutique owner) tries to sell internationally, they are forced to manually calculate exchange rates, manage separate localized product listings, and navigate complex international tax and shipping rules. This friction completely breaks the OHC promise of "zero → live business in under 10 minutes" for anyone wanting a global reach. Competitors like Shopify require expensive third-party apps or advanced plans for robust internationalization, while Wix's native support is rudimentary.

  ## Research Report
  ### Findings & Competitive Analysis
  - **Shopify**: Offers "Shopify Markets" which handles localized domains, currency conversion, and duty calculation. However, it is notoriously complex to configure, often requiring developer assistance to customize storefront themes and handle payout routing.
  - **Wix**: Supports multi-currency display but lacks deep, automated backend localization (e.g., dynamic tax withholding based on localized thresholds).
  - **The Gap**: No platform currently offers an *autonomous* engine that automatically translates catalogs, normalizes prices dynamically based on localized purchasing power parity (PPP), and handles cross-border tax compliance without the merchant needing to configure any rules manually.

  ## Design Doc
  ### Mobile UX Flow (375px first)
  1. **Settings > Global Reach:** A single toggle switch: "Enable Global Sales."
  2. **AI Configuration:** The AI automatically scans the catalog and generates a summary card: "Your products are now available in 140 currencies. Prices are optimized for local markets. We handle the VAT/Duties."
  3. **Order View:** When an international order comes in, the total is shown in the merchant's base currency, with a small Info icon showing the buyer's original currency and the exact conversion rate used.

  ### AI Agent Integration Points
  - **Marketing Agent:** Automatically translates product titles, descriptions, and SEO metadata into the buyer's local language at the edge.
  - **Finance Agent:** Subscribes to real-time FX rate feeds (via NATS event mesh), manages multi-currency ledgers, and handles localized payout conversions to minimize FX fees.
  - **Legal/Compliance Agent:** Dynamically applies local tax rules (VAT, GST) and cross-border duties during the zero-click checkout process.

  ### Data Model & Invariants
  **Multi-Tenant Isolation & Zero Trust:**
  - All ledger and pricing data must strictly enforce isolation via a mandatory `merchant_id` partition key.
  - Direct database access is restricted; operations must go through GRPC/SPIFFE authenticated services where the identity token explicitly defines the tenant boundary.
  - Cross-tenant data bleeds (e.g., mixing localized price models) are structurally impossible due to isolated tenant KMS encryption keys for sensitive financial records.

  **Architecture Diagrams:**

  *Entity-Relationship Diagram:*
  ```mermaid
  erDiagram
      MERCHANT ||--o{ CATALOG_ITEM : owns
      CATALOG_ITEM ||--o{ LOCALIZED_VARIANT : generated_by_ai
      LOCALIZED_VARIANT {
          string currency
          decimal local_price
          string language
          string localized_description
      }
      ORDER ||--|| EXCHANGE_RATE_SNAPSHOT : locks
      ORDER ||--o{ LINE_ITEM : contains
      ORDER {
          string base_currency
          decimal base_total
          string buyer_currency
          decimal buyer_total
      }
      EXCHANGE_RATE_SNAPSHOT {
          string source_currency
          string target_currency
          decimal rate
          timestamp valid_at
      }
  ```

  *Sequence Diagram (Zero-Click Multi-Currency Checkout):*
  ```mermaid
  sequenceDiagram
      participant Buyer
      participant EdgeCDN as CDN Edge Cache
      participant Gateway as API Gateway (Zero Trust)
      participant Compliance as Legal Agent (VAT/Duties)
      participant Finance as Finance Agent (FX)

      Buyer->>EdgeCDN: Request Product Page (from EU IP)
      EdgeCDN-->>Buyer: Return Localized Page (EUR)
      Buyer->>Gateway: Initiate Checkout (Zero-Click)
      Gateway->>Compliance: Calculate VAT for EUR Buyer
      Compliance-->>Gateway: VAT Applied
      Gateway->>Finance: Lock FX Rate (USD to EUR)
      Finance-->>Gateway: Rate Locked
      Gateway-->>Buyer: Show Final Total (EUR)
  ```

  ### Key Design Decisions
  - **Edge Translation:** Content translation and currency conversion must happen at the CDN edge (multi-tenant edge caching) to ensure ultra-low latency.
  - **Locked Exchange Rates:** Exchange rates must be snapshotted and locked at the time of order creation to prevent ledger discrepancies during refunds.
  - **Zero-Config PPP:** The system should default to using Purchasing Power Parity to suggest localized prices, allowing merchants to maximize global revenue without manual pricing tiers.

  ## Implementation Prompt
  **To the Implementer:**
  Build the backend services and mobile-first UI for the "Autonomous Multi-Currency & Cross-Border Localization Engine". The primary CUJ is a merchant flipping a single "Enable Global Sales" switch. Upon activation, the system must automatically (via AI agents) localize product listings, support dynamic multi-currency checkout, and handle all cross-border tax calculations without any further manual configuration. Acceptance criteria include:
  1. A single toggle enables the feature.
  2. Buyers see prices in their local currency based on GeoIP.
  3. The checkout flow automatically calculates and collects necessary local taxes (e.g., EU VAT).
  4. The merchant ledger reflects transactions normalized to their base currency with FX rates locked at checkout.
  5. The mobile UI for managing global settings and viewing international orders must pass the "grandmother test" (perfectly usable on a 375px viewport).

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
