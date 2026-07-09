issue_title: "Research: Mobile-First Agentic Booking & Centralized Inventory POS Integration"
issue_description: |
  # Mission Queue Protocol: Architectural Gap & Scaling Discovery

  ## 1. Problem Statement
  OneHumanCorp (OHC) is missing a seamless, natively integrated booking and inventory management system that is truly "agent-first" and functions flawlessly on mobile devices (375px viewports). Non-technical owners like Maya (baker), Carlos (handyman), and Priya (boutique operator) currently face a disjointed experience when attempting to manage both physical inventory and service bookings. They are forced to rely on complex third-party tools (like Calendly or Shopify Apps) which introduce the "app tax", fragment customer data, and fail to leverage OHC’s AI agents (The Manager, The Ambassador) for proactive operations.

  ## 2. Research Report
  - **Market Landscape:** Competitors like Shopify require users to bolt on third-party apps to handle bookings alongside physical goods, leading to a fractured UX and additional monthly costs. Wix and Squarespace offer booking functionality but remain passive—they don't proactively manage the calendar or re-engage dormant customers.
  - **The OHC Opportunity:** By treating "Services/Bookings" as first-class citizens alongside "Products" in a unified Central Ledger, OHC can eliminate the need for third-party apps.
  - **Data Consistency:** A critical gap is the lack of a real-time, strongly consistent inventory locking mechanism for hybrid merchants (online + in-store).

  ## 3. Design Doc (Architecture Design)

  ### Data Model & Synchronization
  - **Unified Central Ledger (PostgreSQL):** Represents the source of truth for both Physical Products and Service Bookings. Uses row-level locking for critical updates.
  - **Distributed Locks (Redis Redlock):** Implements a temporary reservation system during checkout to prevent double-booking. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{resource_id}`.
  - **Teammate Mesh Interop:** Ensures Cloud/Standalone synchronization of inventory and booking states via Protobuf over Redis/Memory channels.

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors calendar availability and stock levels. Dynamically generates availability blocks, handles rescheduling requests, and triggers low-stock alerts.
  - **Sales/Customer Success Agent ("The Ambassador"):** Automatically identifies customers who haven't booked follow-ups and drafts re-engagement messages with direct booking links.
  - **Finance Agent ("The Accountant"):** Processes splits for POS/Terminal transactions and correlates booking deposits with final payments.

  ### Mobile UX Flow (375px First)
  - **Zero-Click Generation:** The AI assistant generates the initial service catalog and booking availability based on natural language input.
  - **Unified Owner Feed:** The home screen presents a unified timeline of upcoming bookings, new service requests, and inventory alerts, using a clean, touch-friendly UI (touch targets ≥ 44x44px).
  - **Offline/Local First POS Client:** A mobile POS client that caches catalog data locally and uses eventual consistency to sync offline sales when network connectivity is restored.

  ## 4. Implementation Prompt
  **Goal:** Implement the foundation for the Autonomous Booking & Centralized Inventory system.
  **CUJ (Critical User Journey):**
  1.  As a business owner (e.g., Carlos), I log into OHC on my mobile device (375px viewport).
  2.  I navigate to a unified "Offerings" screen where I can define both a physical product and a service booking (e.g., "Repair Consultation").
  3.  I define availability blocks for the service.
  4.  I switch to a customer view and successfully book a slot, observing that the slot becomes unavailable for subsequent bookings.
  5.  I observe the booking appear in the Owner's unified feed.

  **Acceptance Criteria:**
  - Create the necessary PostgreSQL tables (or extend existing ones) for `Service`, `Resource`, `AvailabilityBlock`, and `Booking`.
  - Implement Redis Redlock (or in-memory equivalent for standalone) for inventory/booking reservation.
  - Expose API endpoints (REST/gRPC) for managing offerings and bookings.
  - Build the mobile-first (375px) UI for the owner to view and manage these offerings, adhering to the Translucent Glass and UniFi modular design system.
  - Ensure all new logic has 100% unit test coverage.
  - Write at least one end-to-end Playwright test covering the CUJ above.

  ## 5. Priority & Scope
  - **Priority:** P1
  - **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
