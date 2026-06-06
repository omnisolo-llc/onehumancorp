# Autonomous Subscription and Recurring Billing Engine Architecture

## 1. Problem Statement & Context

Small businesses, such as Leo the Music Tutor and Maya the Baker, require robust recurring billing capabilities (e.g., monthly lesson packages or recurring custom cake boxes) without the complexity of traditional enterprise setups. Existing solutions demand manual webhook configurations, complex third-party tools (like ReCharge), and persistent internet connections for validation, which can fail in mobile-first environments.

This document outlines the architectural design for an autonomous subscription and recurring billing engine built into the OneHumanCorp platform. The engine leverages offline-first data sync on mobile clients and autonomous dunning driven by the AI Finance Agent, effectively bridging the gap between sophisticated billing logic and zero-configuration usability.

## 2. Personas & Core Workflows

### Leo the Music Tutor
- **Use Case:** Monthly lesson packages.
- **Workflow:** A student subscribes to a 4-lesson-per-month package. The engine automatically bills them monthly and grants 4 "lesson credits." Leo redeems these credits offline during the lesson via his mobile app. The redemption is validated locally and synced back when online.
- **AI Assist:** The Finance Agent detects upcoming package renewals and sends automated SMS reminders; if a payment fails, it handles dunning gracefully by sending customized "please update your payment method" messages.

### Maya the Baker
- **Use Case:** Recurring "Cake of the Month" box.
- **Workflow:** Customers subscribe to a monthly cake box. Maya's Operations Agent tracks fulfillment while the Finance Agent manages recurring billing. If Maya's delivery location has spotty cellular service, she can still mark boxes as delivered/picked up, with background sync updating the cloud state later.

## 3. Architecture Design Overview

The proposed architecture relies on a hybrid cloud/edge data model and an AI-driven async job pipeline.

### A. Offline-First CRDT Data Sync (Mobile App)

To solve the offline redemption/validation problem, the mobile app (Tauri/Flutter) will utilize an embedded edge database.

- **Data Store:** SQLite (desktop/native mobile) / IndexedDB (web/PWA).
- **Synchronization Model:** Conflict-free Replicated Data Types (CRDTs).
  - Subscription states (Active, Past Due, Canceled) and "Credit/Entitlement" balances are synchronized between the cloud (PostgreSQL) and the edge client.
  - **Redemption Protocol:** When Leo redeems a lesson credit offline, the local database decrements the credit and logs an event. The UI updates instantly. Upon reconnection, the event log syncs to the cloud, merging the state via CRDT logic to ensure no double-spending of credits.
- **Security:** Local storage is encrypted. Validations involve offline cryptographic signatures appended to entitlement records by the cloud.

### B. AI Finance Agent Dunning Workflow

Traditional platforms require complex rule engines for dunning (retrying failed payments). OHC offloads this to the AI Finance Agent, turning a rigid state machine into an intelligent, conversational workflow.

- **Trigger:** A Stripe recurring payment webhook reports `invoice.payment_failed`.
- **Job Queue:** The event is pushed into the `ai_finance_jobs` PostgreSQL queue.
- **Agent Action:**
  1. The Finance Agent retrieves the customer's profile and interaction history from pgvector memory.
  2. The agent determines the appropriate tone (e.g., gentle reminder for a 1st failure vs. firmer notice for a 3rd failure).
  3. It drafts and dispatches an SMS or email via the Communication service containing a direct Stripe "Update Payment Method" link.
  4. The agent schedules a follow-up job in the queue to check the status in 48 hours.

### C. Backend Architecture (Rust/PostgreSQL)

- **Subscription Data Model:**
  - `subscriptions` table: Tracks plan, current term, status, Stripe subscription ID.
  - `entitlements` table: Tracks consumable credits (e.g., 4 lessons) tied to the subscription.
  - `subscription_events` table: An append-only event log used to compute CRDT state and provide an audit trail.
- **Multi-Tenancy:** All tables include `tenant_id` and enforce Row Level Security (RLS) to ensure Maya cannot see Leo's subscription data.
- **Payment Processing:**
  - Integration with Stripe Billing API.
  - Idempotent webhook handlers process `invoice.paid`, `invoice.payment_failed`, and `customer.subscription.updated` events.

## 4. Execution Pipeline & Dependencies

1. **Phase 1: Foundation (Data Layer)**
   - Design the schema for `subscriptions`, `entitlements`, and the `subscription_events` event log in PostgreSQL.
   - Implement the corresponding Rust service layer with gRPC/REST endpoints, ensuring multi-tenant RLS.

2. **Phase 2: Offline-First Edge Sync**
   - Implement the local SQLite/IndexedDB store on the Tauri client.
   - Develop the CRDT sync protocol to handle offline credit redemption and merge conflicts gracefully.

3. **Phase 3: Stripe Integration & Webhooks**
   - Implement Stripe Billing endpoints for creating plans and initiating subscriptions.
   - Build idempotent, signature-verified webhook handlers to capture billing lifecycle events.

4. **Phase 4: AI Finance Agent Dunning**
   - Implement the job dispatch mechanism triggered by failed payment webhooks.
   - Update the Finance Agent's system prompt and toolset to draft personalized dunning messages and schedule follow-ups.

5. **Phase 5: UI & End-to-End Testing**
   - Create the mobile-first UI for Leo/Maya to define subscription packages and view subscriber status.
   - Build the customer-facing UI for purchasing and managing subscriptions.
   - Write comprehensive unit tests and Playwright E2E tests validating the full flow (online purchase -> offline redemption -> sync -> automated dunning).

## 6. Real UI Gaps & Missing Endpoints

While using the real product via browser UI on `http://localhost:3000`, attempting to configure a subscription failed because the Next.js routes (`/api/subscriptions`) do not have a full, resilient backend integration. We also noted that the offline-first sync (Phase 2) is crucial since the UI hangs or throws unhandled promise rejections when the network is throttled or disconnected during subscription setup.

## 7. Security & Observability

- **Security:** Strict tenant isolation via database RLS. Webhook signatures validated cryptographically. Local mobile databases encrypted. Idempotency keys used for all mutations.
- **Observability:** OpenTelemetry traces span from the mobile client event, through the Rust API, into the Stripe integration and AI agent execution. Prometheus metrics track active subscriptions, failed payment rates, and sync conflict resolutions. Grafana dashboards expose these metrics in plain language for platform monitoring.
