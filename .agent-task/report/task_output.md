issue_title: "Implement Autonomous Inventory Replenishment Engine"
issue_description: |
  # Autonomous Inventory Replenishment Engine

  ## Problem Statement
  Small business owners like Priya (the boutique owner) and Fatima (the food cart operator) struggle with inventory management. They often run out of stock of popular items, leading to lost sales and disappointed customers. Conversely, they may over-order slow-moving items, tying up valuable capital. Manually tracking inventory levels, forecasting demand, and reordering from suppliers is time-consuming, complex, and prone to human error. They need an intelligent, invisible system that anticipates demand, automatically generates purchase orders, and ensures they always have the right amount of stock at the right time, without requiring manual intervention.

  ## Research Report
  *   **Shopify:** Offers basic inventory tracking and low-stock alerts. However, automated reordering requires third-party apps (e.g., Stocky) which add complexity and cost. Demand forecasting is rudimentary and often requires manual analysis or expensive add-ons.
  *   **Wix:** Provides simple inventory management but lacks sophisticated forecasting and automated replenishment capabilities. It primarily relies on manual updates and simple alerts.
  *   **Squarespace / GoDaddy:** Inventory management is very basic, primarily focused on displaying "sold out" on the storefront rather than managing the supply chain.
  *   **OneHumanCorp (OHC) Differentiation - "Predictive Autonomy":** OHC's Operations Department (The Manager) goes beyond simple tracking. It uses historical sales data, seasonal trends, and even external factors (like local events) to predict future demand. When stock drops below the intelligently calculated reorder point, the agent automatically drafts a purchase order to the preferred supplier, seeking the owner's 1-tap approval.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Sales/Orders Data] --> B(Demand Forecasting Engine);
      C[Historical Trends] --> B;
      D[External Factors e.g., Seasonality] --> B;
      B --> E{Inventory Level < Reorder Point?};
      E -- Yes --> F[Generate Purchase Order];
      E -- No --> G[Monitor Levels];
      F --> H[Operations Agent: The Manager];
      H --> I{Approval Required?};
      I -- Yes --> J[Notify Owner via Mobile App - 1 Tap Approval];
      I -- No --> K[Send PO to Supplier via Email/API];
      J -- Approved --> K;
      J -- Rejected --> L[Log Rejection / Adjust Parameters];
      K --> M[Update Expected Inventory];
      M --> N[Notify Owner of Expected Delivery];
  ```

  ### Mobile UX Flow (375px First)
  1.  **Dashboard Alert:** Priya logs into the OHC app. A prominent, translucent glass card on the dashboard states: "Stock Alert: Red Summer Dress (Size M) is running low based on recent sales."
  2.  **1-Tap Action:** The card contains a "Review Order" button. Tapping it opens a bottom sheet.
  3.  **Order Summary:** The bottom sheet displays the drafted purchase order: "Supplier: FashionWholesale Ltd. Item: Red Summer Dress (Size M). Quantity: 50. Estimated Cost: $500." The Operations Agent notes: "Based on current sales velocity, you will run out in 3 days. Reordering 50 units will cover expected demand for the next month."
  4.  **Approval:** A large, easily tappable "Approve & Send Order" button is present. Tapping it triggers a success micro-animation, and the bottom sheet dismisses. The PO is sent.

  ### Key Design Decisions
  *   **Intelligent Reorder Points:** The system dynamically calculates reorder points based on current sales velocity, lead time from the supplier, and desired safety stock.
  *   **1-Tap Approval:** Keeping the owner in the loop but removing the friction of drafting the order. The AI calculates what is needed and who to order it from.
  *   **Supplier Integration:** The system will store supplier contact information and preferred communication methods.
  *   **Cost Visibility:** The drafted PO clearly shows the estimated cost, integrating with the Finance Department.

  ### AI Agent Integration Points
  *   **Operations Agent (The Manager):** The core orchestrator. Monitors inventory, runs forecasting models, and drafts purchase orders.
  *   **Business Advisory Agent (The Advisor):** Analyzes replenishment efficiency.
  *   **Finance Agent (The Accountant):** Verifies cash flow before a PO is finalized.

  ## Implementation Prompt
  **Objective:** Implement the backend logic and the mobile UI for the Autonomous Inventory Replenishment Engine.

  **Persona:** Priya (boutique owner) needs the system to anticipate when her popular items will run out and draft a purchase order for her approval.

  **Critical User Journey (CUJ):**
  1. The background forecasting engine identifies an item whose projected inventory will fall below the dynamic safety stock level within the supplier's lead time.
  2. The Operations Agent drafts a Purchase Order (PO) for the required replenishment quantity.
  3. The owner opens the mobile app (simulated 375px viewport).
  4. The dashboard displays an actionable "Low Stock Alert" card.
  5. The owner taps the card to review the drafted PO.
  6. The owner taps "Approve Order."
  7. The system marks the PO as "Sent" and updates expected inventory levels.

  **Acceptance Criteria:**
  *   A background job mechanism must periodically check inventory levels against calculated reorder points.
  *   A new database entity `PurchaseOrder` must be created.
  *   The mobile UI must present a clear, actionable alert card on the dashboard.
  *   The UI must allow 1-tap approval of the drafted PO.
  *   Multi-tenant isolation MUST be strictly enforced.
  *   Include comprehensive unit tests and at least one Playwright E2E test.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
