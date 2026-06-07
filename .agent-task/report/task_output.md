issue_title: "Implement Proactive Re-engagement System for Agentic Booking"
issue_description: |
  # Research Report: Proactive Re-engagement System for Agentic Booking

  ## 1. Problem Statement
  Service-based small business owners like Leo (the music tutor) or Carlos (the handyman) rely heavily on recurring or repeat bookings. Currently, OHC's booking schema (`BookingRecord`, `BookingService`) focuses on immediate bookings but lacks a proactive re-engagement mechanism. Without AI agents that monitor customer booking frequency and automatically follow up with those who have lapsed, business owners lose significant lifetime value.

  ## 2. Research & Context (Track 1)
  - **Market Insight**: Platforms like Calendly wait for customers to return. Mailchimp requires manual list segmentation. An autonomous system must automatically identify a "missed cycle" (e.g., a student hasn't booked a lesson in 14 days) and prompt action.
  - **OHC Gap**: The native booking system handles the transactional scheduling and deposit (as seen in `src/server/services/booking.rs` and `078_quote_engine.sql`) but does not proactively queue "re-engagement" tasks for the Customer Success (Ambassador) agent.

  ## 3. Architecture Design (Track 2 & Track 3)
  ### Data & Queue Integration
  - **Entity Linking**: Connect the `BookingRecord` completion events to the `ohc_job_queue` (defined in `015_job_queue_and_ledger.sql`).
  - **Scheduled Tasks**: When a booking is marked as 'completed' in `BookingRecord`, an asynchronous job (`type: 'booking_reengagement_check'`) is scheduled for N days in the future based on the service's expected recurring frequency.
  - **The Ambassador Execution**: The job processor queries if a new `BookingRecord` exists for that customer. If not, the Ambassador Agent drafts a personalized re-engagement message (e.g., "Hi [Name], it's been a while since your last guitar lesson. Want to book your next session? Here's my availability: [Link]") and pushes it to the Owner's Agent Feed for 1-tap approval.

  ### Mobile UX Flow (375px)
  1. The business owner receives a push notification: "Leo, 3 students haven't booked in over 2 weeks. Review drafts."
  2. Owner opens the app and sees "Action Required" cards.
  3. Card displays: Customer context (e.g., "Sarah - Last lesson: May 1st") and the AI-drafted message.
  4. Owner taps "Approve" -> The message is dispatched via the configured channel (email/SMS/DM).

  ## 4. Implementation Prompt
  **Target Persona**: Leo the Music Tutor
  **Outcome**: Leo never has to manually track which students have dropped off. The AI automatically drafts follow-ups for lapsed students, securing recurring revenue with zero manual effort.

  **Critical User Journey (CUJ)**:
  1. A student's lesson is marked 'completed' via the `BookingService`.
  2. The system schedules a re-engagement check job in `ohc_job_queue` for 14 days later.
  3. After 14 days, the background job runs. If the student hasn't booked, it creates an "Approve Re-engagement" Action Card in Leo's feed.
  4. Leo taps "Approve" from his 375px mobile view, and the Ambassador agent sends the message.

  **Acceptance Criteria**:
  - Implement a mechanism (e.g., a DB trigger or service layer logic in `booking.rs`) that inserts a re-engagement job into `ohc_job_queue` when a booking is completed.
  - The job processor must verify if a subsequent `BookingRecord` exists for the customer.
  - If no booking exists, it must create an Action Card in the Agent Feed containing the AI-drafted message.
  - Zero-mocking: Must flow through the real Job Queue and Database.

  ## 5. Priority & Scope
  **Priority**: P1
  **Estimated Scope**: Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
