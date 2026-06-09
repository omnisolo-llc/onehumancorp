issue_title: "OHC Native Agentic Booking System"
issue_description: |
  # Autonomous Appointment Booking & Resource Management System

  ## Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that proactively manages the calendar and re-engages dormant clients.

  ## Research Report
  **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.

  **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.

  **Competitor Gaps**:
  - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
  - *Wix*: Complex setup; passive system.
  - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OwnerApp as Owner (Mobile App)
      participant OHC as OHC Booking Engine
      participant OpsAgent as Operations Agent
      participant CSAgent as Customer Success Agent

      Customer->>OHC: Books Service & Pays Deposit
      OHC->>OpsAgent: Update Calendar & State
      OpsAgent->>OpsAgent: Run Nightly Dormant Analysis
      OpsAgent->>CSAgent: Trigger: "Leo's student missed regular slot"
      CSAgent-->>OHC: Draft check-in message & magic link
      OHC->>OwnerApp: Push Notification: "Approve check-in for Student?"
      OwnerApp->>OHC: Tap "Approve"
      OHC->>Customer: Send SMS/Email
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets, at least 44x44px), and proceed to a deposit payment flow integrated directly with the OHC platform.
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions in a clean card layout with translucent glass materials.
  3. **Agent Interaction View**: A card showing "Student hasn't booked in 2 weeks. Send re-engagement?" with prominent "Approve", "Edit", and "Discard" actions.

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors the calendar, parses natural language scheduling/rescheduling requests from customers, and dynamically manages availability based on existing blocks and external calendar sync.
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ### Key Design Decisions
  - **Unified Ledger**: Booking data resides in the same multi-tenant architecture as e-commerce, allowing seamless unified customer profiles.
  - **Proactive Management**: The system doesn't wait for input. Agents actively monitor state and prompt the owner with ready-to-approve actions.
  - **Mobile Parity**: All owner actions, especially approving AI-generated actions, are optimized for a 375px screen to allow management on the go.

  ## Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Target Persona**: Leo the Music Tutor
  **Outcome**: Leo can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Critical User Journey (CUJ)**:
  1. Leo logs into the OHC mobile web app (375px view).
  2. Leo sets up his availability and links his calendar.
  3. A student books a lesson and pays a deposit.
  4. Two weeks pass without the student booking another lesson.
  5. The Operations Agent notices the dormancy and triggers the Customer Success Agent to draft a re-engagement text with a magic booking link.
  6. Leo receives a notification: "Student X missed their slot. Send follow-up?"
  7. Leo taps "Approve" and the message is sent.

  **Acceptance Criteria**:
  - End-to-end integration of booking engine with tenant isolation.
  - Mobile-first (375px) calendar and booking views.
  - Fully functional UI flow for Leo to review and approve the Customer Success Agent's drafted message.
  - E2E Playwright tests simulating Leo setting up availability, a student booking, and Leo approving a follow-up.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
