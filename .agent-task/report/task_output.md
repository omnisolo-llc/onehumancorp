issue_title: "Zero-Touch Global Tax Calculation & Automated Remittance Ledger"
issue_description: |
  # Zero-Touch Global Tax Calculation & Automated Remittance Ledger

  ## Title
  Zero-Touch Global Tax Calculation & Automated Remittance Engine

  ## Problem Statement
  Small business owners (Maya, Priya) struggle with complex, ever-changing multi-jurisdictional tax rules when selling online across states or countries. Maya, who sells custom cakes, shouldn't need a CPA to understand physical nexus versus economic nexus, nor should she manually calculate VAT/Sales Tax for every invoice. Currently, they either guess the tax rate, undercharge, or rely on expensive, disjointed third-party plugins that require manual remittance. The platform needs an invisible, automated tax engine that calculates exactly what is owed and queues up the remittances, allowing business owners to operate globally from day one without fear of compliance issues.

  ## Research Report
  - **Competitive Analysis:**
    - **Shopify:** Offers Shopify Tax, which automates calculations based on rooftop accuracy, but charges additional transaction fees and requires manual remittance steps.
    - **Wix:** Partners with Avalara for automated tax calculations, but integration requires external configuration and subscription fees.
    - **Stripe:** Provides Stripe Tax natively, but it lacks deep coupling with physical POS terminals and offline-first capabilities.
  - **OHC Architectural Gap:** OHC currently lacks a zero-trust, multi-tenant automated tax calculation and remittance ledger that works seamlessly across both online checkout and offline Tap-to-Pay POS. We need a system that caches tax profiles at the edge for offline calculating and uses the Finance Agent to autonomously set aside tax collected into a dedicated virtual ledger.
  - **Data Privacy & Compliance:** The engine must securely handle location and transaction data with zero-trust SPIFFE/SPIRE authorization to prevent cross-tenant leakage.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ TAX_NEXUS : "establishes"
      TAX_NEXUS ||--o{ TAX_RATE : "dictates"
      TENANT ||--o{ LEDGER_ENTRY : "owns"
      LEDGER_ENTRY ||--o{ TAX_REMITTANCE_QUEUE : "queues for payment"

      EDGE_CACHE ||--o{ TAX_RATE : "caches for offline POS"

      TENANT {
          uuid tenant_id
          string business_address
      }

      TAX_REMITTANCE_QUEUE {
          uuid remittance_id
          uuid tenant_id
          float amount
          string jurisdiction
          string status
      }
  ```

  ### UI Wireframes & Screen Flow (375px First)
  - **Settings > Taxes (Merchant View):** A simple toggle switch: "Automate My Taxes". The UI completely hides terms like "Economic Nexus", "VAT", and "GST". Once toggled, a small card displays: "We're currently collecting taxes in: CA, NY, and UK."
  - **Checkout (Customer View):** Customer enters their shipping address. The system calculates rooftop-accurate tax in < 100ms via Edge Cache and updates the total instantly without a page reload.
  - **Daily Briefing:** The AI Finance Agent sends a plain-language mobile notification: "Hey Priya, you collected $145 in sales tax this week. I've automatically set it aside in your Tax Reserve."

  ### Mobile UX Flow
  1. **Activation:** Priya enables "Automate My Taxes" during her Day One onboarding in just one tap.
  2. **Transaction:** Priya uses Tap-to-Pay on her phone for an in-person sale. The app checks the offline-synced local tax cache to add correct local sales tax even without Wi-Fi.
  3. **Reconciliation:** Upon reconnecting, the offline transaction syncs, and the AI Operations Agent automatically segregates the collected tax from her main operating revenue into a virtual sub-ledger.

  ### AI Agent Integration Points
  - **Finance Agent (Finance Dept):** Responsible for continuously monitoring economic nexus thresholds (e.g., crossing $100k in sales in a new state) and proactively alerting the merchant. It also manages the virtual Tax Reserve ledger, ensuring funds are available when remittance is due.
  - **Legal/Compliance Agent:** Reviews the types of products sold (e.g., physical goods vs. digital courses for Leo) and applies the correct product taxability codes invisibly based on catalog descriptions.

  ### Key Design Decisions
  - **Edge Caching for Tax Rates:** We will push generalized tax rate data to the Edge CDN and local SQLite databases. This allows offline-first POS Tap-to-Pay to continue calculating estimated taxes during network outages, bridging the gap upon reconnection via CRDT sync.
  - **Virtual Segregation Ledger:** Instead of letting merchants spend tax revenue accidentally, OHC will automatically sweep tax collected into a reserved virtual ledger account, preventing end-of-year tax shock.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the Zero-Touch Multi-Tenant Tax Calculation and Remittance Engine. Create the `TaxLedger` data models and integrate with the Edge Caching service to push localized tax rates to the edge for offline POS support. Implement the Finance Agent tool binding that automatically calculates taxes based on buyer location and product type, and subsequently sweeps the tax amount into a reserved `Tax_Remittance_Queue` upon successful payment. Ensure the merchant UI only exposes a single "Automate My Taxes" toggle and hides all complex compliance configurations behind an "Advanced Settings" flag. Use SPIFFE/SPIRE for secure cross-service authentication when accessing the ledger.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
