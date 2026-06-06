issue_title: "Implement Autonomous Subscription and Recurring Billing Engine"
issue_description: |
  # OHC Autonomous Subscription and Recurring Billing Engine

  ## Executive Summary
  This research focuses on a critical capability missing in current small business platforms: **Autonomous Subscription Management**. Personas like Leo (music tutor) or Maya (baker offering monthly cake boxes) need recurring revenue streams, but setting up subscription logic (billing, dunning, portal access) typically requires complex third-party tools like ReCharge on Shopify, which contradict our zero-technical-knowledge promise. We propose an architecture that integrates edge-caching and CRDT-based offline resilience so that subscriptions and lesson booking packages can be managed flawlessly even when internet connection drops.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  ### Competitive Audit
  - **Shopify + ReCharge**: Extremely powerful, but requires configuring webhooks, understanding Liquid templates for the portal, and paying an extra monthly fee.
  - **Wix Pricing Plans**: Native but rigid. Difficult to tie seamlessly into physical inventory decrementing or offline POS.
  - **Stripe Billing**: Highly robust for developers, but the no-code checkout links require manual syncing to an external inventory or CRM system.
  - **OHC Gap**: We lack an "out-of-the-box" subscription engine that is natively managed by the AI Agent Departments without any manual webhook setup.

  ## 2. Deep Dive Architecture Design (Track 2)

  ### Target Capabilities
  - **Zero-Setup Subscriptions**: A user can voice-prompt "Create a monthly guitar lesson package for $100," and the engine provisions the Stripe Billing plan, creates the localized product, and builds the customer portal.
  - **Edge-Caching & Offline Resilience**: Subscriptions (e.g., verifying an active membership via tap-to-pay or QR code at a physical location) must work offline. We use CRDT (Conflict-free Replicated Data Types) for offline tracking of membership usage, synchronizing with the central ledger when connectivity is restored.

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Edge Device
          MobileUI[Mobile UI / 375px] --> LocalStore[(Local SQLite / CRDT Store)]
          LocalStore --> OfflineValidator[Offline Subscription Validator]
          LocalStore --> OutboxQueue[Sync Outbox Queue]
      end
      OutboxQueue -- Network Restored --> SyncAPI[Backend Sync API]
      SyncAPI --> CentralLedger[(PostgreSQL / Ledger)]
      CentralLedger --> Stripe[Stripe Billing & Webhooks]
      CentralLedger --> FinanceAgent[Finance Agent]
      FinanceAgent -. Autonomous Dunning .-> CentralLedger
  ```

  ### Mobile UX Flow (375px first)
  1. **Creation**: User taps "Add Product" -> Selects "Subscription". A single card asks for "What is this and how much per month?". No complex billing cycle dropdowns; defaults are handled by AI.
  2. **Offline Validation**: When an offline customer arrives (e.g., Leo's student scanning a QR code for their lesson), the `OfflineValidator` checks the local CRDT store for active status. It optimistically marks the lesson as used.
  3. **Re-sync**: Upon network restoration, the `OutboxQueue` syncs the usage. If a payment had failed in the interim, the `FinanceAgent` handles it (drafts a polite SMS to the student).

  ### AI Agent Coordination
  - **Finance Agent ("The Accountant")**: Monitors Stripe webhook events. On `invoice.payment_failed`, it doesn't just mark "past due" — it drafts a personalized email/SMS to the customer providing a 1-tap update link.
  - **Operations Agent ("The Manager")**: Correlates subscription usage with physical inventory (e.g., decrementing 1 bag of coffee per month for a coffee subscriber).

  ## 3. Security and Multi-Tenant Isolation
  - Strict row-level security (RLS) on the `CentralLedger` per `tenant_id`.
  - The edge `LocalStore` is strictly scoped and encrypted on the device using a tenant-specific key.

  ## 4. Implementation Prompt
  **Feature Name:** Autonomous Edge-Cached Subscription Engine
  **Target Persona:** Leo the Music Tutor

  **Outcome:** A seamless subscription flow where Leo can sell monthly lesson packages, validate student access offline, and rely on the AI to chase down failed payments.

  **Critical User Journey (CUJ):**
  1. Leo opens the OHC app (375px view) and says, "Make a $200/month 4-lesson package."
  2. The UI instantly shows the generated product card.
  3. A student buys the package online.
  4. Leo loses internet connection but the student arrives for a lesson.
  5. Leo taps "Redeem Lesson" on the student's profile. The local edge store decrements the usage via CRDT.
  6. Internet is restored; the background sync reconciles the usage with the PostgreSQL ledger.

  **Next Actions for Engineering:**
  1. Design the PostgreSQL schema for subscriptions and usage CRDT ledgers.
  2. Implement the local SQLite/IndexedDB store on the mobile client for offline subscription validation.
  3. Create the background sync mechanism to reconcile offline usage with the central ledger.
  4. Extend the Finance Agent to listen to Stripe `invoice.payment_failed` webhooks and trigger the autonomous dunning flow.
  5. Ensure 100% test coverage and Playwright E2E tests for the offline-to-online sync flow.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []