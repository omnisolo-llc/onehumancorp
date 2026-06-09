issue_title: "Zero-Trust Offline-Tolerant Mobile Point of Sale (mPOS)"
issue_description: |
  # Mission Queue Protocol: OHC Core Platform Architecture Research

  ## Problem Statement
  Small business owners like Priya (boutique operator) and Carlos (field service owner) often operate in environments with intermittent connectivity—trade shows, basements, or remote job sites. Existing mPOS solutions either require constant connectivity to process payments or have clunky, disjointed offline modes. The platform lacks a unified, resilient, Zero-Trust mPOS architecture that allows offline-first payment capture (with delayed synchronization) while maintaining strict multi-tenant isolation.

  ## Research Report
  - **Competitor Systems Audit**:
    - *Shopify POS*: Excellent ecosystem but requires reliable internet for smooth operation.
    - *Square*: Strong offline payment support, but heavily tied to their proprietary hardware.
    - *Stripe Terminal*: Great SDKs, but requires a robust application-layer strategy for offline resilience.
  - **Gap Identified**: OHC lacks a dedicated, offline-tolerant ledger and local sync architecture for in-person transactions.

  ## Design Doc
  - **Architecture**:
    - **Local State Store (Flutter)**: Uses an encrypted local SQLite/Hive database to capture pending transactions when offline.
    - **Sync Engine**: A background worker pattern that watches connectivity state. When online, it flushes the local ledger to the OHC backend via a secure gRPC API.
    - **Zero Trust & Security**: Each device authenticates via SPIFFE/SPIRE. Offline tokens are short-lived and cryptographically signed.
    - **Multi-Tenant Isolation**: The local store explicitly isolates data by `tenant_id`, and the backend strictly enforces row-level security on the incoming sync payloads.

    ### Architecture Diagram
    ```mermaid
    sequenceDiagram
        participant User
        participant FlutterApp as OHC Mobile App (Flutter)
        participant LocalDB as Encrypted Local Store
        participant Backend as OHC Backend (Go)
        participant Ledger as Postgres Database

        User->>FlutterApp: Rings up cart & taps "Charge"
        alt Network Offline
            FlutterApp->>LocalDB: Store pending transaction securely
            FlutterApp-->>User: Show "Processing Offline" success state
        else Network Online
            FlutterApp->>Backend: Sync transaction via secure gRPC
            Backend->>Ledger: Commit to ledger (Row-level security)
            Backend-->>FlutterApp: Confirm sync
            FlutterApp-->>User: Show online success state
        end

        loop Background Sync
            FlutterApp->>FlutterApp: Monitor network connectivity
            opt When Online
                FlutterApp->>LocalDB: Read pending transactions
                FlutterApp->>Backend: Flush transactions via secure gRPC
                Backend->>Ledger: Commit to ledger (Row-level security)
                Backend-->>FlutterApp: Confirm sync
                FlutterApp->>LocalDB: Mark as synced/cleared
            end
        end
    ```

    ### Entity-Relationship Diagram
    ```mermaid
    erDiagram
        TENANT {
            uuid tenant_id PK
            string name
        }
        TRANSACTION {
            uuid transaction_id PK
            uuid tenant_id FK
            decimal amount
            string currency
            string status "pending, synced, failed"
            timestamp created_at
            timestamp synced_at
        }
        DEVICE_SESSION {
            uuid session_id PK
            uuid tenant_id FK
            string spiffe_id
            timestamp last_seen
        }

        TENANT ||--o{ TRANSACTION : owns
        TENANT ||--o{ DEVICE_SESSION : registers
        DEVICE_SESSION ||--o{ TRANSACTION : processes
    ```

  - **Mobile UX Flow (375px first)**:
    - 1. User rings up a cart (Product variants, total calculation).
    - 2. User taps "Charge".
    - 3. If offline, the UI shows a "Processing Offline" translucent badge (macOS style).
    - 4. The transaction is saved locally and the user sees a success screen to keep the line moving.
    - 5. An asynchronous visual indicator (e.g., a sync icon in the top navigation) shows pending uploads.
  - **AI Agent Integration**:
    - The *Finance Assistant* flags transactions that were captured offline but failed to sync after 24 hours.
    - The *Customer Assistant* drafts a follow-up SMS (if a number was captured) once the transaction finally clears.

  ## Implementation Prompt
  Implement the offline-tolerant mPOS framework for the Flutter frontend and the corresponding secure sync endpoint on the Go backend.
  - **CUJ**: A business owner rings up an order while disconnected from the internet, taps to charge, and sees the transaction successfully queued. When connection is restored, the system automatically syncs the transaction to the backend ledger.
  - **Acceptance Criteria**:
    - The Flutter app can store a transaction locally when offline.
    - The app automatically syncs local transactions to the backend when online.
    - The Go backend securely processes the synced transactions, enforcing tenant isolation.
    - The UI provides clear, non-technical feedback about the offline state and sync progress.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
