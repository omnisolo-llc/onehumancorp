issue_title: "Feature: Native AI-Coordinated Unified Booking & Service Operations Engine"
issue_description: |
  ## Problem Statement
  SMBs in the service sector (e.g., Leo the music tutor, Carlos the handyman, Maya the baker needing custom deposits) face extreme platform fragmentation. Existing platforms like Shopify and Wix require third-party "app taxes" (e.g., $20/mo booking plugins) to handle time slots, deposits, and service quotes. These integrations often break multi-tenant consistency and are confusing for non-technical owners on mobile devices. The gap is the lack of a native, first-class Booking and Service Operations engine where an AI agent can autonomously quote, schedule, and collect deposits in a unified conversational flow.

  ## Research Report
  - **Shopify:** Primarily built for physical products. Bookings require complex third-party apps with disjointed UI.
  - **Wix/Squarespace:** Have native booking modules, but lack deep conversational AI execution; the owner must manually approve and manage slots.
  - **OHC Opportunity:** By building bookings directly into the core `tenant` architecture, the OHC Operations Assistant can automatically turn an Instagram DM ("Can you fix my sink on Tuesday?") into a booked slot, calendar update, and Stripe payment request without the owner needing to configure a plugin.
  - **Data points:** 28% of users cite setup paralysis; service businesses cite "app tax fatigue." Unifying commerce and bookings closes a massive competitive gap.

  ## Design Doc
  ### High-Level Architecture (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ SERVICE : offers
      SERVICE ||--o{ TIME_SLOT : has
      TENANT ||--o{ BOOKING : manages
      BOOKING }o--|| TIME_SLOT : reserves
      BOOKING }o--|| DEPOSIT : requires
      CUSTOMER ||--o{ BOOKING : makes
  ```
  ### Mobile UX Flow (375px First)
  1. **Feed (Triage):** Owner opens app; a card reads, "Operations Agent: Drafted 3 booking replies and scheduled 2 visits for tomorrow."
  2. **Booking Detail:** Tap card -> See native mobile view with clean, translucent glass UI (Apple/Ubiquiti style). Shows customer, requested time, and proposed deposit amount.
  3. **Action:** One-tap "Approve & Send Link" or "Edit." No complex desktop calendar grids on mobile.

  ### AI Agent Integration
  - **Work Triage:** Receives intent for a service/appointment from inbox.
  - **Operations Assistant:** Queries `TIME_SLOT` availability, checks constraints, and locks a slot temporarily using Redis Redlock.
  - **Sales & Revenue Assistant:** Generates the invoice/deposit link (Stripe).
  - **Customer Assistant:** Drafts the localized SMS/DM back to the customer.

  ## Implementation Prompt
  **Goal:** Implement the backend domain and mobile-first UI for a unified Booking & Service Operations Engine.
  **CUJ (Critical User Journey):**
  As an owner (e.g., Carlos), I want to create a service ("Sink Repair, $50 deposit"), view my daily schedule on my phone, and have the AI automatically draft a booking proposal for an incoming customer request.
  **Acceptance Criteria:**
  1. Define the multi-tenant PostgreSQL schema for Services, TimeSlots, and Bookings.
  2. Create the gRPC API endpoints for booking operations in Go (Go + Bazel).
  3. Implement the frontend screens in Flutter (Mobile-First 375px, Translucent Glass tokens) for creating a service and viewing the daily booking feed.
  4. Integrate the Operations Assistant to query availability and draft responses.
  5. 100% Unit Test coverage and full Playwright E2E verification mimicking Carlos's journey.

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
