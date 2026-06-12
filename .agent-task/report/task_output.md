issue_title: "Core Architecture: Native Tap-to-Pay & Unified POS Framework"
issue_description: |
  ## 1. Problem Statement
  Retail and physical-presence owners like Priya (Boutique Operator) and Fatima (Food Cart Operator) suffer from disjointed workflows when bridging online stores with in-person operations. Traditional e-commerce platforms (like Shopify or Wix) either require expensive proprietary POS hardware or force the owner to use a completely separate system (like Square) for in-store sales, which fractures inventory, customer records, and daily financial reporting. They need a unified architecture where a 375px mobile device can seamlessly act as a cash register and inventory manager, driven by OHC’s AI agents.

  ## 2. Research Report
  - **Market Context**: Square dominates physical micro-SMEs because of its accessibility, but struggles with advanced e-commerce. Shopify POS is robust but requires an expensive tier and proprietary hardware to work optimally. Wix relies on third-party POS integrations.
  - **The OHC Opportunity**: Apple and Android both now support native Tap-to-Pay directly on mobile devices (via NFC), eliminating the need for dongles or separate terminal hardware. By integrating Stripe Terminal SDK directly into the OHC Flutter shell, any owner's smartphone instantly becomes a unified POS.
  - **Competitor Gaps**: Existing POS systems do not have proactive Agent layers. If an item runs out of stock in-store, an agent does not automatically pause the online listing or draft a supplier re-order.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ TERMINAL_SESSION : manages
      TERMINAL_SESSION ||--o{ POS_ORDER : processes
      POS_ORDER }|--|| ORDER : inherits
      POS_ORDER }o--|| INVENTORY_LEDGER : depletes

      TENANT {
          uuid id PK
          string name
      }
      TERMINAL_SESSION {
          uuid id PK
          uuid tenant_id FK
          string device_os
          string status
      }
      POS_ORDER {
          uuid id PK
          uuid base_order_id FK
          uuid terminal_session_id FK
          string cashier_id
          string signature_receipt
      }
      ORDER {
          uuid id PK
          float total_amount
          string status
      }
      INVENTORY_LEDGER {
          uuid id PK
          uuid product_id
          int quantity
      }
  ```

  ### Data Model (PostgreSQL)
  - `TerminalSession`: Represents a physical device session linked to a specific Location/Tenant.
  - `POSOrder`: Inherits from standard Order but contains POS-specific metadata (cashier_id, device_id, signature_receipt).
  - `InventoryLedger`: Mutated in real-time. Emits events that the Operations Agent monitors to prevent online overselling while a physical customer is checking out.

  ### AI Agent Integration
  - **Finance Agent ("The Accountant")**: Automatically reconciles the daily physical till with Stripe payouts, summarizing cash vs. card for the owner at 6 PM.
  - **Operations Agent ("The Manager")**: Monitors physical POS sales velocity. If a boutique dress is selling out faster in-store than expected, it dynamically pauses the online variant and drafts a re-order task.

  ### Mobile UX Flow (375px)
  1. **Cashier View**: A high-contrast, large touch-target (min 44x44px) catalog grid. Priya taps items to add to the cart.
  2. **Payment Intent**: User taps "Charge". The app transitions to the native OS Tap-to-Pay (NFC) overlay.
  3. **Success & Receipt**: Clean confirmation screen with 1-tap SMS/Email receipt delivery, capturing the customer's info to build the unified CRM record.

  ## 4. Implementation Prompt
  **Feature Name**: Native Tap-to-Pay POS Module
  **Target Persona**: Priya (Boutique Operator)
  **Outcome**: Priya can use her smartphone to ring up in-store customers using native Tap-to-Pay. In-store inventory and online inventory are perfectly synced, and the Finance Agent provides a unified daily revenue summary.

  **Next Actions**:
  1. Define the multi-tenant PostgreSQL schema for `TerminalSession` and POS-specific order ledgers.
  2. Integrate the Stripe Terminal SDK into the Flutter client, implementing the Cashier UX (375px optimal) for building a cart and triggering the NFC payment sheet.
  3. Wire the backend event stream so that POS checkout events instantly update the `InventoryLedger` and trigger the Operations Agent for velocity monitoring.
  4. Build the E2E Playwright tests simulating the POS cart-building and checkout flow using Stripe's mock terminal readers.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
