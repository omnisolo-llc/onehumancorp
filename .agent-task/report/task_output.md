# Architectural Design: Autonomous Subscription and Recurring Billing Engine

## 1. Executive Summary

**Topic**: Autonomous Subscription and Recurring Billing Engine
**Personas Identified**: Leo the Music Tutor, Maya the Baker

**Problem Statement**:
Small businesses need recurring subscription capabilities (e.g., lesson packages, monthly goods boxes) without complex configuration, webhooks, or expensive third-party paid tools (like ReCharge). Setting up edge-cached, offline-first subscriptions seamlessly is currently difficult and poses a major barrier to non-technical business owners.

**Proposed Solution**:
A native OHC architectural design that utilizes an SQLite/IndexedDB store on mobile devices for offline-first CRDT (Conflict-Free Replicated Data Type) synchronization, enabling offline validation (e.g., student package redemption in a location with poor network connectivity). Background AI agents (specifically the Finance & Payments Agent) handle dunning workflows (retrying failed payments, sending reminders) autonomously and without manual setup.

## 2. Core Architecture

The architecture is designed to be mobile-first, edge-cached, and fully autonomous. It removes the need for webhooks and third-party integrations by building recurring billing directly into the OHC platform.

### 2.1 Offline-First Edge Synchronization (Mobile/Web Client)
- **Local Storage**: IndexedDB (Web/PWA) and SQLite (Native Flutter via sqflite) store subscription entitlements, package balances, and customer statuses locally.
- **CRDT Sync Engine**: Implements Conflict-Free Replicated Data Types (CRDTs) to allow offline mutations (e.g., Leo redeeming a student's lesson package in a basement studio with no cellular service).
- **Optimistic UI Updates**: All actions in the app update the local data store instantly. Network requests are queued and synchronized in the background when connectivity is restored, ensuring a zero-latency feel.

### 2.2 Backend & Data Model (Go + PostgreSQL)
- **Tenant Isolation**: Row-Level Security (RLS) is applied to all subscription and billing tables (`tenant_id`).
- **Subscription Engine**: A centralized engine running on Go orchestrates the billing cycles, entitlement granting, and package expirations.
- **Stripe Integration**: Utilizes Stripe Billing internally, but abstracts all complexity from the user. OHC syncs Stripe webhook events to internal subscription states without requiring the business owner to configure anything.
- **Idempotency**: All billing actions use strict idempotency keys to prevent double-charging or double-redeeming.

## 3. The Autonomous Finance Agent (Dunning & Management)

The AI Finance & Payments agent ("The Accountant") handles the complex edge cases of recurring billing invisibly:
- **Smart Dunning**: When a subscription payment fails, the Finance Agent automatically kicks off a recovery workflow. It analyzes the failure reason (e.g., insufficient funds vs. expired card) and sends a dynamically generated, friendly reminder to the customer via Email or SMS, optimized for the highest conversion time.
- **Churn Prevention**: The agent identifies customers who haven't utilized their subscription recently (e.g., a student who hasn't booked a lesson with Leo) and suggests outreach to the business owner, or auto-sends an engagement message.
- **Zero-Config Setup**: When Maya creates a "Monthly Cake Box", the Finance Agent auto-generates the pricing model, sets up the Stripe subscription product in the background, and generates the Terms of Service.

## 4. Persona Workflows

### Leo the Music Tutor (Lesson Packages)
1. **Creation**: Leo creates a "4 Lessons/Month" package.
2. **Sale**: A student purchases it. The subscription is tracked.
3. **Redemption (Offline)**: Leo travels to a student's house with poor cell service. He taps "Redeem Lesson" in the OHC app. The SQLite local store instantly updates the balance (CRDT) and reflects the change.
4. **Sync**: When Leo gets back to Wi-Fi, the app silently syncs the redemption to the Go backend.

### Maya the Baker (Monthly Goods Box)
1. **Creation**: Maya sets up a "Cake of the Month" subscription.
2. **Billing**: Customers are billed on the 1st of every month.
3. **Failed Payment**: A customer's card expires. The Finance Agent catches the failure, emails the customer a secure link to update their payment method, and pauses their fulfillment order—all without Maya having to click a single button.
4. **Resumption**: Once updated, the agent reactivates the subscription and pushes the order to Maya's queue.

## 5. Security & Observability
- **Encryption**: All offline local data is encrypted at rest on the mobile device.
- **OpenTelemetry**: The billing engine emits distributed traces for every subscription cycle, allowing developers to trace a charge from Stripe webhook -> Go Backend -> AI Finance Agent -> Push Notification.
- **Metrics**: Prometheus metrics track subscription MRR, dunning success rates, and offline CRDT conflict resolution rates.

## 6. Conclusion
By embedding subscription management deeply into the client-side local database and handing the complexity of billing retries to the AI Finance Agent, OHC delivers an enterprise-grade recurring revenue engine that requires zero technical knowledge to operate. This cements OHC's position as the premier platform for service and product-based small businesses.
