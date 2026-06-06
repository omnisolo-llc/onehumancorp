issue_title: "[Research] Add research report on Native Service Bookings & Calendar Sync Architecture"
issue_description: |
  # Native Service Bookings & Calendar Sync Architecture

  **Department:** Operations
  **Status:** Completed

  ## Summary
  Research report detailing the architectural design for a multi-tenant, zero-configuration Native Service Bookings and Calendar Sync system on the OneHumanCorp platform.

  ## Details
  - **Objective:** Eliminate scheduling friction for service-based small businesses by providing an out-of-the-box booking system that natively syncs with major calendar providers (Google Calendar, Outlook).
  - **Core Architecture:** The booking system operates as a core module within the Operations department. It uses PostgreSQL for robust multi-tenant transactional storage of booking slots, reservations, and connected calendar sync states.
  - **Calendar Synchronization Engine:** A background worker built in Rust/Go polls and receives webhooks from external calendar APIs (e.g., Google Calendar API). It translates external calendar events into normalized OHC free/busy blocks, persisting them in a Redis cache for sub-millisecond retrieval during storefront availability queries.
  - **Booking State Machine:** Implement a strict state machine (Draft -> Pending Deposit -> Confirmed -> Completed/Cancelled) using PostgreSQL row-level locking (SELECT FOR UPDATE) to prevent double-booking race conditions during high-concurrency checkout flows.
  - **Frontend Integration:** Expose gRPC/REST endpoints for the Flutter/PWA storefronts to dynamically render available time slots based on the union of configured working hours and real-time external calendar free/busy data. The UI utilizes the OHC Glassmorphism design system for a premium booking experience.
  - **Agentic Workflows:** Integrate with the 'Customer Success' agent to automatically generate and dispatch contextual booking confirmations, reminders, and follow-up review requests via the unified inbox (email/SMS/WhatsApp).
  - **Security & Isolation:** All calendar OAuth tokens are securely encrypted at rest. Row-Level Security (RLS) policies enforce strict tenant data isolation, ensuring a business can only ever access its own booking and calendar sync data.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
