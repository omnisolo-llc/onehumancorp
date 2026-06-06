title: "Research Report: Native Service Bookings & Calendar Sync Architecture"
sections:
  - id: 1
    title: "Introduction"
    content: >
      This report explores the architectural requirements and design strategy for implementing
      a Native Service Bookings & Calendar Sync system within the OneHumanCorp (OHC) platform.
      This feature targets service-based business owners (e.g., Carlos the Handyman, Leo the
      Music Tutor) who currently lack a zero-configuration, integrated booking solution on the platform.
  - id: 2
    title: "Market Need & The 'Zero-Configuration' Goal"
    content: >
      Service businesses rely heavily on scheduling. Traditional solutions (e.g., Calendly, Acuity)
      often require separate subscriptions and complex integrations to connect with a CRM or payment system.
      The OHC Approach:
      - Unified Experience: Bookings, payments (deposits/full), and customer management are handled in one place.
      - Zero Configuration: AI agents (e.g., the Operations Agent) handle the setup, automatically generating booking pages based on the business profile.
      - Seamless Sync: Native integration with external calendars (Google Calendar, Apple Calendar, Outlook) to prevent double-booking.
  - id: 3
    title: "Core Capabilities"
    content: >
      - Availability Management: Define working hours, buffer times between appointments, and time-off.
      - Service Definition: Create bookable services with predefined durations, prices, and required deposit amounts.
      - Two-Way Calendar Sync: Read (Block OHC availability based on busy slots in external calendars) and Write (Push OHC bookings to external calendars).
      - Automated Communication: AI-driven booking confirmations, reminders, and follow-ups.
      - Deposit & Payment Handling: Integrated Stripe checkout for booking deposits or full prepayments.
  - id: 4
    title: "Architectural Design"
    content: >
      4.1 Data Modeling (PostgreSQL):
      We need to introduce robust models for managing availability and bookings, ensuring row-level multi-tenant isolation.
      - 'services' table: Extends existing catalog to support service types (duration, price, deposit_required).
      - 'availability_rules' table: Defines general working hours and exceptions for a tenant.
      - 'bookings' table: Tracks appointments (service_id, customer_id, start_time, end_time, status [pending, confirmed, cancelled], payment_intent_id).
      - 'calendar_sync_connections' table: Stores OAuth tokens for external calendar providers (Google, Microsoft) per tenant.

      4.2 Two-Way Calendar Sync Mechanism:
      - OAuth Integration: Secure OAuth 2.0 flow for Google Calendar API and Microsoft Graph API.
      - Background Syncing (KAIROS): Implement a dedicated background worker (e.g., 'CalendarSyncWorker') using the existing distributed queue. Periodic polling (or webhook reception where supported) to update external busy times.
      - Conflict Resolution: Use optimistic locking when finalizing a booking to ensure the slot wasn't claimed concurrently.

      4.3 AI Agent Integration:
      - Operations Agent: Monitors the 'bookings' table. Handles rescheduling requests and updates availability based on user plain-language commands ("Block off next Friday for vacation").
      - Customer Success Agent: Drafts and sends personalized appointment reminders and post-service follow-up emails/SMS.
      - Sales Agent: If a booking inquiry is received via chat, the agent can propose available time slots and generate a booking link.

      4.4 API & Frontend Layer:
      - Booking Widget: A low-latency, mobile-first calendar component for the storefront.
      - Management Dashboard: A unified calendar view in the Tauri app for the business owner.
      - Cursor Pagination & Lean Payloads: Ensure the booking APIs fetch slots efficiently to support slow networks.
  - id: 5
    title: "Implementation Roadmap"
    content: >
      1. Phase 1: Foundation. Data schema for Services and Bookings. Internal OHC calendar functionality (no external sync).
      2. Phase 2: Payment Integration. Require deposits for bookings via Stripe integration.
      3. Phase 3: External Sync. Implement OAuth and two-way sync with Google Calendar.
      4. Phase 4: AI Orchestration. Empower the Operations and Customer Success agents to manage the booking lifecycle.
  - id: 6
    title: "Conclusion"
    content: >
      A native, zero-configuration booking system is critical for capturing the service-based SMB market. By leveraging OHC's multi-tenant architecture and AI agent orchestration, we can provide a superior, unified experience compared to fragmented third-party solutions.
