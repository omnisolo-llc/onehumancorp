issue_title: "[architecture] Autonomous Omnichannel Tax & Compliance Engine"
issue_description: |
  # Title
  Autonomous Omnichannel Tax & Compliance Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Priya (boutique owner) face significant anxiety and financial risk when dealing with sales tax across multiple jurisdictions. Existing platforms require them to understand complex tax codes, manually set up tax zones, and export CSVs for an external accountant or tool to figure out how much they owe to the government. This creates "Financial Fog" and "Setup Complexity". OHC needs an invisible engine that calculates, collects, and reserves the exact right amount of tax for every transaction across any channel automatically, so the owner never has to think about it.

  ## Research Report
  - **Competitor Audit**:
    - Shopify: Relies heavily on Shopify Tax or third-party apps like TaxJar/Avalara. Setup requires users to manually define physical presences (nexus) and tax overrides for specific product categories (e.g., clothing vs. digital goods).
    - Wix: Requires manual tax configuration or integration with Avalara. Highly confusing for non-technical users.
  - **The Gap**: OHC currently lacks an integrated, zero-config capability that handles dynamic tax calculation out-of-the-box using the AI Agent Departments to classify products and apply appropriate rates seamlessly without user intervention.
  - **Market Data**:
    - "Financial Fog" is a top 10 pain point for SMBs.
    - Multi-state sales tax nexus rules are a leading cause of accidental non-compliance for growing solopreneurs.
    - A zero-config tax system perfectly aligns with OHC's "zero -> live in under 10 minutes" mandate.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Checkout Service
      participant Operations Agent
      participant Finance Agent
      participant External Tax API (e.g. TaxJar)
      participant Ledger

      Customer->>Checkout Service: Initiates checkout (Cart + Location)
      Checkout Service->>Operations Agent: Request item tax classification
      Operations Agent-->>Checkout Service: Item classifications (e.g., "Food", "Digital")
      Checkout Service->>Finance Agent: Request real-time tax calculation
      Finance Agent->>External Tax API: Fetch precise local rates & rules
      External Tax API-->>Finance Agent: Calculated tax breakdown
      Finance Agent-->>Checkout Service: Final tax amount
      Checkout Service->>Customer: Presents total price
      Customer->>Checkout Service: Completes payment
      Checkout Service->>Ledger: Split funds (Revenue vs Tax Reserve)
  ```

  ### Mobile UX Flow (375px First)
  1. **Zero-Config Setup**: During onboarding, Maya never sees a "Tax Zones" page. The Operations Agent simply asks, "Where are you shipping from?"
  2. **Checkout Experience (Customer)**: The customer enters their zip code. The tax is instantly calculated and displayed transparently in the cost breakdown.
  3. **Dashboard (Business Owner)**: Maya sees a clear "Tax Collected" line item in her Daily Briefing card. No complex menus or CSV exports.
  4. **Payouts**: The dashboard clearly delineates the "Available to Payout" vs "Tax Reserved in Escrow" (if integrated with the Treasury wallet), removing the risk of accidentally spending tax money.

  ### AI Agent Integration Points
  - **The Vigilant Manager (Operations)**: Automatically classifies inventory items into correct tax categories based on images or descriptions without Maya needing to know the technical tax codes.
  - **The Accountant (Finance)**: Actively monitors sales velocity to detect potential nexus triggers in new states. Communicates with Maya in plain English: "You're selling a lot in New York! I've set up tax collection for NY starting today."

  ### Key Design Decisions and Why
  - **Zero-Touch Configuration**: We completely hide tax rules, nexus thresholds, and rate tables from the user.
  - **Item Classification via AI**: Because taxability depends on the item type (e.g., grocery vs prepared food), the Operations Agent handles this categorization silently during product creation.
  - **Ledger Segregation**: Tax collected must be cryptographically segregated in the ledger from general revenue to prevent "Financial Fog".

  ## Implementation Prompt
  **To Implementer Agent:**
  Build the Autonomous Omnichannel Tax & Compliance Engine.
  1. Integrate a reliable real-time tax calculation provider (like TaxJar or Stripe Tax) behind the Finance Agent.
  2. Implement the background classification mechanism in the Operations Agent to automatically tag products with the correct tax codes based on their descriptions/images.
  3. Update the Checkout flow to call the Finance Agent for dynamic tax calculation before finalizing the cart total.
  4. Ensure the `Ledger` service handles splitting the incoming payment into a 'Revenue' bucket and a 'Tax Reserve' bucket.
  5. Create a simple "Tax Overview" UI card for the mobile dashboard that explains collected taxes in plain English. No complex tax zone configuration pages should be built.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
