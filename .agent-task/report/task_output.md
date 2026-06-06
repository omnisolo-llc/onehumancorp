# Architectural Design: Autonomous Subscription and Recurring Billing Engine

## 1. Executive Summary

This document outlines the architectural design for the "Autonomous Subscription and Recurring Billing Engine" within the OneHumanCorp (OHC) platform. Our goal is to empower non-technical small business owners—like Leo the Music Tutor, who relies on monthly lesson packages, and Maya the Home Baker, who needs recurring supply drop deliveries—to easily establish, manage, and scale subscription businesses.

The system leverages an offline-first CRDT synchronization architecture, ensuring resilience during spotty internet access (e.g., in a basement studio or a busy food market), and utilizes SQLite/IndexedDB for local storage on mobile and desktop clients. Crucially, the "Finance & Payments AI Agent" ("The Accountant") manages fully autonomous dunning (failed payment recovery) workflows without manual intervention from the business owner.

## 2. Target Personas & Use Cases

### 2.1. Leo — The Music Tutor (Subscription Service)
*   **Need:** Monthly billing for recurring student lesson packages.
*   **Requirement:** An automated way to handle students whose cards expire or fail, without awkward follow-up texts from Leo.
*   **AI Integration:** "The Accountant" agent automatically emails students when a payment fails, provides a secure link to update their card, and pauses lesson scheduling via "The Operations Manager" until payment clears.

### 2.2. Maya — The Home Baker (Physical Goods Subscription)
*   **Need:** "Cake of the Month" club, requiring recurring billing coupled with recurring order generation.
*   **Requirement:** Generating a new fulfillment order for her workflow *only after* the monthly subscription payment succeeds.
*   **AI Integration:** "The Operations Manager" agent creates the order ticket. "The Accountant" manages the recurring billing securely.

## 3. High-Level Architecture

The billing engine operates across the OHC technology stack, integrating deeply with the existing Rust/gRPC/PostgreSQL backend and the local-first Tauri/Flutter edge clients.

### 3.1. Edge Architecture (Offline-First)
*   **Local Storage:** Tauri desktop and Flutter mobile apps use a combination of local SQLite databases and/or IndexedDB (for web/PWA targets) to cache subscription states, product definitions, and recent customer activity.
*   **CRDT Synchronization:** Changes made while offline (e.g., Leo modifying a subscription tier price while on a plane) are recorded locally. A Conflict-Free Replicated Data Type (CRDT) engine ensures that when connectivity is restored, these updates are synchronized with the central PostgreSQL backend seamlessly.

### 3.2. Cloud Architecture (Multi-Tenant Backend)
*   **PostgreSQL:** The source of truth. Features strict row-level security based on `tenant_id`.
*   **gRPC/Axum Services:** The `subscription_service` handles creation, pausing, and cancellation logic.
*   **Stripe Integration:** We leverage Stripe Billing (Subscriptions, Invoices, Payment Intents) as the core payment processor. OHC acts as a smart wrapper and synchronization layer over Stripe.
*   **Redis/Job Queues:** Asynchronous webhook processing from Stripe (e.g., `invoice.payment_failed`) is enqueued using PostgreSQL `SKIP LOCKED` queues, with distributed Redlocks to prevent race conditions during dunning.

## 4. The "Accountant" AI Agent (Autonomous Dunning)

The defining feature of the OHC billing engine is the autonomous resolution of payment issues.

### 4.1. Workflow: The Autonomous Dunning Process
1.  **Event Trigger:** Stripe fires a webhook (`invoice.payment_failed`).
2.  **Queue:** The OHC Backend receives the webhook, verifies the Stripe signature, and pushes an event to the AI Job Queue.
3.  **Agent Activation:** The "Finance & Payments AI Agent" (The Accountant) picks up the job.
4.  **Context Assembly:** The agent queries pgvector memory and the database to understand the customer's history (e.g., "Is this a first-time failure or a chronic issue?").
5.  **Action Formulation:**
    *   The agent drafts a friendly, personalized email or SMS (via Twilio/Postmark integrations) informing the customer of the failure and providing a secure, 1-click Stripe Payment Link to update their payment method.
    *   The agent signals "The Operations" agent to temporarily pause the delivery of physical goods or revoke digital access.
6.  **Follow-up:** If the payment is not updated within X days, the agent sends a follow-up reminder.
7.  **Resolution:** Upon receiving a successful payment webhook, the agent automatically restores access and sends a 'Thank You' receipt.

### 4.2. Tool Access
The AI Agent requires access to specific programmatic tools defined in its `tools` list:
*   `send_customer_message(channel, recipient, message)`
*   `generate_stripe_billing_portal_link(customer_id)`
*   `update_subscription_status(subscription_id, status)`
*   `notify_operations_agent(tenant_id, action_required)`

## 5. Data Model (PostgreSQL / SQLite Schema Sync)

New entities mapped via the CRDT synchronization layer:

*   **`SubscriptionPlans`**
    *   `id`, `tenant_id`, `name`, `price`, `interval` (monthly, weekly), `stripe_price_id`
*   **`CustomerSubscriptions`**
    *   `id`, `tenant_id`, `customer_id`, `plan_id`, `status` (active, past_due, canceled), `next_billing_date`, `stripe_subscription_id`
*   **`DunningEvents`** (For AI context and auditing)
    *   `id`, `tenant_id`, `subscription_id`, `agent_action`, `timestamp`, `resolution_status`

## 6. Security and Compliance

*   **PCI-DSS Compliance:** No actual credit card PANs are stored in OHC databases (SQLite or PostgreSQL). All payment instruments are tokenized via Stripe.
*   **Tenant Isolation:** Row-Level Security (RLS) guarantees that Leo can never see Maya's subscription data, even in the event of an API bug.
*   **Offline Data Security:** Local SQLite databases are encrypted at rest using `SQLCipher` with the key derived securely on device (or managed via the user's secure enclave).

## 7. Next Steps for Implementation

1.  **Define Protobufs:** Create gRPC definitions for `SubscriptionService` in `src/proto`.
2.  **Database Migrations:** Create the `SubscriptionPlans` and `CustomerSubscriptions` schema with RLS.
3.  **Agent Expansion:** Update the system prompt and tool registry for the Finance Agent to handle the `invoice.payment_failed` workflow.
4.  **Tauri/Flutter UI:** Build the subscription management dashboard utilizing OHC's "Premium Glassmorphism" design system, ensuring full functionality at 375px mobile breakpoints.
5.  **CRDT Sync Layer:** Implement the sync mechanisms between the edge database and the Rust backend.
