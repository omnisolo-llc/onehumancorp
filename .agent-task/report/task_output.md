issue_title: "Implement Universal Autonomous Staff Management Mesh"
issue_description: |
  # Title: Universal Autonomous Staff Management Mesh

  ## Problem Statement
  Small business owners often manage shift workers, independent contractors, or gig workers (e.g., Carlos the handyman coordinating with subcontractors, Maya the baker hiring weekend help). Managing staff schedules, tracking hours, calculating payouts, and communicating shift changes happens across scattered tools (WhatsApp, paper schedules, disparate payroll apps). This causes missed shifts, incorrect payments, and significant administrative burden.
  OHC needs an invisible, autonomous engine that handles scheduling, time tracking, and automated payout preparation across a unified mesh, keeping the non-technical owner out of the weeds.

  ## Research Report
  *   **Current State:** OHC lacks a unified staff management and scheduling architecture.
  *   **Competitor Analysis:**
      *   *Homebase / When I Work:* Powerful, but standalone apps that require business owners to sync data manually with their POS or booking system. Too heavy for a solopreneur just hiring their first contractor.
      *   *Square Team Management:* Good integration with POS, but weak on autonomous AI-driven shift filling and contractor communication.
  *   **Proposed Solution:** A Universal Autonomous Staff Management Mesh. This engine integrates directly with the OHC Booking and POS engines. It uses AI to draft schedules based on demand forecasts, tracks time via simple mobile check-ins, and seamlessly pushes data to the OHC Universal Wallet Ledger for payouts.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ STAFF_MEMBER : employs
      STAFF_MEMBER ||--o{ SHIFT : scheduled_for
      STAFF_MEMBER ||--o{ TIMESHEET : logs
      SHIFT ||--o{ SHIFT_SWAP_REQUEST : can_have

      TENANT {
          string id PK
      }
      STAFF_MEMBER {
          string id PK
          string tenant_id FK
          string name
          string role
          float hourly_rate
          string contact_info
      }
      SHIFT {
          string id PK
          string tenant_id FK
          string staff_id FK
          datetime start_time
          datetime end_time
          string status "Scheduled | Completed | Missed"
      }
      TIMESHEET {
          string id PK
          string staff_id FK
          datetime clock_in
          datetime clock_out
          float total_hours
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  *   **Employee View (Mobile Web App - 375px):**
      *   **Action:** An employee opens the OHC mobile view from an SMS link.
      *   **Schedule Screen:** A clean, Unifi-style screen showing upcoming shifts. A large "Clock In" button appears when they are within 15 minutes of their shift.
      *   **Clock-In Action:** Tapping "Clock In" triggers an immediate geolocation or Wi-Fi confirmation (if enabled by the merchant). A satisfying green checkmark confirms the action.
      *   **Swap Requests:** A simple list of upcoming shifts with a "Request Swap" button, leading to a list of eligible co-workers.
  *   **Merchant View (OHC Mobile App - 375px):**
      *   **Dashboard Insight:** A card at the top shows "2 Staff Currently Clocked In" and a prompt to "Approve Timesheets for Payout" if the pay period ended.

  ### AI Agent Integration Points
  *   **Operations Agent:** Proactively messages staff ("Hey Alex, you're scheduled tomorrow at 9 AM. Confirm?"). If Alex declines, the agent automatically texts other available staff to cover the shift.
  *   **Finance Agent:** Periodically reviews approved timesheets and automatically drafts payout transfers in the Universal Ledger, asking the merchant for a single 1-tap approval to disburse funds.

  ### Key Design Decisions
  - **Mobile-First UX:** Staff receive SMS links to view schedules and tap a button to clock in/out on their own phones (no dedicated hardware required).
  - **Ledger Integration:** Timesheets are automatically converted to pending payouts in the Universal Ledger.
  - **Zero-Trust:** All endpoints must validate `tenant_id` rigorously. The engine authenticates via SPIFFE/SPIRE identity for all internal mesh communications.

  ## Implementation Prompt
  Implement the Universal Autonomous Staff Management Mesh backend architecture.
  1. Build the data models for Staff Members, Shifts, and Timesheets that support strict multi-tenant isolation.
  2. Implement a robust scheduling service that allows for shift creation, assignment, and swap requests, heavily utilizing event mesh notifications.
  3. Expose the API endpoints required for the mobile time-tracking flow (clock in/clock out).
  4. Integrate hooks into the AI Operations Agent's processing queue so it can autonomously manage shift communication and replacement requests.
  5. Provide complete unit and E2E test coverage for the scheduling and time-tracking flows. Focus on the data flow and the integration logic without prescribing the specific database technology or lower-level implementation details.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
