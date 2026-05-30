issue_title: "[architecture] Omnichannel Sync Engine & Tap-to-Pay Capabilities"
issue_description: |
  # Problem Statement
  Priya, a boutique owner, operates both a physical storefront and an online store. When a physical sale occurs, she uses a Tap-to-Pay POS system. Currently, this POS system operates independently of the global online inventory database. As a result, when an item is sold in-store, its online inventory is not immediately decremented. This disconnect creates a high risk of double-selling limited-edition items and forces Priya to perform manual, stressful inventory synchronization—an error-prone process that undermines the platform's promise of invisible, automated operations. The business needs a unified system where offline and online sales seamlessly and instantly impact the same global inventory ledger, without the user having to manage the discrepancy.

  # Research Report
  Our platform audit reveals that the current architecture separates POS terminal sessions from the main inventory systems. Competitor platforms often require third-party integrations (Square, Shopify POS) that lack real-time synchronization out-of-the-box or require complex setups, leaving users confused.

  **Key Findings:**
  1. The existing mobile Tap-to-Pay logic is disconnected from the global `InventoryDB`. This creates an architecture gap where in-person purchases fail to trigger the required backend events.
  2. The mobile POS must function securely and reliably even when network conditions fluctuate, requiring an eventual consistency model while immediately locking or decrementing inventory on the local device.
  3. AI Agents (specifically the Operations Agent) should be leveraged to silently manage the synchronization and error-handling without notifying the merchant unless human intervention is explicitly required.

  # Design Doc

  ## Architecture Diagram

  ```mermaid
  graph TD
      subgraph Mobile POS App
          A[Tap-to-Pay SDK] --> B[Local CRDT Store]
          B --> C[Offline Sync Engine]
      end

      subgraph Cloud Platform
          C --> D[OHC API Gateway]
          D --> E[Transaction Event Bus]
          E --> F[Global Inventory DB]
          E --> G[Ledger / Finance DB]

          subgraph AI Agents
              E -.-> H[Operations Agent]
              H -.-> F
          end
      end
  ```

  ## Mobile UX Flow (375px First)
  1. **Dashboard view:** A clean list of available products with visual representations (cards).
  2. **Add to Cart:** Merchant taps items. A slide-up pane displays the cart total using Translucent Glass materials.
  3. **Checkout Selection:** Merchant taps 'Charge'. A bottom sheet presents 'Tap to Pay' or 'Cash'.
  4. **Payment Processing:** If 'Tap to Pay' is selected, the native OS NFC UI appears.
  5. **Post-Payment:** A success screen displays briefly. In the background, the sync engine fires an event to the cloud to decrement inventory immediately.

  ## Key Design Decisions
  - **Local-first Architecture:** To ensure that Priya and Fatima can operate even in low-connectivity areas, the POS uses a local CRDT (Conflict-free Replicated Data Type) store that instantly updates the UI, synchronizing with the cloud in the background.
  - **Event-Driven Cloud Update:** The POS sends a structured transaction event to the cloud, allowing multiple systems (Inventory, Ledger) to react concurrently.
  - **Zero Trust:** Payments are initiated locally but require network access for final Stripe authorization; if offline, only cash transactions and local CRDT updates proceed until connectivity resumes.

  ## AI Agent Integration Points
  - **Operations Agent (The Manager):** Subscribes to the transaction event bus. If a sync conflict occurs (e.g., an item is sold online and offline simultaneously), the agent automatically resolves the conflict based on timestamp logic or places one order on backorder, notifying the merchant.
  - **Finance Agent (The Accountant):** Reconciles cash and Tap-to-Pay ledger entries at the end of the day.

  # Implementation Prompt
  Implement the Omnichannel Sync Engine that bridges the mobile Tap-to-Pay POS and the global Inventory system.

  **Outcome:** When a merchant finalizes a purchase via Tap-to-Pay, the platform must immediately and automatically deduct the sold item from the global inventory, ensuring that the online storefront reflects the accurate stock count. If the device is offline, it must correctly sync once network access is restored.

  **Critical User Journey (CUJ):**
  1. The merchant selects an item and proceeds to checkout using the mobile POS app.
  2. The merchant completes the transaction via Tap-to-Pay.
  3. The local store instantly reflects the sale.
  4. The background sync engine successfully transmits the transaction event to the global event bus.
  5. The global inventory database processes the event and decrements the SKU count.
  6. The merchant logs into the web dashboard and verifies the inventory has decreased by the correct amount without any manual intervention.

  **Acceptance Criteria:**
  - The integration must support offline cash transactions syncing eventually.
  - The system must not double-decrement inventory.
  - A comprehensive E2E test must verify that an in-person transaction reliably updates the global inventory view.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, omnichannel, mobile-pos]
assignees: []
