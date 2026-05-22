issue_title: "[Architecture] Autonomous Hybrid P2P Device Sync Mesh"
issue_description: |
  # [Architecture] Autonomous Hybrid P2P Device Sync Mesh

  ## Problem Statement
  Small business owners operating in challenging network conditions—such as a food cart owner like Fatima working outdoors, or Maya managing orders across an iPad POS in the storefront and a kitchen display system (KDS) in the back—suffer from order desynchronization and data loss when relying entirely on cloud-based sync. These businesses require multiple devices to communicate instantly (e.g., ringing up a falafel at the front counter and instantly printing the ticket in the kitchen) even when the internet connection drops or is severely degraded. Current cloud-first solutions freeze or drop orders during outages, leading to lost revenue and customer dissatisfaction. OHC needs a robust, zero-configuration P2P sync mechanism that allows local devices to form an offline mesh network and securely sync state, before eventually reconciling with the cloud.

  ## Research Report
  - **Competitive Audit**:
    - **Shopify POS**: Strongly cloud-dependent. Struggles heavily in offline scenarios, especially across multiple devices.
    - **Square**: Offers an "Offline Mode" for taking payments, but local multi-device sync (e.g., POS to KDS) without internet can be fragile or requires specific networking setups.
    - **Toast**: Excels at local networking but requires proprietary hardware and complex local network configurations (hardwired routers, static IPs) that non-technical SMB owners cannot manage.
    - **OHC Advantage**: By implementing a zero-config P2P mesh using modern web/mobile protocols (e.g., WebRTC, Bluetooth LE, or Local Network Discovery), OHC can provide the reliability of a hardwired system on standard consumer devices (iPads, Android phones) without the setup friction.

  - **Key Findings**:
    - Intermittent internet is a top 3 complaint for mobile and outdoor vendors.
    - The "Grandmother Test" requires that devices discover each other automatically without the user entering IP addresses or pairing codes.
    - Multi-tenant isolation remains critical even in local offline meshes to prevent data leakage in shared environments.

  ## Design Doc

  ### Data Model & Invariants

  To support offline P2P sync, the data model requires CRDTs (Conflict-free Replicated Data Types) or a robust event-sourced ledger that can be merged deterministically.

  ```mermaid
  erDiagram
      TENANT ||--o{ DEVICE : "owns"
      DEVICE ||--o{ LOCAL_EVENT_LEDGER : "writes to"
      LOCAL_EVENT_LEDGER ||--o{ SYNC_SESSION : "exchanges via"
      DEVICE ||--o{ DEVICE_PEER : "discovers"

      DEVICE {
          uuid id
          string role "POS, KDS, Scanner"
          timestamp last_cloud_sync
      }

      LOCAL_EVENT_LEDGER {
          uuid event_id
          string entity_type "Order, Timecard"
          jsonb payload
          vector clock "Logical Clock for CRDT"
      }

      SYNC_SESSION {
          uuid session_id
          string transport "WebRTC, BLE, mDNS"
          timestamp initiated_at
      }
  ```

  ### AI Department Coordination

  - **Operations Agent**: Monitors the health of the local mesh. If a device drops off the mesh (e.g., the KDS tablet runs out of battery), the Operations Agent triggers a local notification on the primary POS ("Warning: Kitchen Display is disconnected. Orders will not print in the back.").
  - **IT/Support Agent**: Invisibly handles the complex networking (NAT traversal, mDNS resolution, CRDT conflict resolution) without alerting the user unless physical intervention is required.

  ### Sequence Diagram: Offline Order Sync

  ```mermaid
  sequenceDiagram
      participant POS as Storefront iPad (POS)
      participant KDS as Kitchen Android (KDS)
      participant Cloud as OHC Cloud

      Note over POS, KDS: Internet connection lost
      POS->>POS: Ring up Order #101
      POS->>POS: Append to LOCAL_EVENT_LEDGER
      POS->>KDS: Discover via mDNS / BLE
      POS->>KDS: Establish P2P secure channel (SPIFFE/mTLS)
      POS->>KDS: Sync event: Order #101 Created
      KDS->>KDS: Apply CRDT merge, display order
      Note over POS, Cloud: Internet connection restored
      POS->>Cloud: Background sync: LOCAL_EVENT_LEDGER
      KDS->>Cloud: Background sync: LOCAL_EVENT_LEDGER
      Cloud->>Cloud: Reconcile state globally
  ```

  ### Mobile-First UX Flow
  1. **Zero-Config Pairing**: The user brings a new tablet near the primary phone. A translucent glass bottom-sheet appears: "Kitchen Display detected. Tap to connect to your network." No passwords or IP settings required.
  2. **Network Status Indicator**: A small, unintrusive icon in the header (using macOS-style styling) shows green when fully cloud-synced, and yellow when operating in "Local Mesh Only" mode, ensuring the user knows the system is still functioning locally.

  ### Key Design Decisions
  1. **Local-First Architecture**: Reads and writes always hit the local datastore first (e.g., SQLite/SIPDB context or local CRDT store) for zero latency, syncing asynchronously to peers and the cloud.
  2. **Zero-Trust Local Mesh**: Even though devices are on the same local network, all P2P communication must be encrypted using short-lived certificates derived from the tenant's secure identity boundary, preventing rogue devices from snooping.

  ## Implementation Prompt

  **Implementer Agent Task**:
  Implement the core P2P discovery and secure sync protocol for the OHC mobile/desktop clients to enable the Autonomous Hybrid P2P Device Sync Mesh.

  **Customer-User Journey (CUJ)**:
  1. Fatima operates her POS on an Android phone and a KDS on an iPad. The mobile network goes down.
  2. Fatima rings up a new pre-order on the POS.
  3. The POS automatically discovers the KDS on the local network (via mDNS or BLE) and securely transmits the order event.
  4. The KDS receives the event, updates its local state, and displays the order to the kitchen staff instantly, despite the lack of internet.
  5. When the POS regains internet access, it silently syncs the locally merged state back to the OHC Cloud.

  **Acceptance Criteria**:
  - Design and implement a cross-platform local discovery mechanism (e.g., using mDNS/Bonjour or BLE) that requires zero user configuration.
  - Implement a secure P2P transport layer between discovered devices, ensuring mutual authentication scoped to the specific `tenant_id`.
  - Implement a deterministic conflict resolution strategy (e.g., logical clocks / CRDTs) for merging the `LOCAL_EVENT_LEDGER` across devices.
  - Ensure the UI gracefully transitions between "Cloud Synced" and "Local Mesh Only" states with appropriate visual indicators (using OHC design tokens).
  - Ensure background cloud reconciliation is robust against data loss and handles concurrent modifications seamlessly.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
