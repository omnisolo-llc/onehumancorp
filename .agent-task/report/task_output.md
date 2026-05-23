issue_title: "Implement Dynamic Yield Management & Pricing Engine"
issue_description: |
  # Title
  Dynamic Yield Management & Pricing Engine

  ## Problem Statement
  Small business owners frequently lose potential revenue due to static pricing and underutilized capacity. For example, Maya (the baker) may have unsold vegan cakes at the end of the day that will spoil, or Leo (the music tutor) may have unbooked lesson slots tomorrow. Managing flash sales, last-minute discounts, or demand-based surge pricing requires constant manual monitoring, complicated math, and separate marketing tools to notify customers. This manual burden prevents them from maximizing revenue. They need an invisible, AI-driven engine that automatically detects expiring inventory or unbooked capacity, adjusts prices based on predefined rules, and proactively reaches out to likely buyers via the Unified AI Inbox—all while requiring zero technical configuration.

  ## Research Report
  ### Competitive Landscape
  *   **Shopify / WooCommerce:** Offer discount code generators and sale price fields, but require manual intervention to set up "flash sales" or "last-minute deals". No native, autonomous yield management system exists out of the box for standard tiers.
  *   **Airlines & Hotels (e.g., Sabre, Amadeus):** Utilize advanced yield management algorithms, but these are enterprise-grade systems completely inaccessible to SMBs due to complexity and cost.
  *   **Wix / Squarespace Bookings:** Allow manual price adjustments for services, but lack dynamic pricing based on time-to-appointment or historical booking velocity.
  *   **Uber / Lyft (Surge Pricing):** High consumer familiarity with dynamic pricing based on demand, but SMBs lack the tools to implement this locally.

  ### The OHC Gap
  OneHumanCorp currently lacks a systemic, multi-tenant approach to yield management. While we have basic inventory and calendar syncing, there is no autonomous engine that acts as a "revenue manager." This gap prevents us from truly acting as a business partner. We need a capability that bridges the Inventory/Capacity Ledger with the Marketing/Sales AI Agents to trigger automated pricing interventions.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INVENTORY_ITEM : has
      TENANT ||--o{ CALENDAR_SLOT : has
      INVENTORY_ITEM ||--o{ YIELD_RULE : governed_by
      CALENDAR_SLOT ||--o{ YIELD_RULE : governed_by
      YIELD_RULE ||--o{ YIELD_EVENT : triggers
      YIELD_EVENT }|--|| AI_MARKETING_AGENT : notifies
      AI_MARKETING_AGENT ||--o{ CUSTOMER : targets
  ```

  ### Key Design Decisions
  1.  **Rule-Based Baseline with AI Optimization:** The system starts with simple, pre-configured rules (e.g., "If bakery item is 2 hours from expiration, reduce price by 50%") to guarantee safety and predictability, but allows the AI Operations Agent to suggest optimizations based on historical sales data.
  2.  **Invisible Multi-Tenancy:** The `YIELD_RULE` and `YIELD_EVENT` tables must be strictly partitioned by `tenant_id` to ensure absolute data isolation.
  3.  **Proactive Marketing Integration:** A yield event (e.g., price drop) is useless if nobody knows about it. The engine must natively publish events to the OHC Message Bus, which the AI Marketing Agent consumes to draft localized SMS/Email alerts for the business owner to approve with one tap.
  4.  **Zero-Configuration Defaults:** The system must come with industry-specific default rules upon onboarding (e.g., Bakers get perishable goods rules; Tutors get last-minute slot filling rules).

  ### UI/UX Flow (Mobile First - 375px)
  1.  **The Trigger (Push Notification):** Maya receives a notification: "3 Vegan Cakes expiring in 4 hours. Tap to clear inventory."
  2.  **The Action Screen:** A sleek, glassmorphic card appears.
      *   **Headline:** "Clear Expiring Inventory"
      *   **Context:** "You have 3 Vegan Cakes left today. We suggest dropping the price to $15 and notifying your top 20 past customers."
      *   **Action Button:** A large, prominent "Approve & Send" button.
      *   **Advanced Toggle:** A small link to "Edit Sale Details" (revealing specific discount percentages and targeting lists for advanced users).
  3.  **The Result:** A success animation plays. The Marketing Agent dispatches the messages, and the Storefront price is temporarily updated. Maya goes back to baking.

  ## Implementation Prompt
  **Role:** Backend / Full-Stack Implementer
  **Context:** We need to implement the Dynamic Yield Management & Pricing Engine. This system automatically monitors inventory levels and calendar capacity, triggering price adjustments and marketing actions when predefined thresholds are met (e.g., unsold goods near closing time, empty appointment slots tomorrow).
  **User Journey:** The user should receive a single-tap notification on their mobile device suggesting a flash sale or price drop, which, upon approval, updates the storefront and sends marketing messages.
  **Requirements:**
  *   Design the data entities for Yield Rules and Yield Events, ensuring strict tenant isolation.
  *   Implement a background worker (using the existing `src/server/queue.rs` or `src/server/scheduler.rs` infrastructure) that periodically evaluates inventory/capacity against the active rules.
  *   Upon rule triggering, the worker must publish an event that the AI Marketing Agent can consume to draft the user notification.
  *   Ensure all database schemas adhere to the Zero Trust & Security guidelines (multi-tenant boundaries).
  *   *Note:* Do not prescribe specific HTTP endpoints or SQL queries. Focus on the core business logic and integration with the KAIROS event mesh.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []