title: "Research Report: Native Service Bookings & Calendar Sync Architecture"
status: "Completed"
author: "Jules"
date: "2024-05-24"
executive_summary: >
  This report explores the architecture for a zero-configuration "Native Service Bookings & Calendar Sync" system within OneHumanCorp (OHC). Service-based small business owners (e.g., tutors, repairmen, cleaners) lack an intuitive, built-in way to manage bookings. Currently, setting up appointments, syncing calendars, and handling payments requires piecing together multiple external tools or confusing configurations. OHC aims to solve this with a mobile-first, AI-driven, transparent booking system.

problem_statement_and_pain_points:
  description: "Based on the OHC persona (e.g., Leo the Music Tutor, Carlos the Handyman), the current challenges are:"
  points:
    - "Configuration Paralysis: Existing solutions like Calendly or Acuity require technical setup (e.g., webhooks, API keys, timezone rules) that non-technical users struggle with."
    - "Calendar Conflicts: If a user forgets to manually block out time on their personal Google Calendar, they get double-booked."
    - "Payment Friction: Collecting deposits or upfront payments for services often requires setting up a separate invoicing system."
    - "Follow-up Overload: Remembering to send reminders, zoom links, or follow-up messages takes time and mental energy away from the actual service."

proposed_architecture:
  data_model:
    description: "The architecture extends the multi-tenant SaaS model with new entities in PostgreSQL:"
    entities:
      - "services: Defines the offering (name, description, price, duration, deposit_required)."
      - "availability_schedules: Working hours per day, exceptions (holidays), and timezone."
      - "bookings: The actual appointment (customer_id, service_id, start_time, end_time, status [pending, confirmed, cancelled], payment_intent_id)."
      - "calendar_integrations: OAuth tokens and sync metadata for Google/Apple/Outlook calendars."
  calendar_sync_engine:
    description: "A background worker (Go + PostgreSQL SKIP LOCKED) handles calendar synchronization:"
    components:
      - "Inbound Sync: Regularly polls (or receives webhooks) from external calendars (Google Calendar API) to block out busy slots in the OHC availability pool."
      - "Outbound Sync: Pushes new OHC bookings to the external calendar so the owner sees them alongside personal events."
      - "Concurrency: Uses Redis Redlock (`ohc:lock:{tenant_id}:calendar_sync`) to prevent duplicate sync jobs."
  payment_integration:
    description: "Integration with Stripe for seamless payments."
    features:
      - "Service bookings can enforce a 'Deposit Required' or 'Full Payment Required' rule."
      - "Uses Stripe Payment Intents. The booking remains in a `pending` state until the Stripe webhook confirms the payment (`payment_intent.succeeded`)."
  ai_agents_integration:
    description: "AI agents to streamline the booking process."
    agents:
      - "Operations (The Manager): Handles the core booking logic, checks availability against synced calendars, and creates the booking record."
      - "Customer Success (The Ambassador): Auto-generates and sends confirmation emails, SMS reminders 24 hours before the booking, and post-service review requests."
      - "Sales & Acquisition (The Salesperson): If a customer abandons a booking checkout, the agent drafts a polite follow-up DM or email."

user_experience:
  business_owner:
    persona: "Carlos"
    steps:
      - "Setup (Zero-Config): Carlos clicks 'Add Service'. He enters 'Plumbing Fix' and '$100'. He taps 'Sync Google Calendar'. He authorizes via OAuth. That's it."
      - "Management: His dashboard shows today's upcoming jobs. He can tap a job to see customer details or reschedule."
  customer:
    steps:
      - "Booking Flow: Customer visits Carlos's OHC link. They select 'Plumbing Fix'."
      - "Time Selection: They see available slots in their local timezone (auto-calculated based on Carlos's availability minus synced busy times)."
      - "Checkout: They enter payment details for the deposit directly in the mobile-optimized flow."
      - "Confirmation: They instantly receive an SMS with the confirmed time."

competitive_advantage: >
  Unlike Shopify (which requires expensive 3rd-party apps for booking) or Wix (which has a complex booking module), OHC's solution is built into the core platform, requires zero configuration, and leverages AI agents to handle the communication overhead automatically.
