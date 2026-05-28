issue_title: "Implement High-Scale Tap-to-Pay Terminal SDK Integration for Offline Point-of-Sale (POS)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) users, such as Priya (boutique owner) and Fatima (food cart), lack the ability to seamlessly accept in-person payments directly through the platform. Currently, they likely need to rely on external hardware and separate accounting software. This creates friction, fragmented data, and a barrier to seamless omnichannel growth, a critical requirement for a truly hybrid physical-digital business.

  ## Research Report
  - **Market Context**: Competitors like Shopify (Shopify POS) and Stripe (Stripe Terminal) offer robust SDKs for mobile tap-to-pay and terminal integrations. These systems are foundational for businesses operating in both physical and digital spaces.
  - **User Needs**:
    - **Priya**: Needs to take card payments in her boutique via tap-to-pay on her phone or a dedicated terminal, with instant inventory sync to her online storefront.
    - **Fatima**: Needs to rapidly process in-person orders at her food cart, ideally using her existing Android device, ensuring high availability even in spotty network conditions.
  - **Current Gap**: OHC currently lacks a unified, resilient architecture for handling in-person Point-of-Sale transactions, especially offline-capable tap-to-pay on mobile devices.
  - **Strategic Value**: Enabling this unlocks massive growth for physical-product and service-based businesses, solidifying OHC as a single source of truth for all business operations.

  ## Design Doc

  ### Business Journey Mapping (AARRR)
  - **Acquisition**: Market OHC as a complete solution replacing Square or Shopify POS, targeting physical retailers and service providers.
  - **Onboarding**: Simple, one-tap enable of the "Tap to Pay on Phone" feature within the OHC app. No hardware to order.
  - **Activation**: User successfully processes their first test or real transaction in under 2 minutes.
  - **Retention**: Providing offline resilience guarantees (e.g., Fatima selling food during a local event with no cell service).
  - **Revenue**: Transaction fees generated directly through OHC processing volume.
  - **Referral**: Customers experiencing the seamless OHC checkout flow ask the merchant about the platform.

  ### Data Model & Invariants
  ```mermaid
  erDiagram
      Tenant ||--o{ TerminalSession : owns
      TerminalSession ||--o{ Transaction : processes
      Transaction ||--|| OfflineQueue : queued_in
      Transaction }o--|| Ledger : reconciled_to

      TerminalSession {
          string id PK
          string tenant_id FK
          string device_id
          string status
          timestamp last_sync
      }
      Transaction {
          string id PK
          string session_id FK
          decimal amount
          string status
          string receipt_email
      }
      OfflineQueue {
          string id PK
          string transaction_id FK
          string payload_signature
          int retry_count
      }
      Ledger {
          string id PK
          string tenant_id FK
          decimal balance
      }
  ```
  - **Invariants**:
      - Every `TerminalSession` must be strictly scoped to a single `tenant_id`.
      - Transactions processed offline are stored in `OfflineQueue` using an append-only, signed payload structure to prevent tampering.

  ### Zero Trust & Security
  - **Identity (SPIFFE/SPIRE)**: The Terminal SDK must authenticate via short-lived JWTs mapped to the tenant’s OIDC context. Any background syncing agents communicating with payment gateways will use SPIFFE/SPIRE mTLS to guarantee inter-workload identity and prevent unauthorized internal service calls.
  - **Multi-tenant Isolation**: All sync APIs and cache entries must strictly enforce row-level security or programmatic checks requiring the `organization_id` matching the authenticated tenant.

  ### Performance & Offline Targets
  - **Latency**: Online transactions must authorize in < 1500ms (p95).
  - **Offline Capability**: Capable of queuing up to 500 transactions locally.
  - **Payload Targets**: Sync payloads must be compressed to < 5KB per transaction.
  - **Resilience**: The Sync Engine must implement exponential backoff with jitter.

  ### Architecture Flow (Mermaid.js)
  ```mermaid
  graph TD
      MobileApp[Mobile App - OHC] --> TerminalSDK[Tap-to-Pay / Terminal SDK]
      TerminalSDK --> LocalCache[(Local Cache / Offline Queue)]
      MobileApp --> SyncEngine[Sync Engine]
      SyncEngine --> OHC_API[OHC Core API]
      OHC_API --> PaymentGateway[Payment Gateway Integration]
      OHC_API --> Ledger[(Ledger / Inventory DB)]

      subgraph Background Agents
          OHC_API --> FinanceAgent[Finance Agent - Reconciliation]
          OHC_API --> MarketingAgent[Marketing Agent - Customer Follow-up]
      end
  ```

  ### Mobile UX Flow (375px)
  1.  **Checkout Screen**: Clear, large display of the total amount. A prominent "Charge" or "Tap to Pay" button, adopting macOS-style translucent glass cards for order items.
  2.  **Payment Screen**: The screen transitions to the native OS tap-to-pay interface (e.g., Apple Tap to Pay on iPhone or Android equivalent), maintaining a secure, familiar look.
  3.  **Processing**: A brief, smooth animation indicating transaction processing.
  4.  **Success Screen**: A clear "Payment Successful" confirmation, with options to text or email the receipt to the customer. Clean, modular layout ensuring a non-technical user completes this in 30 seconds.

  ### Key Design Decisions
  -   **Offline First**: The SDK integration must gracefully handle network interruptions, queuing transactions securely for sync when connectivity is restored, crucial for users like Fatima.
  -   **Hardware Agnostic (where possible)**: Prioritize software-based "Tap to Pay on Phone" capabilities before introducing proprietary hardware dependencies, keeping the barrier to entry low.
  -   **Agent Integration**:
      -   **Finance Agent**: Automatically reconciles terminal payouts with the general ledger.
      -   **Marketing Agent**: Can trigger a post-purchase review request if the customer opted-in for a digital receipt.

  ## Implementation Prompt
  Implement the backend infrastructure and frontend SDK integration necessary to support in-person Point-of-Sale (POS) transactions. This includes defining the data models for Terminal Sessions, Offline Transaction Queues, and the API endpoints for processing and syncing these transactions with the core OHC ledger. Ensure the mobile checkout flow for Tap-to-Pay is seamless, robust against poor network conditions, and properly integrated with the Finance AI agent for automatic reconciliation.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
