issue_title: "[Architecture] Autonomous Dynamic Pricing & Yield Management Engine"
issue_description: |
  # [Architecture] Autonomous Dynamic Pricing & Yield Management Engine

  ## Problem Statement
  Small business owners often struggle with optimizing pricing based on demand, capacity, and timing. For instance:
  - **Priya (boutique owner)** manually marks down unsold inventory instead of proactively adjusting prices for slower-moving variants.
  - **Leo (music tutor)** charges a flat rate, missing the opportunity to charge premium rates for prime time slots or discount less popular ones to ensure a full calendar.
  - **Carlos (handyman)** loses potential revenue by not adjusting his quotes dynamically when his availability is constrained or demand for emergency repairs spikes.

  Current platforms (like Shopify, Wix, or Calendly) either lack native dynamic yield management or require complex third-party tools that a non-technical SMB owner cannot configure. OneHumanCorp needs an invisible, intelligent engine that dynamically adjusts prices and quotes based on real-time inventory, booking capacity, and market demand without requiring the user to do any complex setup.

  ## Research Report
  **Market Gap Analysis:**
  - Standard eCommerce and booking platforms generally treat pricing as static. Shopify requires complex apps like "Bold Custom Pricing" which are hard to manage.
  - Service platforms (like Calendly) have no yield management—they simply block slots. Airlines and hotels use yield management to maximize revenue, but this technology is largely inaccessible to SMBs.
  - Uber and Airbnb have normalized dynamic pricing for consumers. Small businesses could increase their revenue by 10-20% by adopting similar strategies invisibly.

  **Competitor Approaches:**
  - **Shopify/Wix:** Static pricing. Discounts require manual creation of rules.
  - **Mindbody/Acuity:** Basic dynamic pricing exists for classes, but is clunky and requires heavy manual configuration.
  - **Stripe:** Supports flexible pricing via API, but leaves the logic to the implementer.

  ## Design Doc

  **Data Model & Relationships:**
  Entities required:
  - `YieldProfile`: The dynamic pricing configuration attached to a product, service, or booking type.
  - `CapacityState`: Real-time tracking of available inventory or calendar slots.
  - `DemandSignal`: Aggregated metrics (e.g., page views, booking velocity, local events).
  - `PriceAdjustmentEvent`: Audit log of when and why a price was adjusted.

  *Mermaid ER Diagram:*
  ```mermaid
  erDiagram
    TENANT ||--o{ YIELD_PROFILE : configures
    YIELD_PROFILE ||--|{ INVENTORY_ITEM : applies_to
    YIELD_PROFILE ||--|{ SERVICE_SLOT : applies_to
    YIELD_PROFILE ||--o{ DEMAND_SIGNAL : consumes
    YIELD_PROFILE ||--o{ PRICE_ADJUSTMENT_EVENT : generates
  ```

  **AI Department Coordination:**
  - **Operations Agent:** Monitors inventory depletion rates and calendar capacity.
  - **Finance Agent:** Calculates the optimal price point to maximize yield without deterring conversion, applying rules like "never go below cost basis + 10%".
  - **Marketing Agent:** If yield drops (e.g., empty slots tomorrow), it triggers an automated promotion or nudges waitlisted customers with a "last-minute deal".

  **Mobile-First UX Flow:**
  1. User (Maya/Leo) creates a new service or product.
  2. A clean toggle defaults to ON: "Enable AI Smart Pricing".
  3. A simple bottom sheet (Action Sheet) slides up allowing the user to set a minimum floor price and maximum ceiling price (e.g., "$50 - $100 per hour").
  4. The rest is fully invisible. A dashboard card occasionally reports: "AI Smart Pricing earned you an extra $150 this week by optimizing your prime time slots."

  ## Implementation Prompt
  **To the Implementer:**
  Create the core Autonomous Dynamic Pricing & Yield Management Engine. You must build the backend services and AI agent orchestration that monitor inventory/capacity and adjust pricing dynamically based on demand signals.

  - Ensure the system respects the user's defined floor and ceiling prices.
  - Implement the multi-tenant `YieldProfile` and `PriceAdjustmentEvent` entities.
  - Provide an internal API that the storefront and booking flows call to get the *current* dynamic price.
  - Ensure all database queries are isolated by tenant to guarantee Zero Trust security.
  - Build the mobile-first React components (375px viewport optimized) for the "Smart Pricing" toggle and min/max configuration sheet, adopting macOS-style Translucent Glass materials.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
