# Research Report: Autonomous Subscription and Recurring Billing Engine

## 1. Problem Statement & Personas

**Identified Personas:**
- **Leo the Music Tutor**: Needs subscription-based pricing for monthly lesson packages. Needs a simple setup without webhooks or third-party tools.
- **Maya the Baker**: Needs pre-payment deposits for custom orders, and potentially a subscription model for a "Cake of the Month" club. Needs simple setup without webhooks or third-party tools.

**The Problem:**
Small businesses need recurring subscription capabilities without complex configuration, webhooks, or third-party paid tools (like ReCharge). Currently, setting up edge-cached, offline-first subscriptions seamlessly is difficult. Existing solutions require technical knowledge (e.g., configuring Stripe webhooks) and constant internet connectivity to validate subscriptions.

## 2. Proposed Architecture

We propose an **Autonomous Subscription and Recurring Billing Engine** integrated directly into the OneHumanCorp (OHC) platform, specifically managed by the "Finance & Payments" (The Accountant) AI Agent.

### Core Architectural Components:

1.  **Offline-First Local Data Store (Mobile/Web/Desktop Client):**
    *   **Technology:** SQLite (Standalone Mode via Tauri v2 Desktop) / IndexedDB (Web/PWA via NextJS prototype).
    *   **Purpose:** To store subscription state, active packages, and available credits locally on the user's device. This enables offline validation (e.g., Leo verifying a student's active lesson package in a location with poor cell reception).

2.  **CRDT (Conflict-free Replicated Data Type) Synchronization:**
    *   **Technology:** A CRDT library suitable for Tauri/Rust/Typescript (e.g., integrating with a backend CRDT engine like Automerge or a custom implementation tailored for OHC).
    *   **Purpose:** To synchronize the local SQLite/IndexedDB store with the backend PostgreSQL database seamlessly when the device comes online. This handles offline mutations (e.g., Maya marking a cake delivery complete while offline) and resolves conflicts automatically without user intervention.

3.  **Backend State Management & Dunning:**
    *   **Technology:** Rust (gRPC/Axum API) + Bazel.
    *   **Database:** PostgreSQL (with existing row-level tenant isolation).
    *   **Job Queue:** PostgreSQL `SKIP LOCKED` pattern.
    *   **Purpose:** To serve as the source of truth when online. The job queue handles scheduled tasks like subscription renewals, payment retries, and triggering dunning workflows.

4.  **Autonomous AI Finance Agent (Dunning & Management):**
    *   **Integration:** The "Finance & Payments" (The Accountant) AI Agent.
    *   **Capabilities:**
        *   Automatically retries failed payments based on optimal schedules.
        *   Drafts and sends personalized plain-language emails/SMS to customers whose cards are failing or expiring (Dunning).
        *   Handles subscription upgrades/downgrades automatically based on natural language requests via Customer Success Agent or direct portal interactions.
        *   Requires zero manual configuration of retry rules by the business owner.

### Workflow Example (Leo the Music Tutor):

1.  **Subscription Creation:** Leo creates a "4 Lessons/Month" package in the OHC app. He sets the price and schedule. The AI Finance Agent automatically sets up the underlying Stripe Billing constructs (Products, Prices, Subscriptions) behind the scenes. No webhooks need to be manually configured by Leo.
2.  **Student Purchase:** A student purchases the package via Leo's public OHC page.
3.  **Offline Validation:** The next day, Leo is at a student's house with poor internet. He opens the OHC app. Because the student's subscription state was synced to his local SQLite DB via CRDTs, he can instantly verify they have an active package and log a lesson.
4.  **Sync & Deduct:** When Leo regains connectivity, the CRDT engine syncs the offline action (logging a lesson) back to the Rust backend and PostgreSQL database, decrementing the student's remaining lessons for the month.
5.  **Payment Failure & Dunning:** At the end of the month, the student's card fails. The backend job queue detects this. The Finance AI Agent takes over:
    *   It analyzes the failure reason (e.g., insufficient funds vs. expired card).
    *   It schedules smart retries.
    *   It automatically drafts a polite SMS to the student: "Hi! This is Leo's assistant. Your card for this month's lessons didn't go through. Can you update it here? [Link]".
    *   Leo sees a simple notification: "Payment issue for Student X handled automatically." He doesn't need to configure *how* it's handled.

## 3. Advantages of this Design

*   **Zero Technical Configuration:** Business owners never see webhooks, retry schedules, or API keys. The AI Agent handles the complexity.
*   **True Offline Capability:** The CRDT + SQLite/IndexedDB approach guarantees that core operations (validating subscriptions, checking remaining credits) work even without an internet connection, critical for mobile service providers.
*   **Seamless Conflict Resolution:** CRDTs ensure that if a business owner uses the app on their phone offline, and then on their web browser online, the state merges correctly without data loss or confusing errors.
*   **Proactive Recovery:** The AI Finance Agent acts as a dedicated accountant, recovering failed payments through intelligent dunning workflows, directly increasing revenue for the business owner without their manual effort.
