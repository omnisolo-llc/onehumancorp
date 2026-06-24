issue_title: "Implement Autonomous Quoting & Dynamic Scheduling Engine"
issue_description: |
  # Research Report: Autonomous Quoting & Dynamic Scheduling Engine

  ## 1. Problem Statement
  Service-based small business owners, specifically personas like **Carlos (Field Service Owner)**, suffer from severe operational friction. Currently, managing service requests requires disjointed, manual intervention. When a customer sends a request (e.g., via SMS or WhatsApp), Carlos has to manually read it, estimate the cost, check his calendar, generate a quote, send it back, and wait for confirmation. This fragmented workflow—often happening while he is on a job or driving—leads to lost leads, double-booking, and revenue leakage. The platform needs an intelligent, autonomous engine that handles quoting and dynamic scheduling in one seamless, invisible flow.

  ## 2. Research Report
  - **Market Context:** Traditional platforms (like Shopify or Wix) either lack native service-booking capabilities or rely on disjointed third-party apps (e.g., Calendly, dedicated quoting tools). These tools are passive; they wait for the user to configure availability and for the customer to self-serve.
  - **Competitor Gaps:**
    - *Shopify:* Treats bookings as physical products via clunky apps.
    - *Wix:* Basic booking forms but no proactive, agent-driven quoting or schedule negotiation.
    - *Calendly:* Detached from the core quoting and payment flow.
  - **OHC Opportunity ("Invisible Autonomy"):** OHC can differentiate by integrating an **Autonomous Quoting & Dynamic Scheduling Engine**. By leveraging the Ambassador Agent and Operations Agent, OHC can intercept a customer inquiry, autonomously generate a quote based on predefined service parameters, dynamically propose available calendar slots, and secure a deposit—all within a conversational thread and requiring minimal to no manual intervention from Carlos.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry via DM/SMS] -->|Ingests Message| B(Omnichannel Inbox Router)
      B --> C[Ambassador Agent]
      C -->|Intent: Quote Request| D[Sales Agent - Quote Engine]
      D -->|Requests Pricing & Rules| E[(Service Catalog DB)]
      D -->|Requests Availability| F[Operations Agent - Scheduling Engine]
      F -->|Checks Calendar| G[(Availability & Bookings DB)]
      D -->|Drafts Quote & Proposed Times| C
      C -->|Pushes Notification| H[Owner Mobile App]
      H -->|Owner Approves| I[Conversational Checkout Generated]
      I -->|Sends to Customer| A
      A -->|Customer Pays Deposit| J[Checkout Webhook]
      J -->|Locks Slot & Updates Ledger| F
  ```

  ### Data Model & Invariants
  - **ServiceCatalog:** `service_id`, `tenant_id`, `base_price`, `estimated_duration`.
  - **Quote:** `quote_id`, `tenant_id`, `customer_id`, `service_id`, `proposed_amount`, `status (draft, sent, accepted)`.
  - **BookingSlot:** `slot_id`, `tenant_id`, `start_time`, `end_time`, `status (available, soft_locked, booked)`.
  - **Invariants:**
    - Calendar slots must support a `soft_lock` mechanism (e.g., via Redis) during the checkout flow to prevent double booking.
    - Strict row-level security (RLS) ensuring `tenant_id` isolation for all quotes and bookings.

  ### Mobile UX Flow (375px First)
  1. **Notification:** Carlos receives a push notification: "New service request from John. Quote drafted."
  2. **Review Modal:** Carlos opens the app to a clean, glassmorphic card showing the customer's message, the AI-estimated quote, and proposed times.
  3. **One-Tap Action:** Carlos taps "Approve & Send".
  4. **Customer Experience:** The customer receives the quote via their original channel with a zero-click checkout bubble to pay the deposit and confirm the time slot.

  ### AI Agent Integration Points
  - **Ambassador Agent:** Parses incoming messages to extract service type and urgency.
  - **Operations Agent:** Interacts with the scheduling database to find optimal, contiguous time slots avoiding travel-time conflicts.
  - **Sales Agent:** Calculates the quote based on the service catalog and historical pricing.

  ## 4. Implementation Prompt
  **For the Implementer Agent:**
  Implement the Autonomous Quoting and Dynamic Scheduling Engine tailored for service operators.
  - **User-Facing Outcome:** When a customer requests a service via the omnichannel inbox, the system must autonomously draft a quote and propose available times. The owner (e.g., Carlos) should be able to review and approve this draft with a single tap on a 375px mobile UI, sending a seamless booking and deposit link back to the customer.
  - **Acceptance Criteria:**
    - Implement the database schemas for `Quote` and `BookingSlot` with strict RLS multi-tenant isolation.
    - Create the backend service layer that connects the Ambassador Agent's intent parsing to the quoting and scheduling logic.
    - Build the mobile-first (375px) approval card UI where the owner can review, edit, or approve the generated quote and schedule.
    - Provide Playwright E2E tests verifying the complete flow: from an incoming mock service request, to quote generation, owner approval, and the final state where a calendar slot is locked.
  - **Priority:** P0
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
