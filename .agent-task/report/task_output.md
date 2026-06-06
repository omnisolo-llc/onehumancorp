issue_title: "[Research] OHC Native Service Bookings & Calendar Sync Architecture"
issue_description: |
  # Research Report: Native Service Bookings & Calendar Sync Architecture

  ## Problem Statement
  For service-based business owners like Leo (music tutor) and Carlos (freelance handyman), managing schedules, collecting deposits, and sending meeting links across disparate tools (Calendly, Zoom, Stripe) is complex and frustrating. Current platforms like Shopify require paid third-party apps for booking, causing fragmentation and increased costs. They need an integrated, zero-configuration booking system.

  ## Research Report
  - **Competitor Analysis:** Shopify excels at physical goods but relies on external plugins for service bookings. Wix and Squarespace offer booking add-ons but they are often complex to set up. Calendly is the standard but lacks deep e-commerce integration.
  - **The OHC Opportunity:** By treating a "Booking Slot" as a native resource type alongside "Physical Product", OHC can provide a unified commerce experience. This naturally integrates with our multi-tenant PostgreSQL architecture and Redlock mechanism for preventing double-bookings.

  ## Design Doc
  - **Data Model:**
    - `Service` entity (inherits from core Product model, adds duration, location).
    - `AvailabilitySchedule` entity (recurring rules, overrides).
    - `Booking` entity (time, customer, status, payment intent).
  - **Architecture:**
    - Integrate with Operations Agent to manage scheduling logic.
    - Leverage Redis Redlock (`ohc:lock:{tenant_id}:schedule:{time_slot}`) to prevent double-booking during checkout.
    - Calendar Sync module (Google/Outlook integration) to block personal time.
  - **Mobile UX:**
    - 375px mobile-first agenda view.
    - "New Booking Request" action cards in the Agent Feed for manual approval (if required by owner).

  ## Implementation Prompt
  - Design the database schema for `Service`, `AvailabilitySchedule`, and `Booking`.
  - Implement a booking availability calculation engine that merges standard rules with calendar block-outs.
  - Integrate Redis Redlock to secure time slots during the checkout process.
  - Develop a mobile-first (375px) calendar/agenda view UI for the business owner.
  - Note: Ensure all database entities are scoped by `tenant_id` for multi-tenancy.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, bookings]
assignees: []
