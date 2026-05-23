issue_title: "Implement Autonomous Multi-Jurisdiction Tax & Compliance Engine"
issue_description: |
  **Problem Statement**: Small business owners face immense friction understanding and managing sales tax, VAT, and economic nexus thresholds. Existing platforms like Shopify or Wix require users to manually understand their tax liabilities, monitor thresholds, and configure tax rates or install expensive third-party apps like TaxJar or Avalara. This is a terrifying, highly technical process for a non-technical user. If OHC is the invisible operations team, it must handle tax tracking, collection, and filing preparation autonomously without requiring the user to become a tax accountant.

  **Research Report**:
  - **Competitor Landscape**: Shopify and Wix push tax liability monitoring onto the merchant or require expensive 3rd party integrations. Stripe Tax has a great API but developer-centric UX.
  - **User Psychology**: Users fear tax penalties. They want a "set it and forget it" solution.
  - **The OHC Differentiator**: OHC employs a Legal/Finance Agent that actively monitors sales volume per jurisdiction, alerts the user *before* they hit an economic nexus, and automatically applies the correct tax rate to the checkout without manual configuration.
  - **Key Findings**: OHC must automatically geolocate the buyer, classify product taxability, and calculate real-time tax at checkout, storing the ledger data for the autonomous reporting engine.

  **Design Doc Summary**:
  - *Invisible Configuration*: Automatically determines the seller's home jurisdiction.
  - *Autonomous Nexus Tracking*: Passive tracking of sales volume against economic nexus thresholds.
  - *Real-Time Product Classification*: AI automatically maps product descriptions to standard tax codes.
  - *Zero-Jargon UI*: Simple alerts for users nearing tax thresholds.
  - A full Markdown architecture brief is available at `docs/research/[architecture]_autonomous_tax_and_compliance_engine.md`.

  **Implementation Prompt**:
  Implement the Autonomous Tax & Compliance Engine architecture. Create the background Nexus Tracking Worker that consumes ledger events and evaluates them against predefined jurisdiction thresholds. Integrate a third-party tax calculation provider (e.g., Stripe Tax) into the checkout flow. Develop the Finance Agent behavior to trigger plain-language notifications when nexus thresholds are approached. Build the mobile-first "Taxes" summary view and 1-tap approval bottom sheet. Do not prescribe specific database schemas.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
