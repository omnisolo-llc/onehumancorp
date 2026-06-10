issue_title: "Implement Real-time AI-driven Dynamic Tax and Compliance Integration for Multi-regional SMBs"
issue_description: |
  # Research Report: Real-time AI-driven Dynamic Tax and Compliance Integration for Multi-regional SMBs

  ## Target Persona: Priya (Boutique Owner) & Carlos (Field Service Owner)

  ## Problem Statement
  As small business owners expand their operations across state or national borders (e.g., Priya shipping boutique clothing out-of-state, or Carlos operating field services across multiple county lines), they encounter complex, varying tax regulations. These regulations can change based on the physical location of the business, the destination of the service/product, and the type of product sold. Most non-technical owners find it extremely difficult and stressful to manually track these rules, configure their platforms appropriately, and ensure compliance. This gap leads to potential legal liabilities and manual overhead that an "assistant-first" tool like OHC must resolve autonomously.

  ## Architecture & Design Flow

  ### Data Model & Invariants
  - **Tax Configuration Profile (PostgreSQL):** A schema extension capturing the tenant's primary nexus locations, product taxability categories, and regional tax overrides.
  - **Dynamic Tax Calculator Service:** An internal service connecting to external tax rate APIs (e.g., Stripe Tax, Avalara) while keeping an edge-cached fallback for high-availability.
  - **Checkout Interceptor:** The checkout flow will transparently query the tax calculator to append the correct tax amount dynamically before finalizing the payment intent.

  ### AI Agent Integration
  - **Finance Agent ("The Accountant"):** Continuously monitors the locations of sales. If a merchant hits a "nexus" threshold in a new region, the agent proactively flags this in the owner's feed and drafts a compliance setup form.
  - **Operations Agent ("The Manager"):** Ensures newly added products are correctly categorized for tax purposes by analyzing their descriptions.

  ### Mobile UX Flow
  - A simple card in the 375px mobile feed alerts the owner: "You've reached the tax threshold in California. Let's set up tax collection."
  - The owner taps "Approve", and the Finance Agent securely updates the billing and checkout configurations without the user ever opening a complex settings dashboard.

  ## Implementation Prompt
  - Create the `TaxConfiguration` entity in the central PostgreSQL database.
  - Integrate a dynamic tax calculation middleware in the checkout and POS flows.
  - Build the AI Agent triggers that notify the owner when they are approaching tax nexus limits.
  - Implement the mobile-first approval card to activate tax collection for new regions.
  - Do NOT prescribe specific third-party tax APIs or detailed function signatures; let the implementer define those based on system requirements.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []