issue_title: "Architectural Deep Dive: Unified Booking, Quoting & Deposit Engine"
issue_description: |
  # Research Report: Unified Booking, Quoting & Deposit Engine

  ## Discovery Findings
  Through an audit of the repository, competitor platforms (Square, Calendly, Jobber), and OHC's target personas (e.g., Carlos the handyman, Leo the music tutor), a significant architectural gap was identified. While OHC handles basic storefronts, it lacks an integrated workflow for service providers to seamlessly combine scheduling, custom quoting, and deposit collection.

  The primary friction point for service-based solopreneurs is context switching: they lose 30-40% of leads managing inquiries via SMS, checking separate calendars, generating quotes in another tool, and sending separate payment links.

  ## Proposed Architecture
  The proposed **Unified Booking, Quoting & Deposit Engine** integrates directly with OHC's Operations AI Agent ("The Vigilant Manager") and the backend ledger.

  ### Key Components:
  1.  **AI-Driven Quoting Generation:** When an inquiry arrives via integrated SMS/social channels, the AI Agent extracts intent, checks the calendar for availability, and drafts a custom quote with a slider for deposit requirements.
  2.  **1-Tap Approval:** The business owner receives a mobile push notification and can approve or tweak the quote with a single tap. No complex forms.
  3.  **Customer Experience:** The customer receives a secure, edge-cached link. The mobile-optimized (375px first) page displays the quote, offers available calendar slots, and immediately collects the deposit via Apple Pay/Google Pay.
  4.  **State Management & Isolation:** Data is strictly isolated by `tenant_id` in PostgreSQL. Once the deposit is paid, the system automatically transitions the quote state, confirms the booking in the unified calendar, and triggers the AI to send confirmation messages.

  ### Technical Integrity
  -   **Multi-tenant Isolation:** Strict partitioning of all Quotes, Bookings, and Services by `tenant_id`.
  -   **Offline-First & Resiliency:** Support for optimistic UI updates on the Flutter client, with operations queued locally if the device loses connection.
  -   **Security:** Payments processed via Stripe integration securely, with Webhook-based confirmation loops to ensure transactional integrity between the booking and the ledger.

  ## Issue Formulation
  This architectural design has been formalized into `docs/research/[architecture]_unified_booking_quoting_deposit_engine.md`. This document follows the Mission Queue Protocol, providing a detailed design doc with Mermaid.js diagrams, mobile UX flow (Translucent Glass UI standard), and an implementation prompt for the engineering swarm.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []