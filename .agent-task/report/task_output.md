issue_title: "Implement Unified Booking, Quoting & Deposit Engine for Services"
issue_description: |
  # Research Report
  Service-based small business owners currently lack a unified way to manage complex scheduling, custom quoting, and deposit collection across disparate tools (phone calls, SMS, external calendars, payment links). Managing these operations leads to lost leads, no-shows, and delayed payments. They need a single, mobile-first engine that handles everything from an initial inquiry to final payment seamlessly.

  ## Problem Statement
  Service providers like Leo the music tutor and Carlos the handyman need a unified engine to generate custom quotes, offer calendar timeslots, and secure deposits seamlessly from a mobile device without relying on disconnected tools like WhatsApp and separate Stripe links.

  ## Findings and Competitive Analysis
  - **Gap**: A robust quoting and deposit mechanism intertwined with a calendar scheduling system is missing.
  - **Competitive Analysis**: While Square Appointments is strong in booking/payments, it's rigid in custom quoting workflows. Calendly excels at scheduling but lacks integrated quoting/split deposit structures. Jobber is too complex for simple setups.
  - **Solution**: A unified engine that allows an AI Agent to dynamically generate a custom quote, propose timeslots from a live calendar, and collect a deposit to secure the booking, entirely via mobile.

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

  ### Mobile UX Flow (375px First)
  1.  **Inquiry Received:** User receives an SMS/WhatsApp inquiry. The AI Agent flags it in the OHC Mobile Dashboard under "Action Required".
  2.  **1-Tap Quote Generation:** User taps the inquiry. A half-sheet modal slides up showing an AI-drafted quote based on the service requested.
  3.  **Adjust & Send:** User can adjust the slider for the "Deposit %" or add a line item. Tapping "Send Quote & Timeslots" delivers a secure link to the customer.
  4.  **Customer View (No App Required):** The customer opens the link on their mobile browser. They see a clean, translucent glass-styled card summarizing the quote, a calendar widget to pick a time, and Apple/Google Pay buttons to instantly pay the deposit.
  5.  **Dashboard Update:** The OHC dashboard immediately moves the quote to "Confirmed Bookings" and updates the daily briefing.

  ### AI Agent Integration Points
  *   **The Vigilant Manager (Operations):** Watches inbound messages, extracts intent, drafts the quote, and proposes timeslots based on the calendar.
  *   **The Silent Ambassador (Customer Success):** Sends automated SMS reminders 24 hours before the booking and follows up post-service for reviews or final payment collection.
  *   **The Business Advisor:** Summarizes booking conversion rates in the daily brief.

  ## Implementation Prompt
  Implement the Unified Booking, Quoting & Deposit Engine.
  The system must support the end-to-end journey for a service provider: allowing the creation of custom quotes with variable deposit requirements, linking those quotes to specific calendar timeslots, and securely managing the transition from "Pending" to "Confirmed Booking" upon deposit payment.
  Ensure the UI components follow the macOS-style Translucent Glass materials and mobile-first card layouts. All complex configuration should be abstracted away, focusing on a 1-tap quote approval flow driven by the Operations AI Agent. Acceptance criteria include: successful generation of a quote, customer acceptance with a deposit payment, and automated calendar blocking.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
