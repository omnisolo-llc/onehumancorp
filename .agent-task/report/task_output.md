issue_title: "[Research] AI Appointment Booking & Scheduling Architecture"
issue_description: |
  # Research Report: AI Appointment Booking & Scheduling

  ## Problem Statement
  Carlos (Field Service) and Leo (Tutor) need a way to schedule appointments and manage their calendars. Currently, there is no integrated scheduling capability that an AI agent can read, propose, or write to. They lose time playing "calendar ping-pong" with customers over SMS/DMs. We need an autonomous booking system where the AI assistant can negotiate times based on real availability, hold slots, and finalize bookings.

  ## Research Report
  - Competitors like Square Appointments and Calendly have structured booking, but lack conversational negotiation.
  - An AI-native approach allows the assistant to answer questions ("do you have anything open Tuesday afternoon?") and immediately propose available slots without sending a link.
  - Multi-tenant data isolation is critical: bookings, availability rules, and schedules belong to a specific tenant.

  ## Design Doc
  - **Data Model (Postgres):**
    - `schedules`: tenant_id, user_id, working_hours, timezone
    - `appointments`: tenant_id, customer_id, start_time, end_time, status (pending, confirmed, cancelled)
  - **AI Agent Integration:**
    - The Operations Assistant needs a `check_availability` tool and a `create_appointment` tool.
  - **Mobile UX Flow:**
    - 375px viewport: A simple agenda view for the owner showing today's bookings.
    - An "add availability" sheet with simple time pickers.

  ## Implementation Prompt
  Implement the backend data model and API endpoints for AI-driven scheduling. Include Postgres migrations for `schedules` and `appointments` with multi-tenant RLS. Create gRPC/REST endpoints to query availability and book slots. Add matching tool definitions for the AI assistant so it can call these endpoints. Create a basic 375px-friendly agenda view in the Tauri frontend. Ensure 100% test coverage including a Playwright E2E test verifying a user can see a created appointment.

  ## Scope & Priority
  Priority: P1
  Estimated Scope: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
