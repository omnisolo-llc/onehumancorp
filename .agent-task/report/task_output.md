issue_title: "Implement Autonomous Supply Chain & Vendor Mesh"
issue_description: |
  # Title: Autonomous Supply Chain & Vendor Mesh

  ## Problem Statement
  While OneHumanCorp (OHC) excels at downstream sales (B2C) for small businesses, product- and food-based personas (like Maya the baker, Priya the boutique owner, and Fatima the food cart operator) struggle immensely with upstream operations. Managing raw materials, tracking Bills of Materials (BOMs), predicting when stock will run out, and manually negotiating or reordering from wholesale vendors is a highly manual, error-prone process. This leads to stockouts (lost revenue), over-ordering (wasted capital), and massive time sinks. They need an invisible supply chain manager that autonomously tracks raw material depletion based on sales velocity and automatically triggers vendor reorders before a stockout occurs.

  ## Research Report
  *   **Current Capabilities:** OHC has basic storefront and inventory models, but it focuses purely on finished goods (SKUs) rather than the raw materials required to produce those goods.
  *   **Competitor Analysis:**
      *   *Shopify/Wix:* Focus entirely on finished goods inventory. Complex inventory management (BOMs, vendor POs) requires expensive 3rd-party apps (like TradeGecko/QuickBooks Commerce) which are far too complex for micro-merchants.
      *   *Square:* Offers basic vendor management and low-stock alerts, but relies entirely on the user to manually track components and generate Purchase Orders.
  *   **Gap Identified:** A unified **Autonomous Supply Chain & Vendor Mesh** that connects sales velocity directly to raw material depletion (via dynamic BOMs) and utilizes the AI Operations & Finance Agents to proactively manage vendor relationships and draft Purchase Orders.
  *   **Strategic Advantage:** By solving the "upstream" problem, OHC becomes the true operating system for the business. Maya no longer has to remember to order flour; the OHC agent texts her: "Flour is running low based on your cake orders. I've drafted an order for 50lbs from Acme Supply for $45. Approve?"

  ## Design Doc

  ### Business Journey Mapping (Maya the Baker)
  1.  **Acquisition/Setup:** Maya tells the onboarding agent: "I make a vegan chocolate cake. It takes 2 cups of special cocoa powder from VendorX." The AI autonomously generates a Bill of Materials (BOM).
  2.  **Sales Velocity (Activation):** Maya sells 10 vegan cakes over the weekend via the OHC storefront.
  3.  **Depletion Tracking:** The *Operations Agent* automatically deducts 20 cups of cocoa powder from the internal ledger.
  4.  **Autonomous Reordering:** The agent detects the stock will fall below the safety threshold within 3 days. It queries VendorX's catalog in the Vendor Mesh.
  5.  **1-Tap Approval:** Maya receives a push notification on her lock screen: "You will run out of cocoa by Thursday. Approve $45 PO to VendorX?" Maya taps "Approve".
  6.  **Fulfillment:** The *Finance Agent* executes the payment via the Treasury Wallet, and an email/API request is sent to VendorX.
  7.  **Retention:** The agent logs the expected delivery date and notifies Maya when it should arrive, completing the cycle.

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ FINISHED_GOOD : sells
      TENANT ||--o{ RAW_MATERIAL : stocks
      TENANT ||--o{ VENDOR : buys_from
      FINISHED_GOOD ||--|{ BOM_ITEM : requires
      BOM_ITEM }|--|| RAW_MATERIAL : references
      VENDOR ||--o{ PURCHASE_ORDER : receives
      PURCHASE_ORDER ||--|{ PO_LINE_ITEM : contains
      PO_LINE_ITEM }|--|| RAW_MATERIAL : specifies
      SALES_EVENT ||--o{ DEPLETION_LOG : triggers
      DEPLETION_LOG }|--|| RAW_MATERIAL : reduces

      FINISHED_GOOD {
          string id
          string name
      }
      RAW_MATERIAL {
          string id
          string name
          int current_quantity
          int reorder_threshold
      }
      BOM_ITEM {
          string finished_good_id
          string raw_material_id
          float quantity_required
      }
      VENDOR {
          string id
          string contact_info
      }
      PURCHASE_ORDER {
          string id
          string status "Draft | Sent | Paid | Received"
          float total_cost
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Storefront as OHC Storefront
      participant Ledger as Inventory Ledger
      participant OpsAgent as Operations Agent (AI)
      participant Maya as Owner (Mobile)
      participant VendorGateway as Vendor Mesh Gateway

      Storefront->>Ledger: Event: 1 Vegan Cake Sold
      Ledger->>Ledger: Deduct 1 Cake (Finished)
      Ledger->>Ledger: Deduct 2 Cups Cocoa (BOM)
      Ledger->>OpsAgent: Event: Cocoa below threshold
      OpsAgent->>OpsAgent: Draft Purchase Order based on velocity
      OpsAgent->>Maya: Push: "Low Cocoa. Approve $45 PO?"
      Maya->>OpsAgent: 1-Tap Approve (Mobile)
      OpsAgent->>VendorGateway: Submit PO / Dispatch Payment
      VendorGateway-->>OpsAgent: PO Confirmed
      OpsAgent->>Maya: "Ordered. Arriving Wednesday."
  ```

  ### Mobile UX Flow (375px Viewport)
  1.  **Notification:** Clean, plain-language push notification focusing on the business outcome, not the technical inventory terms.
  2.  **Review Screen (Half-Sheet Modal):**
      *   **Context:** "Based on your recent sales, you need more Cocoa Powder by Thursday."
      *   **Details:** "VendorX • 50lbs • $45.00"
      *   **Action:** Large, thumb-friendly "Approve & Pay" button. Secondary "Edit Amount" ghost button.
  3.  **Inventory Tab (Simplification):** The inventory screen uses a traffic-light system (Green/Yellow/Red dots) rather than complex spreadsheets. Tapping a "Red" item immediately brings up the AI's reorder recommendation.

  ### AI Integration & Security Integrity
  *   **Operations Agent (The Manager):** Continuously monitors the `InventoryLedger` and calculates run-rates based on historical sales data to optimize reorder thresholds dynamically.
  *   **Finance Agent (The Auditor):** Ensures the tenant has sufficient funds in the OHC Wallet before proposing a PO and manages the outbound payment to the vendor.
  *   **Zero-Trust Isolation:** All BOMs, Vendor lists, and POs are strictly partitioned by `tenant_id`. The Operations Agent evaluates inventory events in memory isolated strictly to that tenant.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Implement the Autonomous Supply Chain & Vendor Mesh.
  The system must introduce a dual-layer inventory model supporting both `FinishedGoods` and `RawMaterials`, linked by a `BillOfMaterials` (BOM) construct. Implement an event-driven depletion engine that automatically reduces raw material quantities when a finished good is sold. Build the AI Operations Agent background task that monitors these levels against safety thresholds and drafts a `PurchaseOrder` for the associated `Vendor`.
  Ensure the mobile UI allows the business owner to review and approve the drafted PO with a single tap. The system must enforce strict multi-tenant data isolation. Do not prescribe specific database schemas; design the underlying tables and API structure to fulfill these requirements efficiently.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
