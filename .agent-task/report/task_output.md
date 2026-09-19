issue_title: "OmniSolo Appointment Scheduling & Automated Deposit Integration"
issue_description: |
  # OmniSolo Appointment Scheduling & Automated Deposit Integration

  **Priority:** P1
  **Estimated Scope:** Medium

  ## Problem Statement
  Small business operators like Carlos (field services) and Leo (music tutor) rely on appointments to drive revenue. Currently, non-technical owners face significant friction using fragmented scheduling tools (like Calendly or Acuity) that do not natively integrate with their payment systems, CRM, or multi-channel communications. They struggle with manual quoting, no automated deposit enforcement, and missing leads when busy. They need a unified system where scheduling an appointment automatically handles calendar sync, deposit collection, and customer follow-up without requiring complex integrations or technical setup.

  ## Research Report

  ### Market Benchmarking: Scheduling Domain
  1. **Calendly**
     - *Onboarding:* Fast, but focused purely on meeting scheduling.
     - *Mobile:* Good for viewing appointments, but setting up complex event types (deposits, routing) requires desktop.
     - *Pricing:* Free tier is limited. Pro tier required for routing and payment integrations ($12-$16/mo).
     - *Friction:* Connects to external calendars and Stripe, but remains a separate silo from the core CRM and project management.

  2. **Acuity Scheduling (by Squarespace)**
     - *Onboarding:* Heavier setup, geared towards service businesses.
     - *Mobile:* Dedicated admin app, functional but cluttered.
     - *Pricing:* $20-$50/mo depending on features (SMS reminders, multiple staff).
     - *Friction:* Deeply tied to Squarespace ecosystem; standalone use can feel isolated from a unified business OS.

  3. **Square Appointments**
     - *Onboarding:* Integrated with Square POS.
     - *Mobile:* Excellent mobile-first approach for service providers (POS + Calendar in one app).
     - *Pricing:* Free for individuals, scales up for teams ($29+/mo).
     - *Friction:* Traps users in the Square ecosystem; less flexible for online-only businesses or digital products.

  ### User Sentiment & Concrete Pain Points
  - "I use Calendly for booking, but I still have to manually send a Stripe invoice for the deposit. Sometimes they don't pay in time and I lose the slot." (Carlos - Handyman)
  - "Acuity is great, but it doesn't talk to my CRM or my marketing emails. I have to use Zapier to sync contacts, which breaks all the time." (Leo - Tutor)
  - "I just want a simple booking link on my Instagram that takes a 50% deposit and puts the appointment on my Google Calendar. Why is that so hard?" (Maya - Baker)

  ### Verified Sources
  1. Calendly Pricing & Features: https://calendly.com/pricing
  2. Acuity Scheduling Pricing: https://www.squarespace.com/scheduling/pricing
  3. Square Appointments: https://squareup.com/us/en/appointments
  4. Reddit /r/smallbusiness: Discussions on scheduling workflows and deposit friction.
  5. Trustpilot reviews for Calendly and Acuity noting integration difficulties.

  ### OHC Gap & Invisible Agentic Solution
  - **Gap:** OmniSolo lacks a native, seamlessly integrated booking widget that couples time-slot selection with mandatory deposit collection and CRM synchronization.
  - **Solution:** Implement a native scheduling module within OmniSolo. When a user creates a "Service" (e.g., 1-hour consultation), they can toggle "Require Deposit". The system automatically generates a booking link. When a customer books, the "Scheduler & Productivity Agent" coordinates with the "POS & Invoicing Agent" to process the payment before finalizing the calendar slot, and the "Sales & CRM Agent" logs the new interaction.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Booking_Widget
      participant Scheduler_Agent
      participant Invoicing_Agent
      participant CRM_Agent
      participant External_Calendar

      Customer->>OHC_Booking_Widget: Selects Time Slot
      OHC_Booking_Widget->>Scheduler_Agent: Request Hold
      Scheduler_Agent->>External_Calendar: Check Availability
      External_Calendar-->>Scheduler_Agent: Confirmed
      Scheduler_Agent-->>OHC_Booking_Widget: Prompt for Deposit
      Customer->>OHC_Booking_Widget: Submits Payment
      OHC_Booking_Widget->>Invoicing_Agent: Process Payment
      Invoicing_Agent-->>Scheduler_Agent: Payment Confirmed
      Scheduler_Agent->>External_Calendar: Finalize Booking
      Scheduler_Agent->>CRM_Agent: Create/Update Contact
      Scheduler_Agent-->>Customer: Send Confirmation (Email/SMS)
  ```

  ### UI/UX Flow (375px Mobile First)
  1. **Owner View (Service Creation):**
     - Single screen form to define Service Name, Duration, and Price.
     - Toggle switch: "Require deposit at booking" (displays input for % or fixed amount).
     - "Generate Booking Link" button (copies to clipboard).
  2. **Customer View (Booking):**
     - Clean, OHC Premium Token styled calendar picker.
     - Next step: Simple form (Name, Email, Phone).
     - Final step: Integrated Apple Pay/Google Pay/Card input for deposit.
     - Success screen with "Add to Calendar" button.

  ## Implementation Prompt
  Implement the "OmniSolo Native Scheduler" feature. The Critical User Journey (CUJ) involves an owner (e.g., Carlos) creating a new service offering that requires a 20% deposit. The owner generates a booking link and shares it. A customer follows the link, selects an available time slot, enters their details, and pays the deposit via a simulated payment integration. The system must then persist the booking, block the time slot, create/update the customer record in the CRM, and display the appointment on the owner's OmniSolo dashboard. The solution should leverage existing open-source date-picking libraries (e.g., `react-day-picker`) and ensure a flawless 375px mobile experience.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []