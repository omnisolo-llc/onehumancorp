issue_title: "Implement Hardware-Free P2P Offline Mesh Sync for Multi-Device POS"
issue_description: |
  # Title: Implement Hardware-Free P2P Offline Mesh Sync for Multi-Device POS

  ## Problem Statement
  For solopreneurs and small businesses expanding their in-person operations (like Fatima opening a second food cart, or Priya running a pop-up market stand with an assistant), managing multiple POS devices simultaneously without external network reliance is a critical pain point. Currently, the OHC Tap-to-Pay and Offline-First POS capability functions securely on a single device, queueing transactions for cloud sync. However, if Fatima and her assistant are both ringing up customers on separate Android/iOS devices in an area with no cell service or unstable Wi-Fi, their local inventory state diverges, and they risk overselling limited items. They need an invisible, zero-configuration local network mesh that securely synchronizes POS carts, inventory, and order status across multiple employee devices in real-time without needing a cloud connection.

  ## Research Report
  **Findings & Competitive Analysis:**
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
  2. **Unified State:** Once joined, the connection is established over high-bandwidth Wi-Fi Direct or Multipeer Connectivity. The assistant's inventory reflects Fatima's exact local CRDT state in <100ms.
  3. **Offline Sync Validation:** When the assistant processes a Tap-to-Pay order, the inventory decrements and order data instantly replicates to Fatima's device, showing up in her timeline, even with airplane mode on.
  4. **Self-Healing Uplink:** When *any* device in the mesh regains internet access, it acts as the gateway node, securely flushing the unified CRDT queue to the OHC Cloud.

  ### AI Agent Integration Points
  - **Operations Agent (Local Execution):** Resolves CRDT conflict edges locally (e.g., two devices sell the last Falafel simultaneously) prioritizing the earliest local timestamp and instantly notifying the other device.
  - **Finance Agent (Cloud Execution):** Upon reconnection, reconciles the batched, multi-device P2P transactions to ensure ledger integrity and identifies any Tap-to-Pay declines from the offline queue.

  ### Key Design Decisions
  - **P2P Transport Layer:** Utilize cross-platform frameworks (e.g., integrating a Rust core compiled via UniFFI targeting Apple Multipeer and Android Nearby Connections) for true routerless connectivity.
  - **CRDT Convergence:** Implement a Merkle-DAG or state-based CRDT model to guarantee mathematical convergence of inventory and ledger states regardless of partition length or sync order.
  - **Zero-Trust Mesh Security:** Nodes authenticate over the local P2P mesh using tenant-scoped SPIFFE SVIDs distributed during the last online session, ensuring encrypted, trusted local synchronization.

  ## Implementation Prompt
  **User-Facing Outcome:** As a small business owner (like Fatima) expanding my physical location with an assistant, I can have my assistant run a second OHC mobile app register. Even in a spotty network environment (like a crowded market or basement pop-up), both our devices sync inventory and orders instantly over a local P2P mesh without needing Wi-Fi or a router.
  **CUJ & Acceptance Criteria:**
  1. Two instances of the OHC application are running (e.g., simulated in Playwright or integration tests) representing two separate devices (Fatima's and the Assistant's) for the same tenant.
  2. The devices are configured in an offline mode (simulating no cloud connectivity).
  3. A local P2P mesh connection is established between the two devices using tenant-scoped SPIFFE SVIDs for authentication.
  4. The assistant processes a sale, generating a CRDT mutation for an inventory decrement.
  5. The mutation is instantly synchronized over the P2P mesh to Fatima's device, updating her local UI state to reflect the new inventory count.
  6. When one device regains "cloud connectivity," it securely flushes the synchronized CRDT queue to the central OHC backend ledger, resolving any conflicts via the Operations/Finance Agents.
  7. Automated tests verify convergence of state across multiple nodes and the successful reconciliation in the cloud database.

  ## Priority
  P1

  ## Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
