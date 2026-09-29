issue_title: "Research: Support for Connected Scheduling & Booking Operations"
issue_description: |
  # Mission Queue Protocol: Research Report for Connected Scheduling & Booking Operations

  ## Problem Statement
  Solo service professionals (like Maya the baker, Leo the music tutor, or a solo web designer) struggle to manage bookings and reservations efficiently. Existing solutions either require manual intervention, lack integration with their existing AI tools, or fail to handle complex scheduling scenarios (e.g., deposits, timezones, conflict resolution). This leads to lost revenue, double bookings, and increased administrative burden.

  ## Research Report
  - **Market Landscape:** Current tools like HoneyBook offer basic scheduling and automations, but they often require significant setup and may not integrate seamlessly with the operator's existing workflow. General AI assistants can draft emails but cannot reliably execute complex, stateful booking logic without hallucinating or requiring constant supervision.
  - **User Pain Points:** Operators report frustration with the time spent managing schedules, the complexity of setting up new systems, and the inability of current tools to handle edge cases like cancellations, refunds, and rescheduling autonomously.
  - **OHC Gap:** The codebase currently has foundational elements for booking and calendars (e.g., Google Calendar client, basic booking services), but it lacks a robust, verifiable end-to-end flow that handles conflict resolution, deposits, and status updates reliably under delegated authority. The existing implementation may use placeholders or simulated behavior that needs to be replaced with actual, verifiable logic.

  ### Verified Sources
  1. [Federal Reserve, 2026 employer report](https://www.fedsmallbusiness.org/reports/survey/2026/2026-report-on-employer-firms) - Highlights reaching customers and growing sales as top operational challenges.
  2. [Federal Reserve, nonemployer report](https://www.fedsmallbusiness.org/reports/survey/2025/2025-report-on-nonemployer-firms) - Shows reliance on owners' personal funds and distinct needs for solo businesses.
  3. [OECD, Generative AI and the SME Workforce](https://www.oecd.org/en/publications/generative-ai-and-the-sme-workforce_2d08b99d-en.html) - Generative AI use is at 31%, workload reductions in about a third, but skill needs continue.
  4. [HoneyBook Automations Guide](https://help.honeybook.com/en/articles/6613606-start-automating-your-booking-process-in-honeybook) - Demonstrates the need for automated client files, contracts, and invoices within booking.
  5. [Jobber Features](https://www.getjobber.com/features/) - Indicates that isolated booking is insufficient; the connected customer-to-cash process is required.
  6. [Stripe Checkout Fulfillment](https://docs.stripe.com/checkout/fulfillment) - Documents the necessity of handling webhook-driven fulfillment and repeated/concurrent fulfillment reliably.

  ## Comparative Feature Matrix

  | Feature | HoneyBook | Jobber | Claude for Small Business | OHC (Proposed) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Scheduling & Booking** | Yes | Yes | Limited (Chat) | **Yes (Autonomous)** |
  | **Payment & Deposits** | Yes | Yes | Limited | **Yes (Native Integration)** |
  | **Conflict Resolution** | Basic | Basic | No | **Yes (Robust & Automated)** |
  | **Standing Authority execution** | No | No | No (Requires approval) | **Yes** |
  | **Context Preservation** | Yes (CRM) | Yes (CRM) | Partial | **Yes (Deep & Persistent)** |

  ## Design Doc
  - **Core Entities:** `BookingRequest`, `Reservation`, `AvailabilitySlot`, `PaymentDeposit`.
  - **Key Relationships:** A `BookingRequest` leads to a `Reservation`, which is linked to a specific `AvailabilitySlot` and a `PaymentDeposit` (if applicable).
  - **Architecture:** Enhance the existing booking service to handle concurrent requests and conflict resolution. Integrate deeply with the calendar client to verify availability in real-time. Implement state transitions (e.g., Pending -> Reserved -> Confirmed -> Completed/Cancelled).
  - **Mobile UX Flow (375px):**
    1. Operator views upcoming schedule and pending requests on a clear, single-column dashboard.
    2. Operator configures availability and deposit rules in simple, step-by-step settings screens.
    3. AI assistant presents actionable summaries of scheduling conflicts or necessary approvals.
  - **Mermaid Diagram (High-Level Architecture):**
    ```mermaid
    graph TD
        A[Client Request] --> B{Availability Check}
        B -->|Available| C[Reserve Slot]
        B -->|Unavailable| D[Suggest Alternatives]
        C --> E{Deposit Required?}
        E -->|Yes| F[Process Payment]
        E -->|No| G[Confirm Booking]
        F -->|Success| G
        F -->|Failure| H[Release Slot]
    ```

  ## Implementation Prompt
  Implement a robust, verifiable scheduling and booking engine. The system must support real-time availability checking, concurrent request handling without double booking, and integration with the existing payment and calendar services.
  - **Critical User Journey:** An operator configures their availability; a client requests a slot; the system checks availability, reserves the slot, processes a required deposit, and confirms the booking.
  - **Acceptance Criteria:**
    - Concurrent requests for the same slot must not result in double booking.
    - Reservations must be automatically released if a required deposit is not paid within the specified timeframe.
    - The system must handle timezones correctly.
    - All state transitions must be verifiable and logged.

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - **OHC Target ID:** OHC-05 (Accepted quote -> deposit -> conflict-free booking).
  - **Launch/Run Stage:** Run (ongoing operations).
  - **Observed Gap:** Existing booking logic uses placeholders and lacks robust conflict resolution and actual payment verification.
  - **Evidence Level:** Codebase audit indicates simulated behavior; market research highlights scheduling as a core pain point for solo operators.
  - **Authority Class:** Routine external work (executes under standing policy).
  - **Acceptance Checks:** Verify no double bookings on concurrent requests, successful state transitions for paid deposits, and correct capacity release on cancellation.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
