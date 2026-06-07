issue_title: "Implement Autonomous Booking System"
issue_description: |
  # Mission Protocol Brief: Autonomous Appointment Booking & Resource Management System

  ## Pre-requisite Notes
  During discovery, the docker compose stack failed to start with the following error:
  `failed to extract layer ... failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted`.
  Due to this container startup blocker, we proceeded with generating this brief based on existing research documentation as mandated.

  ## Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ## Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ## Design Doc
  ### Data Model (PostgreSQL)
  - `Service`: The type of appointment (duration, price, deposit required).
  - `Resource`: The provider (e.g., Leo) or physical space.
  - `AvailabilityBlock`: Recurring or specific time blocks when the resource is available.
  - `Booking`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled).

  ### AI Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets), and proceed to a deposit payment flow (Stripe).
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions.

  ## Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Target Persona**: Leo the Music Tutor
  **Outcome**: Leo can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Next Actions**:
  1. Implement the core Data Models (`Service`, `AvailabilityBlock`, `Booking`) with strict multi-tenant isolation.
  2. Develop the Customer Booking Flow UI (mobile-first calendar and slot selection) and integrate it with the existing Stripe payment system for deposits.
  3. Create the Operations Agent capability to parse natural language rescheduling requests and manage calendar availability.
  4. Develop the Owner Dashboard view to manage bookings and view AI-suggested follow-ups.

  ## Docker Whiteout File Error Investigation
  **Root Cause**: The error `failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted` occurs during the Docker Compose startup. This is a known issue when Docker attempts to pull and extract an Alpine-based image (like `valkey/valkey:8-alpine`) on a host using the `overlayfs` storage driver without proper privileges (e.g., inside an unprivileged Docker-in-Docker sandbox). The extraction of `.wh` (whiteout) files, which are used to mark files as deleted in the overlay filesystem, fails due to permission restrictions.
  **Workaround/Fix**: To bypass this in restricted environments, the Dockerfile/docker-compose file can be modified to use a non-Alpine base image (like Debian/Ubuntu based images, e.g., `valkey/valkey:8` instead of `8-alpine`), or the runtime environment needs elevated privileges (`--privileged`) to correctly handle overlayfs whiteout files.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
