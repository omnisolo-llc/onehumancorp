issue_title: "Invisible Supply Chain & Auto-Procurement Engine"
issue_description: |
  # Invisible Supply Chain & Auto-Procurement Engine

  ## Problem Statement
  Small business owners (like Maya the baker or Priya the boutique owner) lack the time and expertise to manually count raw materials, forecast demand based on upcoming orders or seasonal trends, and draft purchase orders to multiple different suppliers. This manual overhead often results in unexpected stockouts, lost revenue, and stressful last-minute grocery store runs.

  ## Research Report

  ### Current Market Landscape
  *   **Shopify:** Relies heavily on third-party apps (like Stocky) for purchase orders and demand forecasting. Native features are limited to simple low-stock alerts that require manual configuration of thresholds.
  *   **Wix & Squarespace:** Basic inventory counting. They do not handle Bill of Materials (BOM) or supplier relationship management natively. Reordering is an entirely manual process done outside the platform.
  *   **Square:** Better at POS inventory, but still requires the merchant to manually identify what to order and generate the PO.

  ### The OHC "Unfair Advantage"
  OneHumanCorp can solve this invisibly by connecting our Universal Capacity and Inventory Mesh with the Autonomous Operations Agent. By understanding a product's Bill of Materials and observing current sales velocity, the AI can preemptively draft a Purchase Order, find the best prices among known suppliers, and present a simple 1-tap "Approve Order" notification to the business owner before stock runs out.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      subgraph OHC Platform
          Sales[Sales & Booking Engine] --> |Order Data| InventoryMesh[Universal Inventory Mesh]
          InventoryMesh --> |Depletes| Stock[Stock Ledger]
          Stock --> |Low Stock Trigger| OpsAgent[Operations AI Agent]
          Sales --> |Sales Velocity| DemandForecaster[Demand Forecasting Engine]
          DemandForecaster --> |Predictions| OpsAgent
          SupplierDB[Supplier & Catalog DB] --> OpsAgent

          OpsAgent --> |Drafts PO| POEngine[Purchase Order Engine]
          POEngine --> |Push Notification| MobileApp[Mobile App UI]
          MobileApp --> |1-Tap Approve| FinanceAgent[Finance AI Agent]
          FinanceAgent --> |Issues Payment & Sends PO| ExternalSupplier[External Supplier API/Email]
      end
  ```

  ### Mobile-first UI Wireframes (375px)

  **Screen 1: The Auto-Procurement Nudge (Dashboard)**
  *   **Component:** Glassmorphism card at the top of the Home Dashboard.
  *   **Content:** "Low stock predicted for Vegan Cake ingredients. Drafted order to Restaurant Depot for $45.20."
  *   **Actions:** [Review & Approve] (Primary Button), [Dismiss] (Ghost Button).

  **Screen 2: Purchase Order Review**
  *   **Header:** "Supplier Order - Restaurant Depot"
  *   **Body:**
      *   List of items: 50lb Flour ($20), 2gal Almond Milk ($15.20), etc.
      *   Contextual AI Note: "You have 12 cake orders this weekend. This order covers those plus standard buffer."
  *   **Actions:** [Edit Quantities] (Secondary), [Approve & Pay $45.20] (Primary).

  ### AI Agent Integration Points
  *   **Operations Department:** Monitors the `Universal Inventory Mesh` and `Demand Forecasting Engine`. When a threshold (based on lead time + buffer) is hit, it queries the `Supplier DB` to construct a drafted Purchase Order.
  *   **Finance Department:** Once the business owner approves the PO, the Finance Agent handles securely transmitting the payment (using the `Invisible Multi-Party Split Payments Ledger`) and sending the finalized PO to the supplier via email or API.

  ### Multi-Tenant Isolation
  *   Supplier data, Bill of Materials definitions, and Purchase Orders must be strictly partitioned by `organization_id`.
  *   The `TenantRegistry` will ensure that the Operations Agent only queries historical sales velocity and inventory levels for the specific tenant it is currently acting on behalf of.

  ## Implementation Prompt

  **To the Implementer:**

  Design and implement the data models and core logic for the Invisible Supply Chain & Auto-Procurement Engine.

  **Core User Journey (CUJ):**
  1. The system understands that "Product A" is made of "Raw Material X" and "Raw Material Y" (Bill of Materials).
  2. As "Product A" is sold, the inventory of X and Y is virtually depleted.
  3. When the projected inventory of X falls below a dynamic threshold (calculated from sales velocity and supplier lead time), a background job drafts a Purchase Order for the default supplier.
  4. The business owner receives a mobile push notification and can approve the drafted PO with one tap.

  **Acceptance Criteria:**
  *   Create the entity relationships capable of representing a Bill of Materials (BOM) linking sellable products to raw materials.
  *   Create the data structures for Suppliers and drafted/approved Purchase Orders.
  *   Implement the background queue worker (using our high-performance background job queue) that scans for low-stock anomalies and triggers the Operations Agent.
  *   Ensure all database tables and queries enforce strict multi-tenant isolation via `organization_id`.
  *   Expose internal APIs for the mobile UI to fetch pending drafted POs and approve them.

  *Note: Do not prescribe specific SQL DDL, API endpoints, or function signatures in this prompt. Focus on the required business capabilities and invariants.*

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []