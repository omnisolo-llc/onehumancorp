issue_title: "Implement Unified Booking, Quoting & Deposit Engine for Services"
issue_description: |
  ## Problem Statement
  Service-based small business owners (like Carlos the handyman and Leo the music tutor) currently lack a unified way to manage complex scheduling, custom quoting, and deposit collection. Managing these operations across disparate tools (phone calls, SMS, external calendars, and separate payment links) leads to lost leads, no-shows, and delayed payments. They need a single, mobile-first engine that handles everything from an initial inquiry to final payment seamlessly.

  ## Research Report
  *   **Current Capabilities:** OHC has basic storefront setups, but it lacks a robust quoting and deposit mechanism intertwined with a calendar scheduling system.
  *   **Competitor Analysis:**
      *   *Square Appointments:* Strong in booking and payments but rigid in custom quoting workflows.
      *   *Jobber / Housecall Pro:* Great for field service quoting but often too complex for simple tutors or independent contractors, and lacking in automated AI follow-ups.
      *   *Calendly:* Excellent for scheduling but limited in integrated quoting and split deposit structures.
  *   **Gap Identified:** A unified engine that allows an AI Agent to dynamically generate a custom quote, propose timeslots from a live calendar, and collect a deposit to secure the booking, entirely via mobile.
  *   **Strategic Advantage:** By integrating the AI Operations Agent directly into the quoting and booking lifecycle, OHC can eliminate the back-and-forth friction that currently causes solopreneurs to lose 30-40% of potential bookings.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ SERVICE : offers
      SERVICE ||--o{ QUOTE : generates
      TENANT ||--o{ BOOKING : manages
      QUOTE ||--|{ BOOKING : transitions_to
      BOOKING ||--o{ INVOICE : requires
      INVOICE ||--|{ PAYMENT : records

      TENANT {
          string id PK
          string name
          string timezone
      }
      SERVICE {
          string id PK
          string tenant_id FK
          string type "Fixed | Custom Quote"
          int duration_minutes
      }
      QUOTE {
          string id PK
          string tenant_id FK
          string status "Draft | Sent | Accepted"
          float total_amount
          float required_deposit
          datetime expires_at
      }
      BOOKING {
          string id PK
          string tenant_id FK
          string quote_id FK
          datetime start_time
          datetime end_time
          string status "Pending Deposit | Confirmed | Completed"
      }
      INVOICE {
          string id PK
          string tenant_id FK
          string type "Deposit | Final"
          float amount
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant AIAgent as The Vigilant Manager (AI)
      participant OHC_Engine as Booking & Quoting Engine
      participant PaymentGateway as Payment / Ledger
      participant Calendar as Calendar Sync

      Customer->>AIAgent: "I need a quote for a 2-hour piano lesson."
      AIAgent->>OHC_Engine: Check Leo's availability & pricing rules
      OHC_Engine-->>AIAgent: Available times & $100 total ($25 deposit)
      AIAgent->>Customer: "I can do 4 PM tomorrow. It's $100 ($25 deposit). Secure it here: [Link]"
      Customer->>OHC_Engine: Clicks link, views Quote & Booking
      Customer->>PaymentGateway: Pays $25 Deposit
      PaymentGateway-->>OHC_Engine: Deposit Confirmed
      OHC_Engine->>Calendar: Block Calendar Slot
      OHC_Engine->>AIAgent: Trigger confirmation SMS
      AIAgent->>Customer: "Booked! See you tomorrow."
  ```

  ### Mobile UX Flow (375px First)
  1.  **Inquiry Received:** User (Carlos/Leo) receives an SMS/WhatsApp inquiry. The AI Agent flags it in the OHC Mobile Dashboard under "Action Required".
  2.  **1-Tap Quote Generation:** User taps the inquiry. A half-sheet modal slides up showing an AI-drafted quote based on the service requested.
  3.  **Adjust & Send:** User can adjust the slider for the "Deposit %" or add a line item. Tapping "Send Quote & Timeslots" delivers a secure link to the customer.
  4.  **Customer View (No App Required):** The customer opens the link on their mobile browser. They see a clean, translucent glass-styled card summarizing the quote, a calendar widget to pick a time, and Apple/Google Pay buttons to instantly pay the deposit.
  5.  **Dashboard Update:** The OHC dashboard immediately moves the quote to "Confirmed Bookings" and updates the daily briefing. All complex terms (invoices, ledgers) are hidden.

  ### AI Agent Integration Points
  *   **The Vigilant Manager (Operations):** Watches inbound messages, extracts intent (e.g., "fix my sink", "piano lesson"), drafts the quote, and proposes timeslots based on the calendar.
  *   **The Silent Ambassador (Customer Success):** Sends automated SMS reminders 24 hours before the booking and follows up post-service for reviews or final payment collection.
  *   **The Business Advisor:** Summarizes booking conversion rates in the daily brief (e.g., "Your new deposit rule increased secured bookings by 15%.").

  ### Performance & Security Integrity
  *   **Zero-Trust Isolation:** Quotes, Bookings, and Calendars are strictly partitioned by `tenant_id`.
  *   **Edge-Caching:** Public quote links must be edge-cached for instant loading, with dynamic elements (available timeslots) loaded asynchronously.
  *   **Offline Tolerance:** Draft quotes created offline on the mobile device are queued locally and synced to the cloud when a connection is restored.

  ## Implementation Prompt
  Implement the Unified Booking, Quoting & Deposit Engine.
  The system must support the end-to-end journey for a service provider: allowing the creation of custom quotes with variable deposit requirements, linking those quotes to specific calendar timeslots, and securely managing the transition from "Pending" to "Confirmed Booking" upon deposit payment.
  Ensure the UI components follow the macOS-style Translucent Glass materials and mobile-first card layouts. All complex configuration should be abstracted away, focusing on a 1-tap quote approval flow driven by the Operations AI Agent. Acceptance criteria include: successful generation of a quote, customer acceptance with a deposit payment, and automated calendar blocking. Do NOT prescribe specific database schemas, API endpoints, or function signatures. Let the implementer design those.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []