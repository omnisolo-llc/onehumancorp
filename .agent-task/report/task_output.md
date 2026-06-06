# Architectural Design: Autonomous Subscription & Recurring Billing Engine

## 1. Overview
The Autonomous Subscription and Recurring Billing Engine allows non-technical business owners like Leo (Music Tutor) and Maya (Baker) to offer subscription plans (e.g., monthly lesson packages, recurring goods boxes) without any configuration. By leveraging offline-first CRDT (Conflict-free Replicated Data Type) synchronization and an AI Finance Agent, the engine ensures resilient access to subscription validation and autonomous management of dunning, renewals, and invoice generation.

## 2. Personas & Use Cases

### Leo (Music Tutor)
- **Goal**: Offer monthly lesson packages to students, automatically track used lessons, and auto-renew packages without administrative overhead.
- **Problem**: Manually managing who paid for lessons, validating unused lessons on-the-go without internet, and chasing down failed payments takes away teaching time.
- **Solution**: The engine syncs subscription entitlements to Leo's mobile app (SQLite/IndexedDB). When a student arrives, Leo can validate a lesson redemption completely offline. The Finance AI Agent handles automatic dunning if a student's card fails.

### Maya (Baker)
- **Goal**: Provide a "Cake of the Month" subscription for local customers.
- **Problem**: Maya has spotty Wi-Fi in her kitchen and at farmers' markets. She needs a way to confirm active subscribers during pickup.
- **Solution**: Maya's device locally stores an active subscriber list utilizing CRDTs to ensure offline reads and queued writes. Subscribers' recurring payments are collected in the background, and she simply sees a unified order fulfillment list.

## 3. Core Architecture

The architecture consists of three primary layers:

### 3.1 Mobile / Edge Layer (Offline-First Store)
- **Storage**: Flutter app uses SQLite for native builds and IndexedDB for the Web/PWA, enabling disconnected operation.
- **Data Model (CRDTs)**:
  - `SubscriptionEntitlement`: Represents the active plan and its balances (e.g., 4 lessons remaining).
  - `RedemptionEvent`: Represents the usage of an entitlement. Handled as an append-only event log.
- **Sync Mechanism**: Uses a bidirectional background sync when online. Writes (like redemption events) are added to a local queue and synced via conflict-free event logs. Reads (validating if a subscription is active) are instantly available from the local store.

### 3.2 Backend Layer (Rust / Go / Postgres)
- **Database Model (PostgreSQL + Row-Level Security)**:
  - `tenant_subscriptions`
  - `tenant_subscription_events`
- **State Machine**: The subscription lifecycle (Active, Past Due, Canceled) is modeled as a state machine.
- **Payments**: Tightly integrated with Stripe Billing (or internal Payment Intents if avoiding external subscriptions). Stripe Webhooks hit the backend to update subscription status, which is then fanned out to mobile devices via sync.

### 3.3 AI Automation Layer (Finance Agent)
- **Agent Responsibility (The Accountant)**: The Finance AI Agent autonomously handles edge cases without owner intervention.
- **Dunning Management**:
  - Automatically drafts and sends friendly reminder emails/SMS to customers whose payments failed.
  - Generates plain-language alerts for the business owner: "Leo, 3 students' cards failed this week. I've already sent them update links."
- **Insight Generation**: Evaluates subscription churn and offers recommendations ("Maya, customers are canceling at month 3. Let's offer a 10% discount on month 4!").

## 4. Technical Workflows

### 4.1 Offline Validation & Redemption
1. **Sync**: While online, the app downloads all active `SubscriptionEntitlement` records for the tenant.
2. **Action**: User (Leo/Maya) redeems an entitlement while offline.
3. **Local Write**: The app records a `RedemptionEvent` in the local CRDT store and deducts the balance locally.
4. **Reconciliation**: When the device regains connectivity, the `RedemptionEvent` syncs to the server. If multiple devices redemptions cause an overdraft, the server resolves based on timestamps and alerts the owner.

### 4.2 Autonomous Dunning (Failed Payments)
1. **Trigger**: Backend receives a `invoice.payment_failed` webhook from Stripe.
2. **State Update**: `SubscriptionEntitlement` changes state to `past_due`.
3. **Agent Action**: The AI Finance Agent is invoked via the job queue (`SKIP LOCKED` pattern).
4. **Execution**: The Agent assesses the customer's history, drafts a personalized message with a secure payment update link, and sends it via email/SMS.
5. **Sync Down**: The updated `past_due` state is synchronized to the mobile client so the owner is aware.

## 5. Security & Edge Considerations
- **Data Pruning**: Local storage only keeps active and recently expired subscriptions to minimize footprint.
- **Conflict Resolution**: Utilizing LWW (Last-Write-Wins) for simple metadata and event sourcing for transactional operations (like redemptions) to ensure idempotency.
- **Tenant Isolation**: Row-Level Security ensures devices only sync data belonging to the authenticated `tenant_id`.

## 6. Development Strategy & Milestones
- **Milestone 1**: Implement the backend subscription models and basic Stripe Billing integration.
- **Milestone 2**: Build the local storage adapter in Flutter (SQLite/IndexedDB) and the CRDT event-sync protocol.
- **Milestone 3**: Wire the AI Finance Agent to listen for payment failure webhooks and autonomously dispatch dunning communications.
