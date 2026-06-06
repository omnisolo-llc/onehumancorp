# Architecture Design: Autonomous Subscription and Recurring Billing Engine

## 1. Executive Summary

Small businesses—like Leo the Music Tutor offering monthly lesson packages, or Maya the Baker offering a "cake-of-the-month" box—require robust recurring billing. Existing solutions (e.g., Shopify + ReCharge) are complex, require manual configuration of webhooks, and often fail to support offline-first or localized scenarios.

This architectural design proposes an **Autonomous Subscription and Recurring Billing Engine** for OneHumanCorp (OHC). By leveraging local SQLite/IndexedDB stores for offline CRDT (Conflict-free Replicated Data Type) synchronization, the system allows for offline validation (such as a student redeeming a prepaid lesson package). Furthermore, the built-in Finance Agent handles dunning and retry workflows completely autonomously.

## 2. Target Personas & Use Cases

### Leo — The Music Tutor
* **Need**: Monthly lesson subscriptions (e.g., 4 lessons/month).
* **Pain Point**: Tracking which students have paid, tracking lesson redemption, and handling failed payments manually.
* **OHC Solution**:
  - Leo creates a "Monthly Guitar Masterclass" package.
  - The Finance Agent automatically charges students.
  - The offline CRDT sync ensures Leo can check student package balances even in a basement studio with no cellular service.
  - The Finance Agent autonomously emails students when their card expires and handles grace periods.

### Maya — The Home Baker
* **Need**: "Cake-of-the-month" subscription box.
* **Pain Point**: Coordinating production cycles with subscription billing dates.
* **OHC Solution**:
  - Maya sets up the recurring product.
  - The Finance Agent groups all monthly charges to trigger on the 1st of the month.
  - If a payment fails, the agent pauses the specific order and notifies the customer to update their payment method, without Maya needing to intervene.

## 3. System Architecture

The architecture consists of three main pillars: Offline-First CRDT Storage, the Autonomous Finance Agent (Dunning & Retry), and the Main Server API.

### 3.1 Offline-First CRDT Synchronization
To support offline validation (e.g., Leo checking if a student has an active subscription offline), subscription states must be replicated locally.

* **Client Storage (Flutter App / PWA)**:
  - Uses `sqflite` (Flutter Mobile/Desktop) and `IndexedDB` (Web) for local data persistence.
  - Implements a CRDT (Conflict-free Replicated Data Type) model for the `SubscriptionStatus` and `PackageRedemption` tables.
* **Data Model (Simplified)**:
  ```sql
  -- Local & Remote Schema
  CREATE TABLE subscriptions (
    id UUID PRIMARY KEY,
    tenant_id UUID,
    customer_id UUID,
    product_id UUID,
    status VARCHAR(50), -- ACTIVE, PAST_DUE, CANCELED
    current_period_end TIMESTAMP,
    vector_clock JSONB -- For CRDT resolution
  );

  CREATE TABLE package_redemptions (
    id UUID PRIMARY KEY,
    subscription_id UUID,
    redemption_date TIMESTAMP,
    status VARCHAR(50), -- PENDING, SYNCED
    vector_clock JSONB
  );
  ```
* **Sync Mechanism**:
  - Background sync queue in the mobile app pushes pending `package_redemptions` to the server when online.
  - Server responds with delta updates for `subscriptions`.

### 3.2 Autonomous Finance Agent (Dunning Workflows)
The Finance Agent handles the complexities of recurring billing invisibly.

* **Trigger**: A cron-based job queue built on PostgreSQL `SKIP LOCKED`.
* **Dunning Process**:
  1. **Payment Attempt**: Server attempts Stripe charge.
  2. **Failure Detected**: If it fails (e.g., insufficient funds), the job is routed to the Finance Agent.
  3. **Agent Action (Automated)**:
     - Day 1: Agent drafts and sends a friendly email/SMS: "Hey [Name], your card for the Monthly Guitar Masterclass couldn't be processed. Update it here: [Link]".
     - Day 3: Retry charge.
     - Day 5: Agent sends a final reminder.
     - Day 7: Agent pauses the subscription and updates the CRDT state, which syncs to Leo's device.
* **No Configuration Required**: The agent uses industry-standard dunning schedules by default. The business owner only receives a weekly report ("3 subscriptions recovered, 1 paused").

### 3.3 Main Server API & External Integration (Stripe)
* **Backend**: Go services expose gRPC/REST APIs for the client.
* **Payment Processor**: Integrates directly with Stripe Billing (or custom Stripe PaymentIntents for complex offline scenarios).
* **Webhooks**: Stripe webhooks update the central PostgreSQL database. The backend increments vector clocks and pushes updates to connected clients via WebSockets (or waits for the client to poll/sync).

## 4. Mobile-First & Glassmorphism UI Design

The interface must adhere to OHC's strict design principles: radically simple, mobile-first (375px), and visually premium.

### Subscription Dashboard (Owner View)
* **Visuals**: Glassmorphism (`backdrop-filter: blur(20px)`) over vibrant gradient backgrounds indicating overall health (e.g., Green for healthy MRR, Orange if many payments are failing).
* **Typography**: Outfit for headers, Inter for data.
* **Layout (375px)**:
  - Top: Large, clear Monthly Recurring Revenue (MRR) number.
  - Middle: A simple list of "Active Subscribers".
  - Bottom Action: "Create New Subscription Plan" (large 44x44px touch target).
* **Zero Jargon**: We don't use words like "Dunning", "Webhooks", or "CRDT". We say: "We'll handle failed payments for you."

## 5. Security & Isolation
* **Tenant Isolation**: Row-Level Security (RLS) on PostgreSQL ensures tenants cannot access other tenants' subscription data.
* **Offline Security**: Local SQLite/IndexedDB data is encrypted at rest where supported by the OS (e.g., iOS Data Protection).

## 6. Implementation Plan
1. **Phase 1: CRDT Foundation**: Implement the local SQLite/IndexedDB schema and the sync protocol with the Go backend.
2. **Phase 2: Stripe Integration**: Connect Stripe Billing webhooks to update the central database.
3. **Phase 3: Autonomous Agent**: Implement the Finance Agent's dunning workflow logic.
4. **Phase 4: UI/UX**: Build the mobile-first Flutter interface using the Glassmorphism design system.

## 7. Interoperability (Cloud vs. Standalone)
As an explicit requirement for the OHC Swarm, the Main Server and Builtin Agent Microservice must stay in sync whether the owner is online (Cloud) or offline (Standalone mode). The Finance Agent's state machine handles network partitioning by locally logging dunning actions to the SQLite database and gracefully catching up with the Main Server’s `SKIP LOCKED` job queues once connectivity is restored.

Distributed locking logic via Redlock (Cloud) gracefully downgrades to file-based local advisory locks (Standalone) when executing billing actions, ensuring that even if a network switch occurs, we do not double-bill a customer.
