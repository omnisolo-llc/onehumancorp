issue_title: "[Architecture] Global Multi-Currency & Cross-Border Engine"
issue_description: |
  # Global Multi-Currency & Cross-Border Engine

  ## Problem Statement
  Small business owners frequently lose international sales because presenting prices in a foreign currency creates friction and distrust. They need an automated system that handles multi-currency pricing, local payment methods, and automated FX reconciliation without manual configuration.

  ## Research Report
  Our platform evaluation highlights that while Shopify and Stripe offer robust capabilities, they expose high complexity to merchants. OHC needs a zero-config engine that hides FX risk and localization logic, providing "Cosmetic Pricing" (e.g., charming price endings) and support for Local Payment Methods (LPMs) out of the box.

  ## Design Doc
  - **Architecture Diagram:**
    ```mermaid
    graph TD;
      subgraph Mobile Client (375px)
          App[OHC Mobile App] --> API[OHC API Gateway];
      end

      subgraph OHC Backend
          API --> LocaleDetector[Locale & IP Detector];
          LocaleDetector --> PricingService[Dynamic Pricing & FX Service];
          PricingService --> DB[(Postgres Main DB)];
      end

      subgraph External
          PricingService --> Stripe[Stripe Terminal API];
      end
    ```

  - **UI Wireframes / Mobile UX Flow (375px First):**
    - The storefront automatically renders the currency formatted nicely in the native locale using glassmorphism.
    - No settings menu interaction is required for the user; however, an "Advanced Settings" switch allows power users to lock exchange rates.

  - **AI Agent Integration Points:**
    - The **Finance Agent** automatically monitors FX conversion costs and alerts the merchant if a specific corridor becomes unprofitable.

  ## Implementation Prompt
  Design and implement the Global Multi-Currency Engine according to the zero-config activation and cosmetic rounding strategies. Ensure that the core ledger transactions strictly adhere to the merchant's home currency, abstracting FX fluctuations into an OHC-managed risk pool. Ensure complete test coverage.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
