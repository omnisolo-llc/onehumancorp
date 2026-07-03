issue_title: "[Platform Research] Optimize OHC AI Automation Strategies for Local Service Businesses"
issue_description: |
  # Research Report: AI Autonomous Booking & Service Automation for OHC

  ## Problem Statement
  Local service businesses (like Carlos the Handyman or Fatima the Food Cart Operator) struggle with disjointed workflows. While dogfooding the OHC platform via local deployment (docker compose up --build), I attempted to set up a service booking flow as "Carlos". The current UI requires navigating multiple disparate screens: adding a service in the catalog, manually checking availability, and creating a manual quote for an external payment link. There is no automated conversational booking flow that natively integrates the `Work Triage` agent with calendar availability and quote generation. This manual pipeline results in lost leads, double bookings, and excessive administrative overhead.

  ## Market & Competitive Research
  - **Market Gap:** While e-commerce platforms handle physical goods well, service-based solopreneurs are underserved. They need an integrated solution that handles scheduling, quoting, and deposit collection in a single, autonomous flow.
  - **Competitor Landscape:**
    - **Shopify:** Requires complex 3rd-party apps for booking.
    - **Wix/Squarespace:** Offer basic manual scheduling, but no AI-driven conversational booking or automated quoting based on context.
    - **OHC Opportunity:** By integrating an AI conversational agent that can parse an inquiry ("Can you fix a leaky pipe on Tuesday?"), check availability, generate a preliminary quote, and send a booking link with a deposit request, OHC can own the service sector.

  ## Design Doc: High-Level Architecture
  ### Architecture & Flow
  - **Work Triage Ingestion:** The `Work Triage` agent listens to incoming channels (SMS, DMs, Web Chat).
  - **Intent Parsing:** AI identifies "booking inquiry" or "quote request" and extracts required parameters (service type, date/time, location).
  - **Operations Coordination:** The `Operations Assistant` queries the tenant's calendar/availability module.
  - **Sales/Quoting Generation:** The `Sales & Revenue Assistant` generates a standardized quote based on predefined service rates and drafts a reply with a booking/deposit link (Stripe Payment Link).
  - **Owner Approval (HITL):** A simple push notification is sent to the owner's mobile device (375px view).

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer
      participant WT as Work Triage Agent
      participant OA as Operations Assistant
      participant SA as Sales Assistant
      participant O as Owner (Mobile UI)

      C->>WT: "Can you fix a pipe on Tuesday?"
      WT->>OA: Check availability for Tuesday
      OA-->>WT: Tuesday 2 PM available
      WT->>SA: Request preliminary quote for "pipe fix"
      SA-->>WT: Quote: $50 + Deposit Link
      WT->>O: Draft Notification: "Approve quote & booking link?"
      O->>WT: Tap "Approve & Send"
      WT->>C: "Hi! I can fix it Tuesday at 2 PM. Quote is $50. Pay deposit here."
  ```

  ### Mobile UX Flow (375px)
  1. **Notification Card:** A unified inbox card highlights a new service request.
  2. **AI Suggested Draft:** A single tap reveals the AI-drafted reply, including the quote and booking link.
  3. **One-Tap Action:** "Approve & Send" button prominently displayed.
  4. **Confirmation State:** Visual confirmation that the message was sent and the time slot is tentatively held.

  ## Implementation Prompt
  **Target Implementer:** Operations & Sales Agent Developer
  **User-Facing Outcome:** When a customer sends a message requesting a service, the AI should automatically draft a reply that includes a preliminary quote and a link to book/pay a deposit, presenting it to the owner for one-tap approval.
  **CUJ & Acceptance Criteria:**
  1. User (e.g., Carlos) receives a message: "Need my lawn mowed this Friday."
  2. The system auto-generates a draft: "Hi! I can mow your lawn this Friday. The estimated cost is $50. Please confirm your booking and pay the deposit here: [Link]."
  3. The owner can view this draft on a 375px screen and approve it with a single tap.
  4. The system updates the calendar with a tentative hold.
  **Technical Note:** Focus on the conversational parsing, quote generation logic, and the mobile-first approval UI. Do not hardcode specific DB schemas or external API endpoints.

  ## Priority & Scope
  **Priority:** P1 (High)
  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
