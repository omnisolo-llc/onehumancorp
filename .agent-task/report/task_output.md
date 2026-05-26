issue_title: "Invisible Autonomous Tax & Global Compliance Engine"
issue_description: |
  # Title: Invisible Autonomous Tax & Global Compliance Engine

  ## Problem Statement
  Small business owners like Priya (Boutique owner selling physically and online) and Leo (Music tutor taking international students) face immense friction when calculating sales tax, VAT, or local compliance requirements. Tax engines are historically difficult to configure, require manually defining tax nexuses, and break during checkout if an API fails. A non-technical small business owner cannot, and should not, have to understand the nuances of cross-border digital goods tax laws or specific local sales taxes. We need an "Invisible Autonomous Tax & Global Compliance Engine" that automatically calculates and remits taxes securely based on their location and customer location, while functioning completely offline for POS scenarios and isolating tenant data.

  ## Research Report
  - **Competitor Limits:**
    - Shopify: Relies heavily on Shopify Tax or third-party apps (Avalara, TaxJar) which require the user to configure nexuses and categories.
    - Wix: Uses standard Avalara integrations but still requires significant manual setup.
    - Stripe Tax: Good API, but developers must manage the edge cases, local caching, and fallback UI if offline.
  - **Discovery:** OHC is missing an intelligent, autonomous tax compliance layer that is deeply embedded in the "Legal & Compliance" department ("The Protector"). This layer must be zero-config (automatically pulling merchant identity and location), function completely seamlessly during offline Tap-to-Pay POS interactions via edge-cached tax profiles, and ensure Zero-Trust multi-tenant isolation.

  ## Design Doc

  ### Key Design Decisions
  - **Zero-Config Inference:** The engine will infer the merchant's location and tax obligations using the OHC-SIP DB profile during onboarding, requiring no manual setup.
  - **Offline-First Resilience:** For POS and mobile usage, the engine will edge-cache localized tax rates (CRDTs) to allow transactions to succeed offline, reconciling safely once back online.
  - **Zero Trust Multi-Tenancy:** Each tenant's tax liabilities and cached rules are strictly isolated using PostgreSQL RLS and authenticated via SPIFFE/SPIRE identity.

  ### AI Agent Integration Points
  - **Legal & Compliance ("The Protector"):** Continually monitors tax nexus thresholds in the background and alerts the user ("The Advisor") in plain English if they cross a new threshold.
  - **Finance & Payments ("The Accountant"):** Automatically tags tax funds from gross revenue, setting them aside in an isolated treasury sub-balance to prevent the owner from accidentally spending tax money.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT-PROFILE ||--o{ PROTECTOR-AGENT : "Infers Location & Nexus"
      PROTECTOR-AGENT ||--o{ TAX-ENGINE-CORE : "Configures Rules"
      TAX-ENGINE-CORE }|--|| EDGE-CACHE : "Syncs Offline Tax Rates"
      MOBILE-POS ||--o{ EDGE-CACHE : "Calculates Offline"
      CHECKOUT-WEB ||--o{ TAX-ENGINE-CORE : "Calculates Online"
      TAX-ENGINE-CORE ||--o{ ACCOUNTANT-AGENT : "Tags Revenue"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  - **Customer Checkout (Web/Mobile):** Taxes appear instantly with zero loading spinners; automatically dynamically localized to the buyer's IP or shipping address.
  - **Merchant Mobile POS (375px):** During an offline sale, the UI cleanly adds the cached local tax percentage automatically ("+ $0.50 Tax").
  - **Advisory Alert (Mobile Push):** "Hi Priya, you've started selling a lot in California! I've automatically updated your tax settings to comply with CA law. No action needed on your part." (Passes the Grandmother Test).

  ## Implementation Prompt
  Implement the "Invisible Autonomous Tax & Global Compliance Engine" within the core OHC platform. Develop the `TAX-ENGINE-CORE` that interfaces securely with the existing `MERCHANT-PROFILE` and the "Legal & Compliance" Agent ("The Protector"). Create an edge-caching sync mechanism that pulls local tax rates to the `MOBILE-POS` client for instant, offline-capable calculations. Ensure that the Finance Agent ("The Accountant") is triggered post-transaction to reconcile tax amounts securely. All database interactions must respect strict multi-tenant isolation rules. The system must operate invisibly to the merchant, calculating correct taxes for both digital and physical goods without any manual configuration.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
