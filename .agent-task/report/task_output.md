# Architectural Design: Autonomous Subscription and Recurring Billing Engine

## 1. Introduction

This document details the architectural design for the Autonomous Subscription and Recurring Billing Engine for OneHumanCorp (OHC). The primary objective is to empower non-technical small business owners, specifically personas like Leo the Music Tutor and Maya the Baker, to seamlessly establish and manage predictable recurring revenue streams without manual intervention or technical overhead.

This design addresses the critical gap where current solutions (e.g., third-party apps on Shopify or complex logic builders) require high technical acumen and disrupt native workflows. OHC's approach integrates subscriptions directly into the core product catalog and Stripe Billing, managed entirely by the platform's embedded AI agents.

## 2. Target Personas and Critical User Journeys (CUJs)

### 2.1. Leo - The Music Tutor
**Context:** Leo offers monthly guitar lesson packages.
**Pain Point:** Managing subscription billing manually and following up with students who fail to pay or whose cards are declined.
**CUJ:**
1. Leo enables "Subscription Billing" for his "Monthly Guitar Lessons" service.
2. A student subscribes online.
3. The Finance Agent automatically processes the monthly charge via Stripe Billing.
4. If a payment fails, the Finance Agent automatically initiates the dunning workflow, contacting the student to update their payment method, without Leo needing to intervene.

### 2.2. Maya - The Home Baker
**Context:** Maya sells weekly bread box subscriptions.
**Pain Point:** Tracking who has paid, managing delivery schedules, and handling customer requests to pause or skip a delivery week while she is busy baking.
**CUJ:**
1. Maya toggles "Subscribe & Save" on her "Weekly Sourdough Box" product.
2. A customer subscribes.
3. The Operations Agent adds the order to Maya's weekly fulfillment queue.
4. The customer texts, "Can I skip next week's box?" The Customer Success Agent (The Ambassador) understands the intent, pauses the subscription for one cycle in the CRDT-synced backend, and confirms with the customer.

## 3. Core Architecture

The architecture relies on an offline-first strategy, ensuring high availability and seamless user experience, even on unreliable networks (common for mobile-first users).

### 3.1. Offline-First CRDT Synchronization
To support the mobile-first requirement and ensure offline resilience, the engine utilizes Conflict-Free Replicated Data Types (CRDTs).

- **Local Storage:** The mobile/desktop clients (Tauri/Flutter) use a local SQLite database (for complex structured data) and IndexedDB (for web/PWA clients) to store subscription state, product catalogs, and recent billing events.
- **CRDT Sync Protocol:**
  - When Leo or Maya modifies a subscription setting offline, the change is recorded locally as a CRDT operation.
  - Upon network restoration, the client syncs these operations with the centralized PostgreSQL ledger.
  - CRDTs ensure that concurrent modifications (e.g., a customer pausing a subscription online while the owner edits the product offline) are resolved deterministically without data loss.

### 3.2. Autonomous Dunning Workflows (Finance Agent)
The Finance Agent ("The Accountant") takes full responsibility for the lifecycle of recurring payments, explicitly handling failures (dunning).

- **Event Ingestion:** The backend listens to Stripe webhooks (`invoice.payment_failed`, `customer.subscription.deleted`, etc.).
- **Dunning Orchestration:**
  - Upon a failed payment, the Finance Agent evaluates the customer's history.
  - It triggers personalized communication (via The Ambassador) to the customer (e.g., "Hi [Name], your payment for [Service] failed. Please update your card here: [Link]").
  - The agent manages a retry schedule (e.g., Day 1, Day 3, Day 7) based on OHC best practices, completely invisible to the business owner unless intervention is strictly necessary.
  - If the final retry fails, the agent automatically pauses the service delivery in the Operations Agent's queue and notifies the owner with a plain-language summary.

### 3.3. Integration with Existing Systems
- **Stripe Billing:** Acts as the underlying payment processor. OHC abstractions simplify the configuration, mapping simple toggles to complex Stripe Subscription logic.
- **Operations Agent ("The Manager"):** Consumes successful billing events to generate fulfillment tasks (Maya's baking queue) or confirm bookings (Leo's calendar).

## 4. Implementation Strategy

1. **CRDT Foundation:** Implement the CRDT sync layer using powersync (as indicated by the existing `docker-compose.yml` profile) to connect the local SQLite/IndexedDB stores with PostgreSQL.
2. **Data Model Updates:** Extend the PostgreSQL schema to support subscription configurations (`Delivery Frequency`, `Discount %`) on products and user-specific subscription states.
3. **Agent Capabilities:**
   - Enhance the Finance Agent's system prompt and tools to handle Stripe webhook events and manage the dunning state machine.
   - Extend the Customer Success Agent to interpret natural language requests for subscription management (pause, skip, cancel).
4. **Mobile UX:** Develop the simple "Subscribe & Save" toggle interface in the mobile-first UI, ensuring all touch targets are ≥ 44x44px and fit comfortably on a 375px screen.

## 5. Security & Isolation
- **Row-Level Security (RLS):** All subscription and billing data is strictly isolated by `tenant_id` in PostgreSQL.
- **Idempotency:** All interactions with Stripe Billing use idempotency keys to prevent double-charging during network retries or offline sync resolutions.