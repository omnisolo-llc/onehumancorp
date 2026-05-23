issue_title: "Implement Offline-First Secure Biometric Identity Mesh"
issue_description: |
  # [Architecture] Offline-First Secure Biometric Identity Mesh

  ## Title
  Implement an Offline-First Secure Biometric Identity Mesh for Shared Terminals

  ## Problem Statement
  For small business owners like Fatima (food cart operator) or Carlos (handyman with apprentices), managing employee access on shared devices is a nightmare, especially when internet connectivity is spotty. Currently, when the internet drops, staff either can't log in, or they just share a single generic "admin" account. This breaks accountability—if an order goes wrong, a tip needs to be split, or cash goes missing, the business owner has no idea who was at the register. We need a way for staff to instantly swap accounts on a single device (like an iPad or low-end Android tablet) using Face ID, Touch ID, or a quick PIN, completely offline, while guaranteeing absolute security and synchronization once the connection is restored.

  ## Research Report
  **Market Context & Competitor Analysis:**
  - **Square & Toast:** Both offer employee PIN codes for shared point-of-sale devices. However, their offline capabilities are limited. While offline payments might process (with risk), user authentication and granular role-based access control often degrade or fail entirely, reverting to basic local caches.
  - **Shopify POS:** Relies on PINs for staff switching. True biometric fallback offline is not a native first-class capability integrated with an AI audit log.
  - **Wix/Squarespace:** Primarily online-first; their physical POS offerings lack robust multi-tenant offline identity meshes.
  - **The Gap:** No platform offers a zero-trust, cryptographically secure identity mesh that operates flawlessly offline using local biometric enclaves (FaceID/TouchID/FIDO2) linked to an AI-driven audit trail. Small business owners need enterprise-grade accountability (who did what, when) wrapped in a "grandmother test" compliant, instantaneous UX.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      DeviceLocalEnclave ||--o{ TerminalSession : "securely signs"
      TerminalSession }|--|| StaffIdentity : "authenticates"
      StaffIdentity ||--o{ AuditLogEvent : "generates"
      AuditLogEvent }|--|| CloudIdentityMesh : "syncs via CRDT"

      DeviceLocalEnclave {
          string device_key PK
          string biometric_hash
          boolean is_offline
      }

      TerminalSession {
          string session_id PK
          string staff_id FK
          timestamp started_at
          timestamp ended_at
      }

      StaffIdentity {
          string staff_id PK
          string role
          string pin_hash
      }

      AuditLogEvent {
          string event_id PK
          string action
          timestamp occurred_at
          boolean synced
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Staff
      participant Mobile Device (App)
      participant Local Crypto Enclave
      participant NATS Hybrid Mesh (Offline Cache)
      participant Cloud Identity Server

      Staff->>Mobile Device (App): Tap 'Switch User' & Authenticate (Biometrics/PIN)
      Mobile Device (App)->>Local Crypto Enclave: Validate local biometric/PIN hash
      Local Crypto Enclave-->>Mobile Device (App): Return signed offline token
      Mobile Device (App)->>NATS Hybrid Mesh (Offline Cache): Log 'Login Event' (Offline CRDT)
      Mobile Device (App)-->>Staff: Instant Access Granted (Sub 100ms)

      Note over NATS Hybrid Mesh (Offline Cache), Cloud Identity Server: ... Later when internet restores ...

      NATS Hybrid Mesh (Offline Cache)->>Cloud Identity Server: Sync event queue via TLS
      Cloud Identity Server-->>NATS Hybrid Mesh (Offline Cache): Ack sync, resolve conflicts
  ```

  ### Mobile UX Flow (375px First)
  1. **The Lock Screen (Offline or Online):** A sleek, translucent glass frosted overlay appears on the 375px screen after inactivity or when tapping "Switch User". It shows prominent, friendly avatars of active staff for that shift.
  2. **Instant Tap:** Fatima’s employee taps their face icon.
  3. **Biometric/PIN Challenge:** The system immediately prompts for FaceID/Fingerprint (or a 4-digit PIN fallback). This check is evaluated 100% locally against the secure enclave.
  4. **Success State:** Upon success, a subtle haptic feedback triggers, and the dashboard instantly transitions (0 latency) to that specific employee's restricted view (e.g., they can take orders but cannot issue refunds).
  5. **Visual Indicator:** The top right corner of the 375px viewport always displays a small chip showing the currently active staff member and an "Offline Mode" badge if disconnected.

  ### AI Agent Integration Points
  - **Security & Ops Agent:** An invisible background agent continuously monitors the offline sync queue. If an employee logs in offline, processes 50 high-risk transactions, and logs out, the agent flags the batched sync file for review and sends a plain-language SMS to the business owner: "Just a heads up, Alex processed 5 refunds while the iPad was offline today. You can review them here."
  - **Support Agent (CS):** If an employee forgets their PIN while offline, the AI agent can step in via SMS to the business owner to authorize a temporary offline bypass code.

  ### Key Design Decisions & Why
  - **Local-First Cryptography:** We rely on the device's native secure enclave (Secure Enclave on iOS, Titan/TrustZone on Android) to store encrypted hashes of PINs and biometric keys. **Why:** This ensures sub-100ms switching speeds and full offline capability without compromising zero-trust principles.
  - **CRDT-based Audit Logs:** All actions taken during an offline session are appended to a local CRDT (Conflict-Free Replicated Data Type) log. **Why:** To guarantee that when connectivity is restored, the multi-tenant mesh can resolve state conflicts without losing a single trace of staff activity.
  - **"Grandmother Test" Compliance:** No mention of keys, syncs, or tokens in the UI. Just "Tap your face to start working."

  ## Implementation Prompt
  **To the Implementer Agent:**
  Build the user-facing Offline-First Secure Biometric Identity Mesh for OneHumanCorp.
  - **User Journey (CUJ):** A staff member must be able to switch to their account on a shared mobile device (375px width) in under 1 second, even with zero internet connectivity. They tap their avatar, provide biometric or PIN auth, and instantly access their dashboard.
  - **Acceptance Criteria:**
    1. The UI must render a staff-switching lock screen matching OHC's translucent glass macOS-style design system.
    2. Authentication must succeed fully offline by verifying against locally cached, securely encrypted credentials.
    3. Every action taken while offline under a specific staff session must be locally logged with that session's identity.
    4. The system must automatically and invisibly sync the offline audit trail to the cloud once connectivity is restored, resolving any state conflicts.
    5. The UI must clearly indicate the current active user and connection status without cluttering the screen.
  - **Note:** Do not prescribe specific database schemas or API endpoints. Focus on the local offline state management, secure enclave interface logic, and the UI/UX flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
