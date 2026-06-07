issue_title: "[Mobile POS] Dynamic Bundle & Upsell Optimization for Offline-First Environments"
issue_description: |
  ## Problem Statement
  For solopreneurs running physical/hybrid shops (like Priya at a popup market or Fatima managing a bustling food cart), the ability to maximize cart value via dynamic bundling is critical. However, current implementations of product bundling and upsells rely entirely on the cloud-based "Marketing Agent" or "Vigilant Manager" for computation. When Fatima operates in areas with spotty cell service (offline or local-mesh mode), the app falls back to basic single-item additions, completely dropping upsell opportunities (e.g., "Add a drink for $1") and bundled pricing logic, directly resulting in lost revenue.

  ## Research Report
  **Market Gap:**
  - **Square:** Handles basic offline bundling but requires rigid, pre-defined rules that cannot adapt dynamically to inventory levels.
  - **Shopify POS:** Relies heavily on online connectivity for advanced discounts, apps, and script-based bundling logic. Offline mode strips these features down to basic calculations.
  - **OHC Opportunity:** By migrating the "Marketing/Ops AI" dynamic bundling rules to the local CRDT state, OHC can ensure that upsells remain intelligent and fully functional even when the device is disconnected. The OHC App must evaluate inventory levels locally and propose bundles without a roundtrip to the cloud.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device (Offline First)
          App[OHC Mobile App 375px] --> LocalDB[(Local SQLite / CRDT)];
          LocalDB --> BundleEngine[Local Bundling Engine];
          LocalDB --> InventoryState[Local Inventory View];
          BundleEngine -->|Evaluates| InventoryState;
          BundleEngine -->|Proposes| CartUI[Dynamic Cart UI];
      end

      subgraph Cloud Environment
          MainDB[(Cloud Postgres Ledger)] -->|Syncs Rules| SyncEngine[Offline Sync Engine];
          Agents[Marketing & Ops Agents] -->|Generates Rules| MainDB;
      end

      SyncEngine --> LocalDB;
  ```

  ### Mobile UX Flow (375px First)
  1. **Cart Building (Offline):** Fatima adds a "Chicken Shawarma" to the cart.
  2. **Local Evaluation:** The Local Bundling Engine evaluates the cart against locally cached rule sets (e.g., "Meal Deal: Add Drink & Fries").
  3. **Upsell Prompt:** A translucent macOS-style card slides up instantly: "Make it a combo? +$3.00 for Drink and Fries. 5 Fries left." (Inventory checked locally).
  4. **Acceptance:** Fatima taps "Accept", and the cart automatically recalculates the bundled price.
  5. **Payment:** The flow proceeds to the native Tap-to-Pay or Cash logging screen.

  ### AI Agent Integration Points
  - **Marketing Agent (Cloud):** Periodically analyzes sales data and generates optimized bundle rules. These rules are serialized and synced to the mobile devices via the CRDT sync engine when online.
  - **Ops Agent (Local Context):** The local engine evaluates the synced rules strictly against the *local* inventory state, ensuring it never upsells an out-of-stock item.

  ### Key Design Decisions
  - **Rule Serialization:** Bundle logic must be represented as pure data (e.g., JSON rulesets) that can be easily synchronized and evaluated locally without requiring heavy V8/JS engines or LLM inference on the mobile device.
  - **Inventory-Aware:** The local evaluation must check the local CRDT inventory before showing an upsell.

  ## Implementation Prompt
  Implement the Local Bundling Engine to support dynamic upsells and bundled pricing in offline environments.
  - **User-Facing Outcome:** When an owner adds items to a cart, the mobile POS instantly suggests relevant upsells and applies bundled pricing, even when fully offline, based on pre-synchronized rules and current local inventory.
  - **CUJ:**
    1. Owner adds "Item A" to cart while offline.
    2. System evaluates local rules and inventory, determining "Item B" is a valid upsell.
    3. UI displays a prompt to add "Item B" for a bundled price.
    4. Owner accepts, cart updates to bundled price.
    5. Owner processes payment (e.g., cash).
  - **Acceptance Criteria:**
    - Bundle rules can be evaluated locally without network access.
    - Upsells are hidden if the target item is out of stock in the local inventory.
    - Bundled pricing correctly overrides individual item pricing when conditions are met.
    - Zero network calls are made during the cart building and evaluation phase.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
