issue_title: "[architecture] Mobile-First Tap-To-Pay Offline Protocol"
issue_description: |
  **Problem Statement**
  Users like Priya the Boutique Owner and Fatima the Food Cart Operator need to accept in-person payments securely. The current documentation outlines "Stripe Terminal (in-person POS)", but lacks a designed architecture for how OHC gracefully handles the critical failure state: network drops during an in-person payment sequence. Non-technical users lose sales and trust if the app just shows a generic error when their phone loses 4G.

  **Research Report**
  Leading platforms like Square and Stripe Terminal provide SDKs, but the implementation logic within the host app dictates the UX during partial outages.
  - OHC needs an architecture that handles offline queuing of signed transaction intents, syncing them securely when connectivity is restored, while providing immediate, confident feedback to the business owner and the customer.
  - By deeply integrating a local SQLite caching layer within the Flutter client (Riverpod state management) and defining a strict synchronization protocol with the Go/Postgres backend, we can provide a zero-downtime perception.

  **Design Doc**
  - **Architecture diagram:**
    ```mermaid
    graph TD;
      MobileClient[Flutter App 375px] -->|Intent| LocalDB[Local SQLite Cache];
      LocalDB -->|Network Check| SyncEngine[Sync Engine];
      SyncEngine -- Online --> StripeTerminal[Stripe Terminal SDK];
      StripeTerminal --> OHCBackend[OHC API - Go];
      SyncEngine -- Offline --> WaitState[Enqueue & Display 'Pending Sync'];
      WaitState -. Reconnect .-> StripeTerminal;
    ```
  - **Mobile UX Flow:** If offline during a swipe/tap, the app flashes a reassuring translucent glass card: "Payment securely captured. Will sync when online." The user can continue to accept the next order without being blocked.
  - **AI Agent Integration:** Finance & Payments agent monitors the offline queue. If a transaction has been queued for >2 hours without syncing, it pushes a simple notification: "You have 3 payments waiting. Open the app when you have Wi-Fi to secure your funds."
  - **Key design decisions:**
    1. Idempotency keys generated on the client and stored in the local SQLite cache to prevent double-charging on reconnect.
    2. Zero Trust: The app does not store raw PANs, only Stripe tokenized intents.

  **Implementation Prompt**
  Implement the offline-first tap-to-pay synchronization protocol in both the Flutter client and the Go backend.
  - **CUJ:** Priya takes a $50 payment in her boutique. Her internet drops. The app securely queues the payment. 10 minutes later, internet returns, and the payment automatically syncs and clears without Priya doing anything.
  - **Acceptance Criteria:**
    - Flutter client must generate and persist UUIDv4 idempotency keys locally before initiating a charge.
    - Go backend must handle idempotency gracefully (returning success if already processed).
    - AI Finance agent must have a recurring check for stale offline queues (simulated via backend flag).
    - 100% unit test coverage for the sync engine logic.
  **Priority**: P1
  **Estimated Scope**: Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
