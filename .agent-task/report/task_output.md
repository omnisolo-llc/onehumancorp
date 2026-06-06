# Architectural Design: Autonomous Subscription and Recurring Billing Engine

## 1. Executive Summary

Small businesses, ranging from tutors offering monthly lesson packages to bakers providing weekly pastry boxes, require robust recurring subscription capabilities. Traditional solutions like ReCharge involve complex configurations, webhooks, and high technical overhead. This document outlines the architecture for an **Autonomous Subscription and Recurring Billing Engine** integrated directly into the OneHumanCorp (OHC) platform. The engine is designed to be completely invisible to the end user (non-technical business owners like Leo the Music Tutor and Maya the Baker), leveraging edge-cached, offline-first architectures using SQLite/IndexedDB for CRDT synchronization, and fully autonomous dunning workflows managed by OHC's Finance AI Agent.

## 2. Personas and Use Cases

### 2.1 Leo the Music Tutor
**Need:** Leo sells monthly guitar lesson packages (e.g., 4 lessons/month). He needs students to subscribe once, have their payments auto-renew, and independently track how many lessons they have remaining.
**Offline Requirement:** When Leo is teaching in a soundproof studio with poor cellular service, he needs his app to instantly validate if a student has an active package to redeem.

### 2.2 Maya the Baker
**Need:** Maya offers a "Cupcake of the Month" subscription box. She needs automatic charge generation and a simple way to pause or skip subscriptions for her customers.
**Offline Requirement:** When delivering boxes at a local farmer's market (often with spotty network coverage), she needs to verify active subscribers and record fulfillment instantly.

## 3. Problem Statement

Setting up recurring revenue streams currently requires:
- Managing complicated payment gateway Webhooks (e.g., Stripe Billing).
- Handling third-party integrations (e.g., Shopify + ReCharge).
- Manually configuring dunning flows (what happens when a card declines).
- Relying entirely on cloud connectivity, meaning offline validation (e.g., scanning a student's package QR code in a basement studio) fails.

For non-technical owners, this friction prevents the adoption of predictable recurring revenue models.

## 4. Proposed Architectural Solution

The proposed solution replaces manual configuration and cloud-dependent state with a synchronized, offline-first CRDT (Conflict-Free Replicated Data Type) architecture, paired with AI-driven autonomous management.

### 4.1 Offline-First CRDT Synchronization

To enable offline validation and redemption (e.g., student package redemption):
- **Local Storage:** The mobile/PWA client will use an embedded SQLite database (via `sqflite` for Flutter native) or IndexedDB (for Web/PWA).
- **State Synchronization:** Subscriptions, current quotas, and redemption logs are modeled as CRDTs. This ensures that offline state mutations (e.g., Leo marks a lesson as "redeemed" while offline) can be deterministically merged with the server state once connectivity is restored.
- **Edge Caching:** Active subscriber status and quotas are edge-cached closer to the user to ensure instant load times even when online.

### 4.2 Autonomous Dunning via the Finance Agent ("The Accountant")

Instead of the business owner configuring complex dunning rules (e.g., "Retry in 3 days, then send email"):
- **Automated Workflows:** The OHC Finance AI Agent handles failed payments autonomously.
- **Smart Retries:** The agent analyzes the reason for the decline (e.g., insufficient funds vs. expired card) and determines the optimal retry schedule.
- **Customer Outreach:** The agent directly emails or texts the subscriber on behalf of the business owner (e.g., "Hi [Student], your card for Leo's lessons expired. Click here to update it") using natural language.
- **Business Owner Transparency:** The owner receives plain-language summaries from the Finance Agent: *"One of your students' cards failed, but I've already reached out to them to get it updated. No action needed from you."*

### 4.3 Stripe Billing Abstraction

- **Backend Integration:** The OHC backend utilizes Stripe Payment Intents and Stripe Billing underneath, but abstracts the entire concept from the user.
- **Webhook Handling:** A centralized webhook ingestion service in the Go backend reliably processes Stripe events, translating them into normalized subscription state updates which are then broadcast to the relevant tenant's connected clients via WebSockets and synced to their local CRDT stores.

## 5. System Components

1. **Client Data Store (Flutter / PWA):** SQLite / IndexedDB storing `SubscriptionState` and `RedemptionLog` CRDTs.
2. **Sync Engine:** Background worker on the client that pushes CRDT deltas to the OHC Go backend and pulls edge-cached updates.
3. **OHC API Layer (Go + PostgreSQL):** Validates CRDT merges, updates the canonical PostgreSQL database (row-level security enforced per tenant), and interfaces with Stripe.
4. **Finance AI Agent Worker (Python / Go Worker):** Subscribes to payment failure events, decides the dunning strategy, executes customer outreach via LLM-generated templates, and logs actions to the tenant's memory stream.

## 6. Conclusion

By abstracting the complexity of subscription management and relying on an offline-first CRDT datastore, OHC empowers non-technical business owners to harness recurring revenue effortlessly. The Finance Agent acts as an invisible accounts receivable department, ensuring smooth cash flow while maintaining the platform's core promise of zero required technical knowledge.
