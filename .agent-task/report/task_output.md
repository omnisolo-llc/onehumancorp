# OHC Autonomous Subscription and Recurring Billing Engine

## 1. Executive Summary

Small businesses, such as Leo the Music Tutor offering monthly lesson packages or Maya the Baker running a "Cake of the Month" club, need an effortless way to manage subscriptions. Traditional recurring billing solutions (e.g., ReCharge, standard Stripe Billing) require complex configuration and webhook management that overwhelm non-technical users.

This document proposes the architecture for an **Autonomous Subscription and Recurring Billing Engine** for OneHumanCorp (OHC). By leveraging a combination of offline-first CRDT synchronization (via SQLite/IndexedDB on mobile clients) and background AI agents (specifically the Finance Agent) for dunning and lifecycle management, OHC can deliver a "zero-configuration" subscription experience.

## 2. Target Personas and Use Cases

*   **Leo the Music Tutor:** Sells a "$200/month for 4 lessons" package. Needs a way to automatically bill students, track lesson redemption, and gracefully handle failed payments without awkward manual follow-ups.
*   **Maya the Baker:** Runs a "Monthly Treat Box" subscription. Needs to easily forecast inventory requirements based on active subscriptions and automatically pause subscriptions if a customer's payment fails.

## 3. Core Architectural Components

### 3.1 Data Synchronization: Offline-First CRDTs

To ensure seamless operation even in low-connectivity environments (e.g., a music studio basement or a busy kitchen), the platform will utilize a Conflict-free Replicated Data Type (CRDT) architecture.

*   **Mobile Client Storage:** Devices will use SQLite (for iOS/Android apps) or IndexedDB (for PWAs) to store subscription state, payment status, and redemption counts locally.
*   **Offline Validation:** A tutor like Leo can mark a lesson as redeemed in the app even without an internet connection. The app validates against the local, cryptographically signed subscription state.
*   **Background Sync:** When connectivity is restored, local CRDT changes are synchronized with the central PostgreSQL backend. This prevents data loss and handles conflicts (e.g., a student canceling a subscription online while a tutor marks a lesson redeemed offline) automatically based on logical timestamps.

### 3.2 Automated Finance & Lifecycle Management (AI Agents)

The complexity of subscription management (proration, dunning, pausing) is abstracted away by OHC's AI agents.

*   **The Accountant (Finance Agent):**
    *   **Autonomous Dunning:** Monitors Stripe webhooks for payment failures (`invoice.payment_failed`). Instead of sending a generic, robotic email, the agent drafts a polite, personalized follow-up message (e.g., "Hey [Student], it looks like the card on file for your lessons didn't go through. Here's a link to update it.") and pushes it to the business owner for one-tap approval, or sends it automatically based on user preference.
    *   **Subscription Pausing:** Automatically pauses fulfillment logic if a payment remains unpaid after the dunning cycle, notifying both the owner and the customer.
*   **The Operations Agent (Inventory & Fulfillment):**
    *   **Forecasting:** For physical goods (Maya), the agent analyzes active subscriptions and upcoming renewal dates to automatically generate raw material purchase lists.
    *   **Redemption Tracking:** For services (Leo), tracks usage against the subscription allowance and resets counters at the start of each billing cycle.

### 3.3 Payments Infrastructure

*   **Stripe Billing Integration:** The underlying payment processing will utilize Stripe Billing to handle secure tokenization and recurring charges.
*   **Zero-Config Webhooks:** The OHC backend fully abstracts Stripe's webhook events. Business owners never see or configure webhooks. The system automatically translates technical Stripe events into plain-language business events within the OHC platform.

## 4. System Flow: The Complete Journey

1.  **Creation:** Leo creates a "Monthly Lesson Package" product in OHC. He sets the price and recurrence (monthly). The platform automatically provisions the corresponding Stripe Product and Price.
2.  **Purchase:** A student subscribes via Leo's OHC-hosted storefront. Stripe handles the initial payment and sets up the recurring schedule.
3.  **Local Sync:** Leo's mobile app syncs the new subscription state into its local SQLite database.
4.  **Offline Redemption:** Leo teaches a lesson in a Wi-Fi dead zone and marks 1 of 4 lessons as used in the app. The app updates local state.
5.  **Reconciliation:** Leo connects to Wi-Fi. The CRDT payload syncs to the central OHC backend. The backend updates the global ledger.
6.  **Billing Cycle & Dunning:** On the renewal date, Stripe attempts a charge. If it fails, the webhook triggers the Finance Agent. The Finance Agent drafts a personalized follow-up text/email and sends a push notification to Leo's phone for approval.

## 5. Security & Consistency Considerations

*   **Signed State:** To prevent malicious tampering of offline subscription states (e.g., a client modifying their IndexedDB to grant unlimited lessons), critical state snapshots pushed to the client will be cryptographically signed by the backend. The client cannot validate a redemption against an unsigned state.
*   **Idempotency:** All synchronization endpoints and payment webhooks must be strictly idempotent to handle network retries gracefully.

## 6. Implementation Phasing

*   **Phase 1:** Core Stripe Billing integration and backend modeling (PostgreSQL) for basic recurring payments.
*   **Phase 2:** Implementation of the CRDT sync engine and mobile-first offline storage (SQLite/IndexedDB).
*   **Phase 3:** Integration of the Finance Agent for autonomous dunning workflows and natural language notifications.
