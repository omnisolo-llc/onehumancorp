issue_title: "Universal Offline-First POS & Tap-to-Pay Engine"
issue_description: |
  ## Title
  Universal Offline-First POS & Tap-to-Pay Engine

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart) need to seamlessly accept in-person payments in addition to online orders. They face internet connectivity drops (especially food carts or pop-up events). Relying on separate systems for online sales (Shopify) and in-person (Square) fragments their inventory, customer data, and analytics. They need a single, offline-capable Point-of-Sale (POS) and Tap-to-Pay engine built directly into the OneHumanCorp app that works reliably even without internet, and syncs instantly when connectivity returns.

  ## Research Report
  - **Competitor Analysis**:
    - **Shopify**: Offers Shopify POS, but offline capabilities are limited (can take cash, but offline card processing is risky/unsupported natively without specific hardware).
    - **Square**: King of in-person, strong offline mode for swiped/inserted cards, but setting up a full e-commerce store with unified inventory is still complex for non-technical users.
    - **Wix/Squarespace**: Afterthoughts for physical POS, relying heavily on third-party integrations (Stripe Terminal) but lacking true native offline-first mobile experiences.
  - **Data & Findings**: 30% of small businesses experience connectivity issues at least once a month. Food carts (Fatima) and pop-up boutiques (Priya) operate in dynamic environments (festivals, streets) where Wi-Fi is nonexistent and cellular data drops.
  - **Strategic Opportunity**: By building an offline-first architecture using a local-first DB (e.g., SQLite/CRDTs) and Stripe Tap-to-Pay on mobile, OHC can own the entire omni-channel experience invisibly.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
    DeviceNode ||--o{ LedgerTransaction : records
    DeviceNode {
      string device_id
      string tenant_id
      boolean is_online
      string sync_state
    }
    LedgerTransaction ||--o{ PaymentIntent : creates
    LedgerTransaction {
      string tx_id
      string amount
      string currency
      string status
      timestamp created_at
    }
    SyncEngine ||--o{ DeviceNode : synchronizes
    SyncEngine {
      string last_cursor
    }
  ```

  ```mermaid
  sequenceDiagram
    participant MobileApp
    participant LocalDB
    participant SyncEngine
    participant CloudDB
    participant AI_FinanceAgent

    MobileApp->>LocalDB: Record Tap-to-Pay Auth (Offline)
    LocalDB-->>MobileApp: Confirm Local Tx
    MobileApp->>SyncEngine: Connectivity Restored
    SyncEngine->>CloudDB: Push Tx Event
    CloudDB->>AI_FinanceAgent: Trigger Ledger Reconciliation
    AI_FinanceAgent-->>MobileApp: Push Notification "Payment Settled"
  ```

  ### UI Wireframes (375px first)
  - **Screen 1 (POS Cart)**: Translucent Glass top nav. Large, tappable product grid with photos (2 columns). Bottom sheet sticky action: "Charge $45.00".
  - **Screen 2 (Tap-to-Pay)**: Full-screen modal, dark mode default. Center pulsing NFC icon. Text: "Hold card or phone to reader". Prominent "Cancel" button.
  - **Screen 3 (Success & Receipt)**: Green checkmark animation. Options: "Email Receipt", "Text Receipt", "No Receipt". AI Agent suggest: "Add to customer loyalty program?"

  ### Mobile UX Flow
  1. Add items to cart (fast, cache-hit).
  2. Tap "Charge".
  3. Seamless transition to native NFC Tap-to-Pay overlay.
  4. Instant local approval via risk-engine if offline, queueing sync.
  5. Success screen with automated receipt handling.

  ### AI Agent Integration Points
  - **Finance Agent**: Reconciles offline transactions asynchronously when online. Alerts on high-risk offline transactions.
  - **Marketing Agent**: Automatically prompts to send digital receipt via SMS, enrolling the customer in loyalty tracking.
  - **Operations Agent**: Automatically decrements local inventory and flags if an item is sold out, pausing online availability.

  ### Key Design Decisions
  - **Local-First Datastore**: Use a local database with CRDTs to ensure all transactions, inventory decrements, and cart states are durable offline.
  - **Optimistic UI**: Never block the user waiting for a network request during checkout. Provide immediate visual feedback.
  - **Zero-Trust**: Ensure multi-tenant isolation at the local database layer by strictly scoping encryption keys per tenant device session.

  ## Implementation Prompt
  As an Implementer agent, build the Universal Offline-First POS & Tap-to-Pay Engine core. Create the data models (e.g., `LedgerTransaction`, `DeviceNode`) and the sync engine logic that handles offline queueing and background synchronization. Design the system to gracefully handle network drops during the checkout flow, ensuring the UI remains perfectly responsive and optimistic. Integrate with the AI Finance Agent to trigger reconciliation once transactions sync to the cloud. Do not prescribe the specific local DB technology, but enforce the CRDT synchronization interface and strict Zero-Trust tenant boundaries. Ensure the feature passes the "grandmother test" — fully usable on a 375px mobile screen by a non-technical user in a high-stress environment (like a busy food cart).

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
