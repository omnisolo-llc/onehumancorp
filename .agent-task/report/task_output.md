issue_title: "Implement Autonomous Supplier Procurement Mesh"
issue_description: |
  **Full Research Report:**
  Existing e-commerce platforms (like Shopify and Wix) offer basic low-stock alerts, but they place the burden of procurement on the business owner. This requires switching contexts to clunky B2B portals or manually emailing vendors, leading to stockouts and lost revenue.

  **Findings:**
  - Small businesses (e.g., Maya the baker, Priya the boutique owner) need proactive, AI-driven procurement.
  - The process must be manageable from a mobile device using a 1-tap approval flow.
  - OHC can bridge the gap between inventory ledgers and external suppliers by deploying an AI Operations Agent to monitor stock burn rates.

  **Proposed Next Steps:**
  Build the Autonomous Supplier Procurement Mesh.

  # [Architecture] Autonomous Supplier Procurement Mesh

  ## Problem Statement
  For small business owners like Priya (Boutique) and Maya (Baker), running out of raw materials or physical inventory is catastrophic. Currently, inventory management is entirely reactive: they notice they are out of flour or a specific t-shirt size, manually find their supplier, generate a purchase order via email or a clunky B2B portal, and arrange payment. This disjointed, manual process leads to stockouts, delayed orders, and lost revenue. They need a proactive, mobile-first system where AI automatically predicts stockouts, negotiates with integrated suppliers, and prepares a 1-tap reorder approval, ensuring they never run out of the goods they need to operate.

  ## Research Report
  *   **Current Capabilities:** Existing e-commerce platforms (like Shopify and Wix) have basic "low stock alerts" but leave the actual procurement and purchasing entirely up to the user via manual external processes.
  *   **Competitor Analysis:**
      *   *Shopify:* Has basic inventory tracking and purchase orders, but lacks an intelligent agent to proactively draft and negotiate orders based on predictive usage.
      *   *Wix / Squarespace:* Extremely rudimentary inventory management. No native procurement.
      *   *Dedicated B2B Tools (e.g., TradeGecko/QuickBooks Commerce):* Too complex for micro-businesses, requiring desktop-heavy management and manual data entry.
  *   **Gap Identified:** A unified procurement mesh that bridges the gap between OHC’s internal inventory ledger and external suppliers. This mesh allows the Operations AI Agent to monitor burn rates, draft purchase orders, and present a simple 1-tap approval to the owner before stockouts occur.
  *   **Strategic Advantage:** By turning procurement from a manual chore into an autonomous, AI-driven process with single-tap approvals, OHC saves business owners hours per week and guarantees operational continuity.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ INVENTORY_LEDGER : manages
      INVENTORY_LEDGER ||--o{ STOCK_ITEM : contains
      STOCK_ITEM ||--o{ PROCUREMENT_RULE : defines
      TENANT ||--o{ SUPPLIER : connects_to
      SUPPLIER ||--o{ PURCHASE_ORDER : receives
      PURCHASE_ORDER ||--|{ PO_LINE_ITEM : includes
      PO_LINE_ITEM }|--|| STOCK_ITEM : references

      TENANT {
          string id PK
          string name
      }
      STOCK_ITEM {
          string id PK
          string tenant_id FK
          string name
          int current_quantity
          int reorder_threshold
      }
      SUPPLIER {
          string id PK
          string tenant_id FK
          string name
          string contact_method "Email | API | WhatsApp"
      }
      PURCHASE_ORDER {
          string id PK
          string tenant_id FK
          string supplier_id FK
          string status "Draft | Approved | Sent | Fulfilled"
          decimal estimated_total
      }
      PROCUREMENT_RULE {
          string id PK
          string stock_item_id FK
          int target_restock_quantity
      }
  ```

  ```mermaid
  sequenceDiagram
      participant OHA as Operations AI Agent
      participant Mesh as Procurement Mesh
      participant Owner as Mobile App (Owner)
      participant Supplier as External Supplier

      OHA->>Mesh: Detect stock item < reorder_threshold
      Mesh->>Mesh: Generate Draft Purchase Order
      Mesh->>OHA: Request Approval
      OHA->>Owner: Push: "Low stock on Flour. Approve $50 reorder?"
      Owner->>OHA: 1-Tap Approve (Lock Screen)
      OHA->>Mesh: Update PO Status -> Approved
      Mesh->>Supplier: Dispatch PO via Supplier's preferred method
      Supplier-->>Mesh: Confirm Order & Delivery Date
      Mesh->>OHA: Notify Owner of delivery schedule
  ```

  ### UI Wireframes & Screen Flow (375px Mobile-First)
  1.  **Proactive Alert:** A standard mobile push notification: "⚠️ Running low on Medium White T-Shirts. Tap to review reorder."
  2.  **1-Tap Procurement Card (Half-Sheet Modal):**
      *   **Top:** Context: "You have 2 left. Based on current sales, you will run out by Thursday."
      *   **Middle:** Draft Purchase Order summarizing supplier, item, quantity, and estimated cost (e.g., "Supplier: Alpha Apparel. 20x Med White T-Shirts. Total: $100").
      *   **Bottom:** Large primary button: "Approve & Pay". Small secondary button: "Edit Order".
  3.  **Edit Order Flow (Conversational):** If the user taps "Edit Order", a chat interface opens. The owner can say, "Add 10 Large black shirts too," and the AI instantly updates the PO card. No manual spreadsheets.

  ### Key Design Decisions & Integrity
  *   **Zero-Trust Integration:** Suppliers and procurement rules are strictly isolated per `tenant_id`.
  *   **Abstracted Complexity:** The business owner does not see "Purchase Orders" or "SKUs" unless they enter an "Advanced Mode". They simply see "Restock Flour".
  *   **Omnichannel Dispatch:** The Procurement Mesh supports multiple dispatch protocols, allowing the AI to email a PDF PO to a traditional supplier, or send a WhatsApp message to an informal vendor.
  *   **Optimistic Approvals:** Approving a PO on mobile updates the UI instantly, queuing the dispatch process in the background.

  ### AI Agent Integration Points
  *   **Operations Agent (The Manager):** Continuously monitors the `INVENTORY_LEDGER` against burn rates. Triggers the creation of draft POs when thresholds are breached.
  *   **Finance Agent (The Accountant):** Verifies that the tenant has sufficient cash flow or credit to approve the purchase order before presenting it to the owner.

  ### Mobile UX Flow
  *   The owner receives an alert, taps it to view the translucent glass modal with the PO details.
  *   The owner taps "Approve". The UI instantly reflects the "Ordered" status.
  *   The system dispatches the order to the supplier behind the scenes.
  *   When the supplier confirms, a silent update adjusts the "Expected Delivery" date in the owner's dashboard.

  ## Implementation Prompt
  **Task for Implementer:** Build the Autonomous Supplier Procurement Mesh.

  **User Journey (CUJ):**
  1. The AI Operations Agent detects a low inventory level for a specific item.
  2. The system automatically drafts a Purchase Order for the item's primary supplier based on defined procurement rules.
  3. A push notification is sent to the business owner requesting approval.
  4. The owner approves the PO with a single tap.
  5. The system dispatches the PO to the supplier via their preferred contact method.

  **Acceptance Criteria:**
  *   Implement the core data models for Suppliers, Procurement Rules, and Purchase Orders.
  *   Create a mesh component that can generate draft POs based on inventory thresholds.
  *   Build an abstraction layer for dispatching POs (e.g., via Email, SMS, or mock API).
  *   Ensure the approval workflow is designed for optimistic UI updates.
  *   Enforce strict multi-tenant data isolation.
  *   Do NOT prescribe specific database technologies; focus on the robust state transitions of the Purchase Order.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []