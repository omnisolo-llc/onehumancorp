issue_title: "Autonomous Dynamic Pricing & Waste Reduction Engine"
issue_description: |
  # Title: Autonomous Dynamic Pricing & Waste Reduction Engine

  ## Problem Statement
  Small business owners, particularly in food & beverage (like Fatima, the food cart operator) and service industries (like Carlos, the handyman), face massive margin compression due to unsold perishable inventory and unoptimized peak-hour scheduling. They cannot manually monitor demand spikes or impending expiration times to adjust prices dynamically. For Fatima, unsold food at the end of the day is a total loss. For Carlos, accepting a low-margin job during peak storm season means turning away higher-paying emergency repairs. They need an invisible agent that autonomously adjusts pricing to clear inventory and maximize peak yield without requiring complex dashboard management.

  ## Research Report
  *   **Competitor Analysis:**
      *   *Shopify/Wix:* Lack native, intelligent dynamic pricing based on time or inventory velocity. They rely on manual discount codes or clunky third-party apps that require rule-setting (e.g., "if time = 5pm, discount 20%").
      *   *Uber/Airlines:* Utilize advanced surge pricing, but these enterprise models are inaccessible to SMBs.
      *   *Too Good To Go:* Helps clear food waste but acts as a middleman, taking a 25%+ cut and removing the customer relationship from the business owner.
  *   **Pain Points:**
      *   "I throw away $50 worth of food every night because I don't have time to text my customers a discount."
      *   "I get flooded with calls when it snows, but I'm charging my regular summer rates because updating my site is too hard."
  *   **The OHC Advantage:** By leveraging the Operations Agent (to monitor inventory velocity and expiration) and the Sales Agent (to analyze booking density), OHC can autonomously generate and apply targeted, time-sensitive pricing adjustments.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant IM as Inventory/Capacity Mesh
      participant OAgent as Operations Agent
      participant SAgent as Sales Agent
      participant PricingEngine as Dynamic Pricing Engine
      participant App as Mobile UI (Owner)
      participant Storefront as Edge Cache Storefront

      IM->>OAgent: Event: Perishable Inventory > 50% & Time = T-2hrs to Close
      OAgent->>PricingEngine: Trigger Waste Reduction Protocol
      PricingEngine->>SAgent: Calculate optimal discount to clear stock
      SAgent-->>PricingEngine: Recommend 30% Markdown + Push Notification
      PricingEngine->>App: Queue Action: "Clear Remaining Stock?" (1-tap approve)
      App-->>PricingEngine: Owner taps 'Approve'
      PricingEngine->>Storefront: Update Price instantly (Invalidate Cache)
      PricingEngine->>SAgent: Trigger SMS/Push to local past customers
  ```

  ### Mobile UX Flow (375px)
  1.  **Notification (Lock Screen):** "Hey Fatima, you have 10 Halal Platters left. Tap to discount 30% and notify your regulars."
  2.  **Action Card (Dashboard):** Translucent glass card showing the suggested discount, the predicted revenue recovered ($45), and a prominent, thumb-friendly "Approve & Send" button.
  3.  **Advanced Settings (Hidden):** A toggle to "Always Auto-Approve" for items within 1 hour of closing.

  ### Key Design Decisions
  *   **Opt-In Automation:** To build trust, the engine initially requires 1-tap approval via an Action Card. Once the owner trusts the agent, they can toggle "Auto-Pilot."
  *   **Zero-Config Rules:** The owner never sets a rule. The AI calculates the discount based on historical clearance rates and current weather/time data.
  *   **Multi-tenant Isolation:** Pricing signals and local demand models are strictly partitioned per tenant to prevent data leakage between competing businesses.

  ## Implementation Prompt
  Implement the Autonomous Dynamic Pricing & Waste Reduction Engine.

  **Core User Journey (CUJ):**
  1. The system must monitor the `InventoryMesh` for items tagged as perishable or time-sensitive (e.g., daily specials, unfilled calendar slots for today).
  2. When the current time approaches the business's closing time (or slot start time) and inventory remains high, the system must trigger a dynamic pricing event.
  3. The engine must generate an Action Card requesting 1-tap approval from the owner to apply a calculated discount and optionally notify recent customers.
  4. Upon approval, the system must update the item price in the multi-tenant edge storefront instantly.

  **Acceptance Criteria:**
  *   Do not require the user to configure complex "If/Then" pricing rules.
  *   Must enforce strict tenant isolation for all inventory and pricing operations.
  *   Must pass the "Grandmother Test": the approval UI must be understandable by a non-technical user in under 10 seconds.
  *   Ensure all backend pricing adjustments are eventual-consistency safe but feel instantaneous on the mobile frontend via Optimistic UI.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
