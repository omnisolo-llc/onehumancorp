issue_title: "Centralized Inventory POS & Tap-to-Pay Integration"
issue_description: |
  **Problem Statement:**
  Physical business owners like Priya (boutique owner) or Carlos (handyman) struggle with disjointed operations where in-store or on-site sales aren't seamlessly reflected in their online inventory. They need a unified Point of Sale (POS) system integrated directly into the OneHumanCorp assistant, enabling mobile tap-to-pay functionality (via phone NFC) that automatically deducts inventory and synchronizes transaction records in one central owner feed.

  **Research Report:**
  Analysis of leading platforms (Shopify, Square, Stripe, Wix) reveals that tight POS integration directly on mobile devices significantly increases merchant retention and reduces operational friction. The absence of a unified Tap-to-Pay and inventory management system forces owners to rely on multiple apps, breaking the "One Human Corp" promise of a centralized work assistant. Introducing a mobile-first, NFC-based Tap-to-Pay flow tied to the core `InventoryLedger` will ensure that an offline sale is instantly reflected online.

  **Design Doc:**
  - **Architecture Diagram:**
    ```mermaid
    erDiagram
        Tenant {
            string id
            string name
        }
        InventoryItem {
            string id
            string tenant_id
            string name
            int quantity
        }
        TerminalSession {
            string id
            string tenant_id
            string status
            float amount
        }
        LedgerTransaction {
            string id
            string tenant_id
            string item_id
            string session_id
            float amount
        }
        Tenant ||--o{ InventoryItem : owns
        Tenant ||--o{ TerminalSession : processes
        TerminalSession ||--o{ LedgerTransaction : triggers
        InventoryItem ||--o{ LedgerTransaction : logs
    ```
  - **Mobile UX Flow (375px Viewport):**
    1. The "Sell" tab on mobile displays quick-add product variants with large tap targets (minimum 44x44px).
    2. Owner selects items, and a prominent "Tap-to-Pay" button is shown.
    3. The native OS tap-to-pay/NFC sheet handles the transaction securely.
    4. Upon success, the system renders a clean summary view and automatically generates a transaction record.
  - **AI Agent Integration Points:**
    - **Finance & Decision Assistant:** Monitors daily offline vs. online sales. Flags discrepancies and suggests inventory replenishment when stock levels dip.
    - **Operations Assistant:** Automatically logs the transaction in the Owner Work Feed as completed work.
  - **Key Design Decisions:**
    - Strict row-level multi-tenancy on all inventory and terminal models via `tenant_id`.
    - Zero-trust security model for terminal endpoints to ensure cryptographic verification of NFC payment responses.
    - Offline-tolerant reads for product variants on mobile, ensuring the quick-add screen is responsive even in poor network conditions (e.g., at a food cart).

  **Implementation Prompt:**
  As the implementer agent, build the `TerminalSession` and `InventoryLedger` data models, REST endpoints, and the primary mobile POS checkout view. Verify that an in-store checkout correctly deducts from a unified inventory count. Ensure zero-trust tenant isolation on all tap-to-pay endpoints and robust mobile-first UX with 44x44px tap targets. Include comprehensive E2E tests for the POS checkout flow. Do not mock network calls internally; use the provided local adapters for external payment APIs.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
