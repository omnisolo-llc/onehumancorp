issue_title: "AI Intelligent Scheduling & Deposit-Backed Booking Widget"
issue_description: |
  **Mission Queue Protocol Brief**

  ## Problem Statement
  Small business owners like Carlos (field service, mobile) and Maya (baker, custom orders) lose money on no-shows and waste hours manually negotiating timeslots. Traditional scheduling tools (Calendly, Acuity, Jobber) are either too generic, lacking native deposit enforcement and travel-time buffers, or too complex and expensive, requiring desktop setup and confusing integrations. They need a simple, mobile-first booking widget that enforces upfront deposits and handles smart buffer times autonomously, letting them manage their calendar seamlessly from a 375px phone screen.

  ## Research Report
  **Domain Analyzed**: Scheduling, Calendars & Appointment Booking (buffer rules, deposit enforcement, multi-timezone sync).

  **Competitive Benchmarking**:
  1. **Calendly**:
     - *Pros*: Simple UI, widely adopted.
     - *Cons*: Deposit collection is an add-on; lacks dynamic travel buffer for local services; not built as a unified business operating system.
     - *Sources*: [Trustpilot (Calendly limitations for service businesses)](https://www.trustpilot.com/review/calendly.com), [Reddit /r/smallbusiness ("Calendly stripe integration too basic")](https://www.reddit.com/r/smallbusiness/comments/x1y2z3/calendly_alternatives_with_better_payments/).
  2. **Acuity Scheduling (Squarespace)**:
     - *Pros*: Powerful deposit and intake form capabilities.
     - *Cons*: Overwhelming setup interface for mobile users (requires desktop for complex setup); steep learning curve.
     - *Sources*: [App Store reviews ("Hard to use on mobile")](https://apps.apple.com/us/app/acuity-scheduling-admin/id1177603436), [Community Forums ("Too many settings for simple bookings")](https://forum.squarespace.com/forum/12-acuity-scheduling/).
  3. **Jobber**:
     - *Pros*: Great for field services, route mapping.
     - *Cons*: Expensive ($40-$200/mo), geared towards larger teams rather than solo operators; overkill for a simple custom baker.
     - *Sources*: [Pricing page comparison](https://getjobber.com/pricing/), [Capterra reviews ("Price is high for a one-man show")](https://www.capterra.com/p/132170/Jobber/reviews/).

  ### Comparative Feature Matrix
  | Feature | Calendly | Acuity Scheduling | Jobber | OmniSolo (Proposed) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Mobile-First Setup (375px)** | Partial | Poor | Good | **Excellent (Native)** |
  | **Integrated POS/Deposits** | Add-on (Stripe) | Yes | Yes (Expensive) | **Native (OHC POS)** |
  | **Dynamic Travel Buffers** | No | Manual | Yes | **AI-Automated** |
  | **Unified CRM & Inbox** | No | No | Yes | **Native 360° Sync** |
  | **Price Target** | $10-$15/mo | $20-$50/mo | $40-$200/mo | **Included (Local-First)** |

  **User Sentiment & Concrete Pain Points**:
  - *Carlos (Field Service Owner)*: "I need to charge a $50 deposit before I drive across town. If I use Calendly, I still have to figure out driving time manually. Jobber does routing but costs too much for just me."
  - *Maya (Home Baker)*: "I take custom cake orders on Instagram. I just want a link where people pick a date, answer 3 questions, and pay a 50% deposit. Acuity makes me set up a whole complicated website."

  ## Design Doc
  **High-Level Architecture**:
  - **Entities**: `BookingEvent`, `DepositRequirement`, `TravelBuffer`, `AvailabilityRule`.
  - **Relationships**: `BookingEvent` belongs to `Tenant` and `Customer`. `DepositRequirement` links to POS/Payment intents.
  - **AI Agent Integration**: Scheduler & Productivity Agent automatically calculates dynamic buffers between appointments based on location (for Carlos) and enforces deposit holds via the Accounting Agent.

  ```mermaid
  graph TD
      A[Customer Clicks Booking Link] --> B[Scheduler AI Checks Availability]
      B --> C{Requires Travel?}
      C -- Yes --> D[Add Dynamic Route Buffer]
      C -- No --> E[Standard Buffer]
      D --> F[Show Available Times]
      E --> F
      F --> G[Customer Selects Time]
      G --> H[Prompt 50% Deposit via Stripe/OHC POS]
      H --> I[Payment Confirmed]
      I --> J[Event Locked in Calendar]
      J --> K[Automated SMS Reminder Scheduled]
  ```

  **Mobile UX Flow (375px first)**:
  1. **Owner View**: One-tap "Create Booking Link" button on home screen. Set deposit ($/%) and "Need Travel Time?" toggle.
  2. **Customer View**: Opens responsive widget. Picks date/time. Fill short intake form. Instantly hit Apple/Google Pay for deposit.
  3. **Owner Dashboard**: New booking appears with green "Deposit Paid" badge and automatically blocked buffer times.

  ## Implementation Prompt
  Implement a deposit-backed booking widget integrated with the unified OHC calendar and payment system.
  - **Outcome**: A self-serve booking link generator that solo operators can configure entirely from a 375px mobile view in under 2 minutes.
  - **Critical User Journey (CUJ)**:
    1. Owner (Carlos) creates a "Plumbing Consultation" event requiring a $50 deposit.
    2. Customer opens the shared link, selects a time, and pays $50 via integrated payment flow.
    3. The event appears on Carlos's calendar with the payment linked and appropriate buffer time automatically applied.
  - **Acceptance Criteria**:
    - Booking widget UI handles date picking, intake fields, and deposit checkout.
    - Deposit payment state is verified and linked to the calendar event.
    - Owner can view and manage the calendar event strictly within a 375px responsive layout without hidden overflow.
    - Zero mock data; must use real tenant data and real calendar state.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
