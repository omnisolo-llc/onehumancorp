issue_title: "Implement Unified Agentic Autonomous Booking & Resource System for Mobile-First Operations"
issue_description: |
  # OHC Native Agentic Booking System

  ## Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  From a non-technical owner/operator's perspective, they want to be able to tell their assistant "I'm available for 1 hour piano lessons on Tuesday and Thursday afternoons. It costs $50 and I need a 50% deposit. Also follow up with any student who hasn't booked in a month" and have the assistant do the rest. They don't want to configure availability blocks across 3 different systems.

  ## Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ## Design Doc

  ### Architecture
  ```mermaid
  erDiagram
      Tenant ||--o{ Service : "offers"
      Tenant ||--o{ Resource : "owns"
      Tenant ||--o{ Booking : "manages"
      Service ||--o{ Booking : "is booked in"
      Resource ||--o{ AvailabilityBlock : "has"
      Resource ||--o{ Booking : "is assigned to"
      Customer ||--o{ Booking : "makes"

      Service {
          uuid id PK
          uuid tenant_id FK
          string name
          int duration_minutes
          decimal price
          decimal deposit_required
      }

      Resource {
          uuid id PK
          uuid tenant_id FK
          string name "e.g., Leo, Room A"
      }

      AvailabilityBlock {
          uuid id PK
          uuid resource_id FK
          datetime start_time
          datetime end_time
          boolean is_recurring
          string recurrence_rule
      }

      Booking {
          uuid id PK
          uuid tenant_id FK
          uuid customer_id FK
          uuid service_id FK
          uuid resource_id FK
          datetime start_time
          datetime end_time
          string status "pending, confirmed, completed, cancelled"
          string payment_intent_id "Stripe reference"
      }
  ```
  *Note: All tables must use `tenant_id` for row-level security.*

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager")**:
    - **Trigger**: Receives natural language input like "Block off next Friday, I'm taking a long weekend."
    - **Action**: Interacts with the `AvailabilityBlock` and `Booking` models to block the calendar, potentially prompting the user to reschedule existing bookings.
    - **Trigger**: External calendar sync (e.g., Google Calendar) detects a new event.
    - **Action**: Creates a corresponding `AvailabilityBlock` (as unavailable) and resolves conflicts.
  - **Sales/Customer Success Agent ("The Ambassador")**:
    - **Trigger**: A background job periodically scans for `Customer`s with past `Booking`s but no future `Booking`s within a certain timeframe (e.g., 3 weeks for a music student).
    - **Action**: Drafts a re-engagement message with a direct booking link and proposes it to the owner in the Unified Agent Feed.

  ### Mobile UX Flow (375px First)

  **1. Customer Booking Flow (Public Storefront):**
  - **Screen 1 (Service Selection):** Clean list of services with prices. Large tap targets.
  - **Screen 2 (Date/Time Selection):** A vertical scrollable list of available days, expanding into available time slots. No complex multi-month calendar views. Tap target for a slot >= 44px height.
  - **Screen 3 (Deposit & Confirm):** Integration with Stripe Elements. Large "Pay Deposit & Book" button sticking to the bottom of the screen.

  **2. Owner Management Flow (Dashboard):**
  - **Screen 1 (Unified Agent Feed / Agenda):** The main view is today's agenda interspersed with Agent action cards.
    - Card: "Leo, 3 students haven't booked a lesson this month. I drafted a message to them. [Review & Send]"
    - Agenda item: "2:00 PM - Piano Lesson with Sarah [Pending Deposit]"
  - **Screen 2 (Natural Language Setup):** Instead of a complex grid to set availability, a chat interface:
    - User: "I want to offer 30-min guitar lessons on Wednesdays for $30."
    - Agent: "Got it. I've created the '30-min Guitar Lesson' service and set your availability for Wednesdays. When do you want to start and end on Wednesdays?"

  ## Implementation Prompt
  Implement the OHC Native Agentic Booking System targeting Leo the Music Tutor.

  **User-Facing Outcome:**
  Leo can offer lesson packages via a simple mobile link. Customers can view his dynamic availability (which the Operations agent manages based on his natural language inputs and Google Calendar), select a slot, and pay a deposit. Leo sees his daily bookings in a mobile-friendly feed and receives AI-drafted follow-ups for students who miss a week.

  **Critical User Journey (CUJ):**
  1.  Leo logs into the OHC mobile app.
  2.  Leo tells the Assistant: "Set up a 1-hour advanced piano lesson service for $100, requiring a 50% deposit. I'm available Tuesdays and Thursdays 3pm-6pm."
  3.  The Agent confirms the creation.
  4.  A customer visits Leo's OHC booking link on their phone, selects a Thursday 4pm slot, and pays the $50 deposit via Stripe.
  5.  Leo receives a push notification and sees the confirmed booking in his daily agenda view.
  6.  (Fast forward 3 weeks) The Sales Agent presents a card in Leo's feed: "Customer [Name] hasn't booked since their last lesson. Drafted check-in message. [Send]"

  **Acceptance Criteria:**
  -   Data models implemented with strict multi-tenant RLS.
  -   Customer booking flow is 100% functional and responsive on a 375px viewport.
  -   Stripe deposit payment is integrated and robust against network flakiness.
  -   Operations agent can parse basic availability natural language and create Services/AvailabilityBlocks.
  -   E2E Playwright test covers the full flow from service creation (by owner) -> booking and payment (by customer) -> viewing the booking (by owner).

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
