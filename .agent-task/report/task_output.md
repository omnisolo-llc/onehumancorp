# Architectural Design: Autonomous Subscription and Recurring Billing Engine

## 1. Problem Statement
Small businesses need recurring subscription capabilities (lesson packages, monthly goods boxes) without complex configuration, webhooks, or third-party paid tools (like ReCharge). Currently, setting up edge-cached, offline-first subscriptions seamlessly is difficult.

**Persona Identified**: Leo the Music Tutor, Maya the Baker

## 2. Proposed Solution
A design that utilizes an SQLite/IndexedDB store on mobile devices for offline-first CRDT synchronization, enabling offline validation (e.g., student package redemption). Background AI agents (Finance Agent) handle dunning workflows autonomously and without manual setup.

## 3. Architecture

### 3.1. Edge-Cached, Offline-First Subscriptions
- Use an SQLite/IndexedDB store on the mobile device for offline-first capabilities.
- Synchronize data using CRDT (Conflict-free Replicated Data Type) to resolve conflicts and ensure data consistency.
- Subscriptions and user limits (e.g. remaining lesson packages) are cached locally.
- When offline, packages can be validated locally (e.g., student package redemption).

### 3.2. Background AI Agent (Finance Agent)
- Runs autonomously in the background.
- Automatically handles dunning workflows, like tracking unpaid subscriptions and following up with students/customers.
- Integrates tightly with Stripe Billing or local billing for payment intent creation.
- Doesn't require manual setup.

## 4. Workflows

### 4.1 Subscription Setup
1. Leo creates a Monthly Guitar Lesson package in the OHC app.
2. The Finance Agent registers the product and pricing within the backend (e.g., via Stripe API).
3. The offline-first store replicates the product details to Leo's mobile app.

### 4.2 Offline Package Redemption
1. A student arrives for a lesson and Leo is offline (no network).
2. Leo marks the lesson as redeemed in the OHC mobile app.
3. The app updates the local CRDT store, reducing the student's remaining package count.
4. When network connectivity is restored, the mobile app syncs the CRDT delta back to the server.

### 4.3 Autonomous Dunning Workflow
1. The Finance Agent detects a failed recurring payment or an impending subscription renewal.
2. The Agent autonomously drafts and sends an email/SMS reminder to the customer.
3. The Agent updates the local and server state regarding the payment failure.
4. Leo receives a brief summary notification about the handled failure.

## 5. Security & Isolation
- Data synchronized via CRDT is cryptographically signed to prevent tampering during offline edits.
- Tenant isolation is enforced at the database level (`ENABLE ROW LEVEL SECURITY`).
- All webhooks are validated securely.
