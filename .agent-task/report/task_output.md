issue_title: "Offline-First Omnichannel Inventory Sync via Optimistic Mutation CRDTs"
issue_description: |
  # Offline-First Omnichannel Inventory Sync via Optimistic Mutation CRDTs

  ## Problem Statement
  For solopreneurs and small businesses expanding their in-person operations (like Fatima opening a second food cart, or Priya running a pop-up market stand with an assistant), managing multiple POS devices simultaneously without external network reliance is a critical pain point. Currently, the OHC Tap-to-Pay and Offline-First POS capability functions securely on a single device. However, if Fatima and her assistant are both ringing up customers on separate Android devices in an area with no cell service or unstable Wi-Fi, their local inventory state diverges, and they risk overselling limited items. They need an invisible, zero-configuration local network mesh that securely synchronizes POS carts, inventory, and order status across multiple employee devices in real-time without needing a cloud connection.

  ## Research Report
  - **Shopify POS**: Requires a stable internet connection for syncing between multiple terminals.
  - **Square**: Offers an offline mode, but synchronization across multiple devices in the same physical location without cloud connectivity is limited and prone to conflict when eventually synced.
  - **OHC Gap**: OHC currently handles offline transactions per-device using a local SQLite/CRDT outbox. However, there is no peer-to-peer (P2P) mesh to reconcile CRDTs locally between multiple active devices.

  ## Design Doc
  ### Data Model & Invariants
  - Leverage an Optimistic Mutation Engine with Conflict-Free Replicated Data Types (CRDTs).
  - Implement a Local-First architecture where `OperationIntent` logs are stored locally.
  - The local sync engine establishes a zero-config Bluetooth Low Energy (BLE) or Wi-Fi Direct mesh network.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant DeviceA as App (Fatima)
      participant Mesh as P2P Local Mesh (BLE/WiFi Direct)
      participant DeviceB as App (Assistant)
      participant Gateway as OHC API Gateway (Cloud)

      DeviceA->>DeviceA: Mark "Vegan Cake" Sold Out (Offline)
      DeviceA->>Mesh: Broadcast State Vector (CRDT)
      Mesh->>DeviceB: Receive State Vector
      DeviceB->>DeviceB: Resolve CRDT (Update Inventory)

      Note over DeviceA: Network Restored
      DeviceA->>Gateway: Sync Batched Local Intents
      Gateway-->>DeviceA: 200 OK (Cloud State Synced)
  ```

  ### Mobile UX Flow (375px First)
  1. **Dashboard**: Fatima and Assistant open the app. A small, premium translucent pill indicates "Local Sync Active (2 Devices)". No configuration is required.
  2. **Inventory Depletion**: Assistant sells the last Vegan Cake via Tap-to-Pay. The transaction is instantly broadcasted over the P2P mesh.
  3. **Immediate Update**: Fatima's screen instantly updates to show Vegan Cake as "Sold Out", preventing an oversell.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager")**: Monitors the eventual cloud sync and resolves complex multi-device conflicts (if any) gracefully in the background without overwhelming the user with "sync error" dialogs.

  ## Implementation Prompt
  Implement the P2P Local Mesh network layer in the Flutter frontend and integrate it with the existing `MutationService`.
  - **User-Facing Outcome**: Two devices running the app in offline mode can synchronize inventory updates and cash/tap-to-pay intent states seamlessly across a local P2P connection (BLE/WiFi Direct).
  - **CUJ**:
    1. Device A and B are offline.
    2. Device A decrements stock.
    3. Device B's UI updates to reflect the stock change without cloud connection.
  - **Acceptance Criteria**: Establish a P2P connection mechanism for Flutter (e.g. using `nearby_connections`). Send CRDT state vectors between devices. Persist received updates into the local SQLite store and reflect in the UI immediately.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
