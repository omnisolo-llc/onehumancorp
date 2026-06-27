issue_title: "Implement Autonomous Agent-Driven Booking & Resource Management System"
issue_description: |
  **Title**: Implement Autonomous Agent-Driven Booking & Resource Management System

  **Problem Statement**:
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients. This fragmentation hurts revenue and creates operational overhead for non-technical users.

  **Research Report**:
  - **Shopify**: Requires third-party apps for robust booking, adding $15-$30/month and fracturing the user experience. Bookings are treated awkwardly as products.
  - **Wix & Squarespace**: Offer native booking but lack proactive, agent-driven management. They are passive systems waiting for customer action.
  - **GoDaddy**: Basic appointment capabilities, lacks complex resource routing and AI follow-up.
  - **Calendly**: Excellent scheduling but detached from the primary business storefront, deposit flows, and unified customer relationship management.

  **The OHC Opportunity**:
  By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.

  **Design Doc**:

  *Architecture Diagram*
  ```mermaid
  graph TD
      Customer[Customer Mobile Browser] -->|Selects Slot & Pays Deposit| OHC_API[OHC Web/API Layer]
      OHC_API -->|Reserve Slot| Postgres[(PostgreSQL Central Ledger)]
      OHC_API -->|Process Payment| Stripe[Stripe Integrator]
      Postgres -->|Trigger Event| Agent_Operations[Operations Agent]
      Postgres -->|Trigger Event| Agent_Sales[Sales/Success Agent]
      Agent_Operations -->|Sync Calendar| GoogleCal[External Calendar Sync]
      Agent_Sales -->|Check Dormant| Notifications[Push Notification to Owner]
      Notifications --> Owner[Owner Mobile App 375px]
  ```

  *UI Wireframes / Screen Flow (375px first)*
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (44x44px touch targets), and proceed to a deposit payment flow (Stripe).
  2. **Owner Dashboard**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions (e.g. "Student X hasn't booked in 2 weeks. Send reminder?").

  *Mobile UX Flow*
  - The calendar renders vertically on mobile.
  - Days with availability are highlighted. Tapping a day expands available time slots below it.
  - After selecting a slot, an inline payment sheet asks for the deposit, preventing cart abandonment.
  - All critical paths require no horizontal scrolling.

  *AI Agent Integration Points*
  - **Operations Agent ("The Manager")**: Monitors the calendar, handles natural language rescheduling requests from customers, and dynamically adjusts availability based on existing blocks and external calendar sync (Google/Apple).
  - **Sales Agent ("The Promoter")**: Scans booking history to identify churn risks. Drafts re-engagement SMS/emails with a direct, pre-filled booking link.

  *Key Design Decisions*
  - **Unified Ledger**: Treat time slots as a specific type of inventory in PostgreSQL, using row-level locking for concurrent booking attempts.
  - **Proactive Over Passive**: The AI must push Action Cards to the owner's feed rather than requiring the owner to check a calendar dashboard.
  - **Zero Trust/Multi-Tenant**: Secure all calendar queries strictly by `tenant_id` at the row level.

  **Implementation Prompt**:
  **User-Facing Outcome**: Leo the Music Tutor can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments natively, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Critical User Journey (CUJ)**:
  1. Leo sets his availability via the OHC Mobile App (375px).
  2. A student visits Leo's OHC link, taps a time slot, and pays a $20 deposit via Stripe.
  3. The Operations Agent locks the slot, updates Leo's calendar, and sends a confirmation.
  4. Three weeks later, the Sales Agent notices the student hasn't booked again and pushes a card to Leo: "Drafted reminder to Student X. Approve?"
  5. Leo taps "Approve" from his phone.

  **Acceptance Criteria**:
  - Implement core Data Models (`Service`, `AvailabilityBlock`, `Booking`) with strict multi-tenant isolation.
  - Develop Customer Booking Flow UI functioning flawlessly on 375px viewports (no horizontal scroll, 44x44px buttons).
  - Integrate Stripe for deposits linked to specific `Booking` states.
  - Ensure Playwright E2E tests cover the full flow from customer selection to owner approval.
  - No mock API calls in E2E tests; all state must resolve via PostgreSQL.

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
