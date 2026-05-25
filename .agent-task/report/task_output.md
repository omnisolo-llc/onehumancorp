issue_title: "Implement Autonomous Yield & Dynamic Pricing Engine"
issue_description: |
  # Autonomous Yield & Dynamic Pricing Engine

  ## Problem Statement
  For small business owners like Maya (who gets flooded with weekend rush orders for custom cakes) and Fatima (who needs to sell out her halal food pre-orders before the end of the day), setting and adjusting prices manually is a constant chore. Small business owners often leave revenue on the table during peak demand or face inventory spoilage when demand is low. Existing tools like Shopify or Wix require manual discount codes, complex rules engines, or expensive third-party plugins. Our personas need an intelligent, invisible yield management system that automatically optimizes pricing based on real-time capacity, inventory, and demand signals—maximizing their revenue without requiring them to lift a finger.

  ## Research Report
  **Market Landscape:**
  - **Shopify/Wix:** Rely on static discount codes or manual bulk price adjustments. Dynamic pricing apps exist (e.g., Prisync, Wiser) but cost $50-$200+/month and are designed for large e-commerce catalogs, not service availability or daily food inventory.
  - **Square/Toast:** Basic time-based pricing (e.g., happy hour) exists, but true algorithmic yield management requires enterprise integrations.
  - **Airlines/Hotels:** Utilize advanced yield management to maximize revenue per available seat/room.

  **The OHC Opportunity:**
  Bring enterprise-grade yield management to the SMB level via the KAIROS AI agents. By tying into the existing `Capacity and Inventory Ledger`, the `Finance Department` AI agent can autonomously adjust pricing in real-time. For example:
  - Automatically adding a 20% "Rush Fee" to Maya's bookings if her calendar is 90% full for the weekend.
  - Automatically dropping Fatima's pre-order meals by 30% at 3:00 PM if she still has 20 portions left.
  - Adjusting Leo's tutoring rates based on high-demand time slots (e.g., after school hours).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Checkout / Booking UI] --> B(Yield Pricing API)
      B --> C{Capacity & Inventory Ledger}
      C -->|Real-time state| D[Finance AI Agent]
      D -->|Evaluates rules & market demand| B
      E[Business Owner OHC App] -->|Toggles Auto-Yield On/Off| D
      B --> F[Invisible Multi-Party Ledger / Checkout]
  ```

  ### UI Wireframes & Mobile UX Flow
  **375px Mobile Viewport UX:**
  1. **Settings / Product View:** A simple, macOS-glass styled card inside the product/service settings titled "AI Revenue Maximizer".
  2. **The Switch:** A single robust toggle switch (Ubiquiti UniFi style) labeled "Enable Auto-Yield Pricing".
  3. **Configuration (Hidden behind "Advanced"):**
     - Minimum acceptable price (e.g., "$10 floor for remaining food")
     - Maximum peak price (e.g., "+50% for rush orders")
  4. **Customer Facing:** The customer sees a small, elegant badge during checkout (e.g., "⚡ Rush Pricing applied" or "📉 End of day deal!").

  ### AI Agent Integration Points
  - **Finance Department Agent:** Monitors the inventory/capacity levels in real-time. If a predefined condition is met (e.g., < 2 hours left in business day and inventory > 20%), it recalculates the localized price cache.
  - **Marketing Department Agent:** Can automatically generate and push a social media post or SMS notification (via Ayrshare/Twilio integrations) announcing the temporary price drop to drive instant demand.

  ### Key Design Decisions & Why
  - **Opt-in Simplicity:** The feature is a simple toggle. Small business owners do not want to configure complex elasticity curves. They just want "more revenue" or "less waste".
  - **Safety Bounds:** We must implement hard floor and ceiling boundaries to prevent the AI from pricing items at $0.01 or $10,000, ensuring business safety and protecting the owner's reputation.
  - **Real-time Evaluation at Edge:** Pricing must be cached at the edge to ensure high-performance loading of the storefront, but invalidated instantly when the Finance Agent triggers a yield update.

  ## Implementation Prompt
  **Context:** We need to implement the Autonomous Yield & Dynamic Pricing Engine for OneHumanCorp.
  **Objective:** Build the backend logic and the frontend mobile-first UI for the "AI Revenue Maximizer".
  **Acceptance Criteria:**
  1. A business owner can toggle "Auto-Yield" on a product/service via the mobile UI.
  2. The UI includes fields for "Minimum Price" and "Maximum Price" bounds.
  3. The Finance AI Agent can securely adjust the active price of an item based on its current inventory level or booking capacity without manual intervention.
  4. The storefront UI reflects the dynamic price instantly.
  5. All design elements must follow the macOS-style Translucent Glass and UniFi modular card layout paradigms. Ensure the "grandmother test" is passed—no complex configuration by default.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_estimated_scope: Medium
