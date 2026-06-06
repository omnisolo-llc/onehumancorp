issue_title: "AI-Automated Booking & Dynamic Scheduling Architecture"
issue_description: |
  # Research Report: AI-Automated Booking & Dynamic Scheduling Architecture

  ## Problem Statement
  Service-based small business owners (like Carlos the handyman and Leo the music tutor) lose hours every week managing their schedules, coordinating via text or email, and dealing with no-shows. Traditional booking systems (e.g., Calendly, Acuity, or built-in Wix bookings) force the owner to manually set up complex rules, rigid time blocks, and generic email reminders. These tools are purely functional but lack the intelligence to handle edge cases, dynamic routing, conversational booking, or personalized follow-ups. For a non-technical user, setting up these rules is confusing and often leads to double bookings or unoptimized schedules.

  ## Research Findings
  - **Calendly / Acuity Scheduling:** Excellent standalone utilities for tech-savvy users but require rigid, manual configuration of availability, buffer times, and event types. They do not deeply integrate with the rest of the business operation (e.g., inventory, marketing) without Zapier or custom integrations.
  - **Wix Bookings / Squarespace Scheduling:** Integrated into the site builder, but still require complex setup matrices for services, staff, and hours. They rely on basic templated emails for reminders and lack conversational AI handling for rescheduling or intent-based booking.
  - **OHC Opportunity:** Leverage AI to turn scheduling from a static calendar into an active "Operations Manager". Instead of rigid rules, the owner provides natural language preferences ("I don't work Sunday mornings, and I need 30 mins travel time between house calls"). The AI dynamically handles the routing, calendar block generation, conversational booking with the customer, deposit collection via Stripe, and automated context-aware follow-ups for retention.

  ## Next Steps
  Implement the AI-Automated Booking & Dynamic Scheduling flow:
  1. Create a natural language configuration interface where a business owner inputs their availability preferences.
  2. Implement the parsing logic into a structured `BookingRule` schema in PostgreSQL.
  3. Develop the dynamic slot generator querying existing bookings and integrated Google Calendar.
  4. Implement Redis Redlock for temporary slot reservation to ensure no double bookings.
  5. Integrate with Stripe to manage deposit collection directly within the booking flow.
  6. Connect "The Salesperson", "The Manager", and "The Ambassador" agents.
  7. Implement comprehensive Playwright E2E testing to cover the UI configuration and booking CUJ.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []