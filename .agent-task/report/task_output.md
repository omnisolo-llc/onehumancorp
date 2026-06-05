issue_title: "[Research] Architect Mobile-First Native Booking System for Service Businesses"
issue_description: |
  # Research Report: Native Booking System Architecture

  ## Problem Statement
  Service businesses (like Carlos the Handyman or Leo the Music Tutor) are a core persona for OHC. Currently, these users must rely on clunky third-party integrations (Calendly, Acuity) that break the "Zero Technical Knowledge" promise and the "Mobile-First" experience. They need a native, integrated booking system that handles time slots, deposits, and automated communications entirely within OHC.

  ## Research Report
  - **Market Context**: ~19% of SMBs report "Lack of Booking Integration" as a primary pain point.
  - **Competitor Analysis**:
    - *Shopify*: Weak native booking (relies heavily on complex apps).
    - *Wix*: Has native booking, but the mobile management experience is poor.
    - *Squarespace*: Good desktop booking (Acuity), but disjointed mobile experience.
  - **OHC Opportunity**: Deliver a truly mobile-first booking experience where service providers can manage their availability and appointments directly from their phone, powered by AI for automated follow-ups and quoting.

  ## Design Doc
  ### Data Model (High-Level)
  - `Service`: The offering (e.g., "Plumbing Fix").
  - `AvailabilitySchedule`: The provider's working hours and exceptions.
  - `Booking`: The actual appointment (includes state: pending, confirmed, completed, cancelled).
  - `Deposit`: Payment intent linked to the booking.

  ### AI Integration Points
  - **Operations Agent ("The Manager")**: Monitors schedule, sends reminders, handles rescheduling requests.
  - **Sales Agent ("The Salesperson")**: Generates quotes based on customer descriptions before booking confirmation.

  ### UX Flow (Mobile First - 375px)
  1. Customer selects Service -> Picks Date/Time (Native mobile date picker).
  2. Customer enters details & pays deposit via Stripe.
  3. Provider receives push notification.
  4. Provider approves/manages booking from the OHC mobile dashboard card view.

  ## Implementation Prompt
  Design and implement the core backend entities (PostgreSQL schemas) and gRPC/REST API contracts for the OHC Native Booking System. Ensure row-level security (tenant isolation) is strictly applied. Create the initial mobile-first (Flutter/Web) UI components for the "Provider Dashboard - Upcoming Bookings" view using the OHC Translucent Glass design system.

  ## Metadata
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
