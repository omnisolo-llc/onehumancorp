issue_title: "Architect Invisible P2P Local Device Sync Mesh"
issue_description: |
  # Title: Architect Invisible P2P Local Device Sync Mesh

  ## Problem Statement
  Fatima (Food Cart, 50) and Priya (Boutique Owner, 35) often run their businesses using multiple devices simultaneously. Fatima uses a tablet at the front window to take orders and an Android phone in the back as a Kitchen Display System (KDS) to see what to cook. Priya uses her iPhone to check inventory while her employee rings up a customer on the store iPad.
  However, when the internet connection drops or becomes unstable (a frequent occurrence in food carts or thick-walled boutiques), traditional cloud-based Point of Sale (POS) systems break down. Devices stop talking to each other. An order placed on Fatima's front tablet never appears on her KDS in the back, leading to missed orders and angry customers. Existing solutions either force the business to halt until the cloud returns (Square/Shopify POS) or require expensive, professionally installed hardwired local servers (Toast). A non-technical small business owner needs their devices to magically stay perfectly synced with each other, even completely offline, with absolutely zero networking configuration.

  ## Research Report
  *   **Competitor Analysis**:
      *   **Square POS & Shopify POS**: Highly reliant on the cloud for multi-device sync. "Offline Mode" is typically restricted to a single device queuing credit card swipes; it does not sync open tickets between a front-of-house tablet and a back-of-house screen.
      *   **Toast**: Offers robust offline local network sync, but requires a complex, hardwired ethernet local area network (LAN) and an expensive on-premise server. This completely violates OHC's "zero to live in 10 minutes from a phone" mandate.
  *   **The OHC Differentiator**: We introduce the "Invisible P2P Local Device Sync Mesh". OHC mobile apps will autonomously form a secure, local peer-to-peer (P2P) network using Wi-Fi Direct, Apple Multipeer Connectivity, and Bluetooth Low Energy (BLE). State changes (like a new order) are replicated directly between physically proximate devices. When the WAN (internet) connection is restored, the "leader" device automatically reconciles the local mesh state with the OHC global cloud ledger.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph OHC Cloud (Global)
          Gateway[Zero-Trust Edge Gateway]
          Ledger[Global Ledger & State DB]
          OperationsAgent[AI Operations Agent]
      end

      subgraph Invisible Local Mesh (Offline-Capable)
          DeviceA[Device A: Fatima's Tablet - Front]
          DeviceB[Device B: Fatima's Phone - KDS Back]

          DeviceA <-->|P2P Sync: BLE / Wi-Fi Direct / mDNS| DeviceB
      end

      DeviceA -->|WAN Sync| Gateway
      DeviceB -.->|WAN Sync Fallback| Gateway
      OperationsAgent -.-> Gateway
  ```

  ### Data Model & Invariants (Conceptual)
  ```mermaid
  erDiagram
      TENANT ||--o{ LOCAL_DEVICE : "authorizes"
      LOCAL_DEVICE ||--o{ MESH_EVENT : "emits/receives"

      TENANT {
          string id PK
          string name
      }
      LOCAL_DEVICE {
          string device_id PK
          string tenant_id FK
          string role "leader, follower"
          timestamp last_seen
      }
      MESH_EVENT {
          string event_id PK
          string tenant_id FK
          string type "order_created, ticket_completed"
          blob encrypted_payload
          timestamp local_timestamp
      }
  ```

  ### Key Design Decisions & Invariants
  *   **Zero-Config Auto-Discovery**: Devices logged into the same OHC Tenant ID must continuously and silently discover each other using mDNS/Bonjour and BLE advertising. No IP addresses or pairing codes are ever shown to the user.
  *   **Conflict-Free Replicated Data Types (CRDTs)**: Local state (like the KDS order queue) must be structured using CRDTs to guarantee mathematical eventual consistency without requiring a central coordinator, preventing merge conflicts when devices go offline and reconnect.
  *   **Zero Trust & Multi-Tenancy (SPIFFE/SPIRE)**: Local P2P communication is strictly authenticated. Devices must exchange short-lived SPIFFE/SPIRE derived certificates. A device cannot decrypt or even acknowledge P2P sync packets from a neighboring food cart's OHC app.
  *   **Leader Election**: The mesh autonomously negotiates a "leader" node (usually the device with the strongest WAN connection or battery life) responsible for ultimately syncing the offline mesh state back to the OHC cloud.

  ### Mobile UX Flow (375px First)
  1. **The Invisible Operation**: Under normal circumstances, the mesh is completely invisible. The UI simply works. Fatima taps "Submit Order" on the tablet, and it instantly appears on the phone screen.
  2. **Offline Indicator (Subtle)**: If the cloud connection drops, a small, elegant translucent pill appears at the top of the screen: *"☁️ Offline (Local Sync Active)"*.
  3. **Advanced Diagnostics (Hidden)**: Deep within the "Advanced Settings" menu, a user can view a "Local Devices" card showing connected mesh peers (e.g., "Front Tablet (Online)", "Kitchen Phone (Connected via Bluetooth)"). This must pass the grandmother test by hiding technical jargon.

  ### AI Agent Integration Points
  *   **AI Operations Agent**: Monitors the health of the local mesh when connected to the cloud. If the Operations Agent detects that Fatima's KDS phone hasn't synced in 4 hours, it can proactively send an SMS or push notification: *"Your kitchen screen seems to be offline. Try bringing it closer to the front tablet to sync your latest menu."*

  ## Implementation Prompt
  **Objective**: Implement the Invisible P2P Local Device Sync Mesh for the OHC mobile client to enable zero-config, offline multi-device synchronization.

  **User Journey (CUJ) & Acceptance Criteria**:
  1. Fatima is running her food cart. Her front tablet (taking orders) and back phone (KDS) are both running the OHC app.
  2. She loses cell service. The app must continue to function smoothly.
  3. She takes a new order for a falafel wrap on the tablet.
  4. The tablet autonomously discovers the KDS phone via BLE/Wi-Fi Direct.
  5. The order appears on the KDS phone instantly, despite the lack of internet.
  6. When cell service returns, the local state is quietly synchronized with the OHC cloud.

  **Acceptance Criteria**:
  - Implement a local transport layer that supports BLE and Wi-Fi Direct discovery.
  - Implement a CRDT-based local data store capable of merging concurrent offline edits.
  - Ensure strict tenant isolation: P2P payloads must be cryptographically signed and encrypted so only devices on the same tenant can read them.
  - All technical complexity must be hidden; the UI must not prompt the user for IP addresses or pairing codes.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
