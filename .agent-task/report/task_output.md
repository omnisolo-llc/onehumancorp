issue_title: "Implement AI-Powered Autonomous Booking System & Customer Re-engagement"
issue_description: |
  # Task Overview
  Service-based small business owners struggle with fragmented booking systems. They typically bolt on third-party tools (like Calendly) to their main website, which leads to a disconnected experience and manual follow-ups. Existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ## Problem Statement
  Service-based small business owners (e.g., Music Tutors, Handymen) require an autonomous booking system seamlessly integrated with their e-commerce storefront. Current solutions like Shopify or Wix rely on passive tools or expensive third-party apps, failing to provide proactive agent-driven calendar management or automatic follow-ups for dormant clients.

  ## Research Report
  - **Market Context**: Platforms like Shopify treat bookings as products and lack native calendar management, whereas Wix offers passive scheduling. Neither provides AI-driven proactive re-engagement.
  - **Competitive Edge**: By integrating an autonomous booking system powered by Operations and Sales AI Agents, OHC can eliminate the "app tax" while proactively managing resource availability and generating revenue through targeted follow-ups.

  ## Design Doc
  ### Data Model
  - `Service`: Defines the type of appointment (duration, price, deposit required).
  - `Resource`: The provider (e.g., human) or physical space.
  - `AvailabilityBlock`: Time blocks when the resource is available.
  - `Booking`: The actual appointment linked to Customer, Service, and Resource, maintaining state (pending, confirmed, completed).

  ### Mobile UX Flow (375px)
  1. **Customer View**: A responsive, touch-friendly calendar view for date/slot selection leading into a seamless Stripe deposit flow.
  2. **Owner Dashboard**: A unified feed displaying upcoming bookings, new requests, and AI-suggested re-engagement actions via the unified Agent feed.

  ### AI Agent Integration
  - **Operations Agent**: Parses natural language rescheduling requests and updates calendar availability dynamically based on external syncs.
  - **Customer Success / Sales Agent**: Automatically identifies dormant customers and drafts targeted SMS/Email follow-ups with direct booking links.

  ### Key Design Decisions
  - Natively mapping Bookings alongside standard product checkout schemas allows a unified cart/deposit experience.
  - Agent-driven feed notifications reduce cognitive load on the owner, replacing traditional complex scheduling grids.

  ## Implementation Prompt
  **Outcome**: Enable a fully integrated, AI-driven booking system for service providers (e.g. Leo the Music Tutor) that handles bookings, deposits, calendar sync, and automatic re-engagement of dormant students.

  **Acceptance Criteria**:
  1. Implement core Booking data models (Service, Resource, AvailabilityBlock, Booking) with strict tenant isolation.
  2. Build a mobile-first (375px) Customer Booking Flow UI for slot selection and deposit processing.
  3. Integrate Operations Agent capability to manage calendar availability via natural language.
  4. Build Owner Dashboard unified feed cards for booking requests and AI-suggested follow-ups.
  5. Provide Playwright E2E test verifying the booking flow.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
