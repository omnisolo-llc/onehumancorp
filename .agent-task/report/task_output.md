issue_title: "Multi-Location & Franchise Hierarchy Architecture"
issue_description: |
  # Research Report: Agentic Multi-Location & Franchise Hierarchy Architecture

  ## 1. Problem Statement
  As OneHumanCorp (OHC) scales its reach, the persona "Jun (Location Manager)" becomes a critical user. Jun runs the day-to-day operations for one specific site of a larger operation (e.g., a multi-location coffee shop or a boutique franchise). Currently, OHC assumes a flat `tenant_id` structure where one tenant equals one business with one global inventory and unified staff access.

  Competitors like Shopify and Square support multi-location natively, but they approach it as a purely administrative database feature. Jun struggles because existing tools do not offer an AI-native way to coordinate staff, manage location-scoped inventory, or escalate local issues to the regional owner automatically.

  ## 2. Research Report
  - **Market Context**: Traditional systems (Shopify, Square) allow merchants to add "locations" and track inventory per location. However, this often results in a complex, manual routing of online orders and fragmented reporting. Square excels at multi-location POS, but lacks agentic coordination.
  - **The OHC Opportunity**: OHC can differentiate by treating locations not just as inventory buckets, but as "Autonomous Hubs." The Operations Agent at a specific location should be able to predict local stockouts and automatically communicate with the Regional Owner or the central warehouse agent.
  - **Competitor Gaps**:
    - *Shopify*: Multi-location is data-heavy. Routing rules are static (e.g., "always fulfill from Location A first").
    - *Square*: Excellent reporting per location, but no proactive AI to tell the location manager *what* to do based on those reports.
    - *Wix*: Very basic multi-location support, often requiring third-party plugins for complex local pickup and delivery routing.

  ## 3. Design Doc
  ### Architecture Diagram & Data Model
  ```mermaid
  erDiagram
      ORGANIZATION ||--o{ TENANT_LOCATION : owns
      TENANT_LOCATION ||--o{ PRODUCT_INVENTORY : holds
      TENANT_LOCATION ||--o{ STAFF : employs
      TENANT_LOCATION ||--o{ AGENT_WORKER : hosts
      PRODUCT ||--o{ PRODUCT_INVENTORY : "has stock at"
  ```
  - **Data Model (PostgreSQL)**:
    - Introduce an `organization_id` (parent) and `location_id` (child tenant) hierarchy.
    - `inventory_count` must move from the global `products` table to a location-scoped `product_inventory` ledger.
    - Staff Members are scoped to specific locations or globally.

  ### Mobile UX Flow (375px)
  1. **Location Switcher**: A seamless, translucent glass dropdown at the top of the app shell (375px) allowing Jun to see his location's Agent Feed, or the Owner to swipe between locations.
  2. **Location-Scoped Feed**: Jun's Agent Feed only shows Action Cards for his location (e.g., "Store 2: Low Stock on Coffee Beans", "Store 2: New Pickup Order").
  3. **Escalation Flow**: An Action Card for Jun might have an "Escalate to Owner" button. If an irate customer leaves a bad review, the Customer Success Agent drafts an escalation summary for the regional owner and sends it to the owner's global feed.

  ### AI Agent Integration Points
  - **Operations Agent**: Now operates in a "location-aware" context. When an online order comes in, the agent dynamically routes it to the optimal location based on real-time stock and distance, rather than static rules.
  - **Finance Agent**: Generates plain-language daily performance summaries comparing Location A to Location B for the owner, while giving Jun a summary focused only on Location A's daily targets.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Multi-Location Hierarchy & Agentic Routing
  **Target Persona**: Jun the Location Manager
  **Outcome**: Jun can manage a single location's inventory, staff tasks, and customer feedback through a localized Agent Feed, while the system automatically handles inventory sync and issue escalation to the regional owner.

  **Next Actions**:
  1. Implement the hierarchical data model (`organization` -> `location`), refactoring the central ledger to support location-scoped inventory.
  2. Update the `Operations Agent` context to be location-aware, enabling smart order routing and local-stock alerts.
  3. Build the Mobile-First (375px) Context Switcher UI, allowing owners to view aggregate data or drill down into specific locations.
  4. Develop the Agent-driven "Escalation Protocol" where a Location Manager can pass complex Action Cards to the global Owner's feed.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
