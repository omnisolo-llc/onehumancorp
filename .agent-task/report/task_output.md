issue_title: "[Mobile Tap-to-Pay & Offline-First POS] Unified Physical and Digital Retail"
issue_description: |
  # Architecture Document: Hardware-Free P2P Offline Mesh Sync for Multi-Device POS

  ## Problem Statement
  For solopreneurs and small businesses expanding their in-person operations (like Fatima opening a second food cart, or Priya running a pop-up market stand with an assistant), managing multiple POS devices simultaneously without external network reliance is a critical pain point. Currently, the OHC Tap-to-Pay and Offline-First POS capability functions securely on a single device, queueing transactions for cloud sync. However, if Fatima and her assistant are both ringing up customers on separate Android devices in an area with no cell service or unstable Wi-Fi, their local inventory state diverges, and they risk overselling limited items. They need an invisible, zero-configuration local network mesh that securely synchronizes POS carts, inventory, and order status across multiple employee devices in real-time without needing a cloud connection.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify POS:** Offers robust multi-register support, but heavily relies on a stable internet connection or an expensive local server/hub architecture to sync complex states between registers. True peer-to-peer (P2P) offline sync without a central network is limited.
  - **Square POS:** Excellent multi-device synchronization when online. Offline mode functions for queuing payments, but true inter-device inventory synchronization typically degrades if the primary local network goes down.
  - **OHC Opportunity:** By leveraging native mobile P2P frameworks (like Apple's Multipeer Connectivity or Android's Wi-Fi Direct/Nearby Connections) combined with Local CRDTs, OHC can create an invisible, self-healing local mesh network. The devices can sync state securely without a router or cloud gateway, leapfrogging legacy systems.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Local Environment (No Cloud/Router)
          DeviceA[Fatima's OHC App 375px] --> LocalDB_A[(CRDT Store A)];
          DeviceB[Assistant's OHC App 375px] --> LocalDB_B[(CRDT Store B)];
          LocalDB_A <--> P2PMesh[P2P Mesh Network];
          LocalDB_B <--> P2PMesh;
          DeviceA <--> P2PMesh;
          DeviceB <--> P2PMesh;
      end

      subgraph Cloud Environment
          Gateway[OHC API Gateway]
          MainDB[(Cloud Postgres Ledger)]
          Agents[AI Agent Swarm]
      end

      P2PMesh -- Network Restored (Any Device) --> Gateway;
      Gateway --> MainDB;
      Gateway --> Agents;
  ```

  ### Mobile UX Flow (375px First)
  1. **Zero-Config Pairing:** When Fatima's assistant opens the OHC app, the system automatically detects Fatima's nearby device via Bluetooth LE/mDNS. A translucent "Join Local Register Network" prompt appears.
  2. **Unified State:** Once joined, the connection is established over high-bandwidth Wi-Fi Direct. The assistant's inventory reflects Fatima's exact local CRDT state in <100ms.
  3. **Offline Sync Validation:** When the assistant processes a Tap-to-Pay order, the inventory decrements and order data instantly replicates to Fatima's device, showing up in her timeline, even with airplane mode on.
  4. **Self-Healing Uplink:** When *any* device in the mesh regains internet access, it acts as the gateway node, securely flushing the unified CRDT queue to the OHC Cloud.

  ### AI Agent Integration Points
  - **Operations Agent (Local Execution):** Resolves CRDT conflict edges locally (e.g., two devices sell the last Falafel simultaneously) prioritizing the earliest local timestamp and instantly notifying the other device.
  - **Finance Agent (Cloud Execution):** Upon reconnection, reconciles the batched, multi-device P2P transactions to ensure ledger integrity and identifies any Tap-to-Pay declines from the offline queue.

  ### Key Design Decisions
  - **P2P Transport Layer:** Utilize cross-platform frameworks (e.g., integrating a Rust core compiled via UniFFI targeting Apple Multipeer and Android Nearby Connections) for true routerless connectivity.
  - **CRDT Convergence:** Implement a Merkle-DAG or state-based CRDT model to guarantee mathematical convergence of inventory and ledger states regardless of partition length or sync order.
  - **Zero-Trust Mesh Security:** Nodes authenticate over the local P2P mesh using tenant-scoped SPIFFE SVIDs distributed during the last online session, ensuring encrypted, trusted local synchronization.

  ### Implementation Prompt
  Implement the Tap-to-Pay and Offline-First POS capabilities.
  - **User-Facing Outcome:** Users can open the mobile app, add items to a cart, and process a payment by having a customer tap a credit card directly on the merchant's phone. The app must remain responsive and allow inventory management even with airplane mode on.
  - **CUJ (Critical User Journey):**
    1. User adds item to cart in the mobile app.
    2. User selects "Tap to Pay".
    3. Customer taps card.
    4. Payment succeeds and inventory decrements.
    5. (Alternative) User is offline, logs a cash sale, and the app syncs the sale to the cloud when reconnected.
  - **Acceptance Criteria:**
    - Native Tap-to-Pay flow is triggered on iOS/Android.
    - App state (inventory, orders) is readable and writable when offline, syncing automatically upon network restoration.
    - No developer jargon (CRDT, sync) is visible to the user.
    - UI strictly adheres to the glassmorphism and card-based design system.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
