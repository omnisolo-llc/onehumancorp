issue_title: "Implement AI-Assisted Offline-Tolerant Tap-to-Pay Terminal Architecture"
issue_description: |
  ## Problem Statement
  For mobile-first operators like Priya (boutique operator) and Fatima (food cart operator), relying entirely on a stable internet connection for in-person transactions is a critical failure point. In high-density areas, markets, or events, network connectivity can drop. Current web-only or strict-online checkout flows prevent them from capturing physical sales. Additionally, reconciling offline-queued transactions with inventory once connectivity is restored requires manual effort, causing inventory drift and accounting headaches. OHC currently lacks an offline-tolerant, native Tap-to-Pay architecture with AI-driven reconciliation for point-of-sale (POS) operations.

  ## Research Report
  - **Competitive Landscape**:
    - *Stripe Terminal / Tap to Pay on iPhone/Android*: Offers native SDKs that handle the secure reading of NFC cards without external hardware. However, it requires a robust wrapper to handle caching, store-and-forward logic for inventory, and idempotency.
    - *Shopify POS*: Has deep offline capabilities, queueing transactions and syncing them when online. It leads in this space but is heavy and feels like a complex suite.
    - *Square*: Pioneered offline mode for POS, allowing merchants to take swiped/dipped payments without connectivity, assuming the risk for declined cards, but guaranteeing smooth operations.
  - **OHC Gap**: OHC requires a Tap-to-Pay integration within the Flutter PWA/Native app that allows taking payments and queuing operations locally, with our "Finance & Decision Assistant" agents handling background sync, conflict resolution, and ledger updates silently once back online.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Operator as Mobile App (Flutter)
      participant T2P as Tap-to-Pay SDK (Native)
      participant LocalDB as Local SQLite / Hive
      participant OHC as OHC Backend API
      participant Agent as Finance/Inventory Agent
      participant Stripe as Stripe Terminal API

      Operator->>T2P: Initiate Tap-to-Pay (Offline Mode Active)
      T2P-->>Operator: Tokenized Payment Intent (Stored Locally)
      Operator->>LocalDB: Save Transaction & Idempotency Key
      Operator-->>Operator: Update UI (Payment Pending Sync)

      Note over Operator, OHC: Network Restored

      Operator->>OHC: Sync queued transactions (Idempotency Keys)
      OHC->>Stripe: Process Payment Intents
      Stripe-->>OHC: Confirmations
      OHC->>Agent: Trigger Ledger & Inventory Reconciliation
      Agent-->>OHC: Resolution & Alerts for failed payments
      OHC-->>Operator: Sync complete, update UI to 'Paid'
  ```

  ### UI Wireframes & Screen Flow (375px)
  1. **Checkout Screen**: A clean UniFi-style card showing the order total and a large, prominent "Tap to Pay" button.
  2. **NFC Modal**: A native translucent glass overlay prompting the user to hold the card near the phone.
  3. **Offline Indicator**: A subtle, amber status token at the top reading "Offline: Queuing Payments".
  4. **Success State**: Immediate green confirmation "Payment Queued" if offline, or "Payment Successful" if online.
  5. **Sync Dashboard**: A background sync status bar that appears when connectivity is restored, turning green once the AI agent confirms ledger reconciliation.

  ### Mobile UX Flow
  - **Action**: Operator taps "Checkout".
  - **Condition**: Network is unavailable.
  - **Result**: App switches to Offline Tap-to-Pay mode. Payment is securely tokenized locally.
  - **Feedback**: Immediate haptic feedback and visual confirmation so the operator can serve the next customer instantly.
  - **Resolution**: Upon reconnection, transactions sync automatically. If a card is declined post-sync, the Finance Assistant drafts an SMS/Email to the customer (if known) or alerts the operator via the Work Triage feed.

  ### AI Agent Integration Points
  - **Operations Agent**: Temporarily reserves inventory based on queued offline transactions to prevent overselling on the digital storefront.
  - **Finance & Decision Assistant**: Monitors the sync queue. If a transaction fails to capture once online, it alerts the owner and drafts a recovery action. It also summarizes the day's offline vs. online sales in the daily brief.

  ### Key Design Decisions
  - **Local-First Persistence**: Using a robust local database (like Hive or SQLite in Flutter) to queue transactions guarantees no data loss during app crashes or OS suspends.
  - **Idempotency**: Strict use of UUIDs generated at the moment of the offline transaction to ensure Stripe does not double-charge upon sync.
  - **Owner Trust**: The UI must explicitly tell the owner that the payment is *queued* rather than fully *settled*, balancing operational speed with financial transparency.

  ## Implementation Prompt
  **To the Implementer**: Implement the offline-tolerant Tap-to-Pay queue architecture in the Flutter mobile application. Create a local transaction queuing service that intercepts payment intents when the device is offline. Design a background sync manager that flushes the queue to the `POST /api/payments/sync` endpoint using strict idempotency keys once connectivity is restored. Ensure the UI at 375px gracefully displays the "Offline / Queued" state using the OHC Premium Token translucent materials. Do not implement the Stripe SDK wrapper yet; focus on the robust queuing, state management, and API sync layer. Write comprehensive Playwright/Flutter widget tests to verify the offline-to-online transition and ensure no mock data is used in the final UI components.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
