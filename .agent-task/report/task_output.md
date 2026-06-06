# Architectural Design: Autonomous Subscription and Recurring Billing Engine

## 1. Problem Statement
Small businesses, such as a music tutor (Leo) or a home baker (Maya), need recurring subscription capabilities (lesson packages, monthly goods boxes) without complex configuration, webhooks, or third-party paid tools like ReCharge. Currently, setting up edge-cached, offline-first subscriptions seamlessly is difficult. A reliable system is required to handle billing gracefully even during offline states (e.g., student package redemption) and automate dunning workflows behind the scenes.

## 2. Proposed Architecture Overview
This solution proposes an embedded, offline-first subscription and recurring billing engine built on the following pillars:
1. **Offline-First Synchronization via CRDTs**: Using an embedded local database (SQLite/IndexedDB) on mobile devices for edge storage and Conflict-free Replicated Data Types (CRDTs) to ensure data synchronization and offline resolution.
2. **Autonomous Background Dunning (Finance Agent)**: An AI agent ("The Accountant") handles retry logic, dunning emails, and partial payment options entirely autonomously without manual configuration.
3. **Seamless Provider Integration**: Under-the-hood mapping to Stripe Billing/Payment Intents, abstracted completely from the user.

## 3. Detailed Component Design

### 3.1 Offline-First Local Store & CRDT Sync
- **Local Database (Edge)**: Mobile and web apps use an embedded SQLite database (via Flutter sqflite) or IndexedDB to maintain a cached state of customer subscriptions, remaining credits (e.g., Leo's lesson packages), and billing statuses.
- **CRDT Synchronization**:
  - To support offline validation (e.g., redeeming a lesson credit when offline in a studio), credit balances and redemption logs are modeled as CRDT counters/sets.
  - Changes are recorded locally and synced to the cloud via a background worker queue upon network restoration.
- **Sync Protocol**: The main backend server acts as the central source of truth, resolving CRDT states and persisting them to PostgreSQL.

### 3.2 Finance Agent (The Accountant)
- **Role**: Replaces manual subscription configuration. The user only needs to define the offer (e.g., "Monthly Guitar Lessons for $100").
- **Dunning Workflows**: When a recurring payment fails, the Finance Agent automatically:
  - Assesses the decline reason.
  - Schedules retries based on optimal timing rather than fixed intervals.
  - Drafts and sends personalized, empathetic follow-up emails (or SMS) to the customer.
- **Grace Periods & Downgrades**: The agent can temporarily authorize service access during grace periods and automatically pause/downgrade access if payment is ultimately not secured.

### 3.3 Backend Billing Engine
- **Data Model (PostgreSQL)**:
  - `SubscriptionPlan`: Base offer details.
  - `CustomerSubscription`: Tracks the lifecycle of a subscription.
  - `LedgerTransaction`: Immutable log of credits and debits for offline sync reconciliation.
- **Payment Abstraction**: The engine interfaces with Stripe Billing in the background, utilizing Stripe's retry logic where applicable but overriding it with the Finance Agent's intelligent logic when appropriate.

## 4. User Journeys

### Persona: Leo the Music Tutor (Service Packages)
1. Leo creates a "$200/month for 4 lessons" package.
2. A student subscribes. The backend provisions 4 "credits" to the student's account.
3. **Offline Scenario**: Leo is in a basement studio with no internet. After a lesson, he taps "redeem lesson" on his app. The local SQLite database decrements the credit locally via a CRDT operation.
4. When Leo regains connectivity, the app syncs the redemption event to the backend.

### Persona: Maya the Home Baker (Monthly Box)
1. Maya creates a "Monthly Cookie Box for $30".
2. A customer's card expires.
3. The Finance Agent detects the failure, pauses the fulfillment order automatically (notifying Maya's Operations Agent), and sends a polite SMS to the customer asking them to update their card.
4. Once updated, the charge processes, and the Operations Agent resumes fulfillment.

## 5. Security & Consistency Considerations
- **Event Sourcing**: All billing events and offline syncs are processed as an append-only event log to prevent race conditions.
- **Idempotency**: All network requests from the edge device use unique idempotency keys to handle intermittent connection drops safely.
- **Cryptographic Signatures**: Offline redemptions can optionally be cryptographically signed by the device to prevent tampering before sync.
