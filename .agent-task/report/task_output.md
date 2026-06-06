# Research: Autonomous Subscription and Recurring Billing Engine

## 1. Executive Summary
This document proposes the architectural design for an Autonomous Subscription and Recurring Billing Engine for OneHumanCorp (OHC). Designed for everyday business owners like Leo (Music Tutor) and Maya (Baker), it abstracts away all complex configuration, webhooks, and third-party tools. It leverages an offline-first CRDT synchronization model (SQLite/IndexedDB) for edge-cached mobile operations and uses the Finance AI Agent for fully autonomous dunning and billing management.

## 2. Identified Personas
- **Leo the Music Tutor**: Needs subscription-based pricing for monthly lesson packages. Needs to validate student packages even if internet connectivity drops temporarily (e.g., in a basement studio).
- **Maya the Baker**: Needs recurring orders for her "Cake of the Month" club. Wants hands-off payment collection and dunning.

## 3. Core Architecture

### 3.1. Offline-First Storage & CRDT Sync
- **Mobile Persistence**: IndexedDB (for Web/PWA) and SQLite (for Flutter/Mobile Apps).
- **Data Model**: Subscription entitlements and billing periods are represented as Conflict-Free Replicated Data Types (CRDTs). This allows offline validation (e.g., a student redeeming a lesson) and synchronizes transparently when the device regains connectivity.
- **Offline Validation**: The client stores cryptographic proofs of active subscription periods locally. The Finance Agent pushes down "entitlement tokens" that expire at the end of the billing cycle.

### 3.2. Autonomous Finance Agent (The Accountant)
- **Billing Execution**: The background AI agent handles the entire subscription lifecycle: creating invoices, capturing funds, and issuing receipts.
- **Autonomous Dunning**: Instead of static rules (like "retry on day 3"), the Finance Agent intelligently orchestrates payment retries based on historical success rates, time of day, and personalized customer interaction (e.g., drafting a polite email if a card fails).
- **No Webhooks Required**: Because the Finance Agent continuously polls and responds to the job queue (using PostgreSQL `SKIP LOCKED` pattern), the system does not require users to configure webhooks or integrate with Zapier.

### 3.3. Edge-Cached Delivery
- **Entitlement Delivery**: Active subscription statuses are cached at the edge (CDN/Redis).
- **Latency Optimization**: Validation requests ping the nearest edge node, falling back to local SQLite/IndexedDB if offline.

## 4. User Experience (Zero Setup)
1. **Creation**: Leo types "Create a monthly guitar lesson package for $100". The Operations Agent builds the product, and the Finance Agent creates the subscription tier.
2. **Execution**: The system automatically charges the student each month. If a charge fails, the Finance Agent autonomously contacts the student.
3. **Redemption**: Leo opens the OHC app offline in his studio. The app instantly verifies the student's active subscription using the local CRDT store.

## 5. Security & Compliance
- **PCI-DSS Compliance**: All cardholder data is securely vaulted with Stripe (Billing/Payment Intents). OHC only stores reference tokens and entitlement CRDTs.
- **Audit Logs**: Every dunning action or entitlement change is durably recorded for transparency and compliance reporting by the Legal & Compliance Agent.
