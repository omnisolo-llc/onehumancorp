issue_title: "[Research] AI-Native Automated Quoting & Proposal Generation"
issue_description: |
  # Research Report: AI-Native Automated Quoting & Proposal Generation

  ## Executive Summary
  Service-based businesses (like field services and creative agencies) face significant friction when converting inquiries into paid work. Traditional tools require the owner to manually translate customer needs into itemized estimates, often leading to delayed responses and lost revenue. OneHumanCorp (OHC) can solve this by introducing an Agentic Quoting & Proposal system that listens to customer inquiries, extracts requirements, checks inventory/availability, and autonomously drafts professional, mobile-first estimates ready for the owner's 1-tap approval.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  The quoting and estimating landscape is heavily fragmented:
  - **Field Service Management (FSM) Giants (Jobber, Housecall Pro, ServiceTitan):** Powerful but complex. They require manual data entry for every quote and are designed as heavy CRMs rather than proactive assistants.
  - **Agency/Creative Tools (HoneyBook, Dubsado):** Excellent for beautiful proposals and client portals, but lack autonomous AI drafting. They are templates waiting to be filled out.
  - **E-commerce Platforms (Shopify, Square):** Optimized for fixed-price products, not dynamic service estimates based on custom requirements.

  **The OHC Gap:** Existing solutions are "empty vessels" that wait for the user to do the work. OHC must provide an active **Sales Agent** that drafts the quote the moment the lead arrives in the unified inbox.

  ## 2. Deep Dive Architecture Design (Track 2)

  ### Target Personas
  - **Carlos (Handyman, 42):** Needs fast, simple estimates generated from text messages or phone calls while on the road, with an integrated deposit link.
  - **Nora (Agency Principal, 39):** Needs professional proposals drafted from client intake forms, including project phases and contractor assignments.

  ### Data Model & Sync Protocol
  - `QuoteRequest`: Captures the raw inquiry (from DM, form, or call transcript).
  - `Estimate`: The structured proposal (line items, taxes, deposit requirements, validity period).
  - `ServiceItem`: Reusable catalog of services with dynamic pricing rules.

  ### AI Agent Coordination
  - **Sales Agent ("The Closer"):** Ingests the `QuoteRequest`. Uses RAG against the `ServiceItem` catalog and past accepted quotes to draft an `Estimate`. It calculates costs and suggests a deposit amount.
  - **Operations Agent ("The Manager"):** Validates the drafted quote against schedule availability and resource constraints before it is presented to the owner for approval.

  ### Mermaid Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant WorkTriage as Work Triage
      participant SalesAgent as Sales Agent
      participant OpsAgent as Operations Agent
      participant OwnerApp as OHC Mobile App (Owner)

      Customer->>WorkTriage: Sends inquiry (e.g., "Need kitchen sink fixed")
      WorkTriage->>SalesAgent: Passes parsed intent & context
      SalesAgent->>OpsAgent: Checks resource availability
      OpsAgent-->>SalesAgent: Confirms slot available
      SalesAgent->>SalesAgent: Drafts Estimate (Parts, Labor, Deposit)
      SalesAgent->>OwnerApp: Pushes notification with drafted quote
      OwnerApp->>OwnerApp: Owner reviews on 375px screen
      OwnerApp-->>Customer: Owner taps "Approve & Send Link"
  ```

  ## 3. Technical Integrity & Mobile-First Review (Track 3)

  ### Mobile-First UX Flow (375px)
  1. **Notification:** Carlos receives a push: "New lead: Kitchen sink repair. Quote drafted."
  2. **Review Screen:** A clean, translucent card interface. The top shows the customer's original message. Below is the itemized AI-drafted quote (e.g., $50 parts, $150 labor).
  3. **Interaction:** Touch targets are large (44x44px min). Carlos can tap "Edit Items" to adjust prices via native number pads, or simply swipe/tap "Approve & Send".
  4. **Customer View:** The customer receives an SMS/Email link leading to a mobile-optimized, branded approval page with a Stripe Checkout deposit flow.

  ### Performance & Security
  - The quoting engine operates as an asynchronous background job (PostgreSQL `SKIP LOCKED` pattern).
  - Strict row-level security (RLS) ensures quotes and service catalogs are isolated by `tenant_id`.
  - The drafted quote remains in a "pending_approval" state; the AI cannot bind the business to a contract without explicit owner consent.

  ## 4. Implementation Prompt (Track 4)

  **Feature Name:** AI-Native Automated Quoting & Proposal Generator

  **Target Personas:** Carlos the Handyman, Nora the Agency Principal

  **Outcome:** An automated pipeline where customer inquiries are transformed into itemized estimates by the Sales Agent. The owner receives a push notification and can approve and send the quote with a single tap on their mobile device.

  **Critical User Journey (CUJ):**
  1. Carlos is on a job site. A new customer texts his business number: "Can you fix a leaky pipe tomorrow?"
  2. The system's Work Triage captures the message. The Sales Agent identifies the intent as a service request.
  3. The Sales Agent pulls the "Standard Plumbing Repair" rate from Carlos's catalog, checks tomorrow's availability via the Operations Agent, and drafts a $150 estimate.
  4. Carlos receives a notification on his Android phone, opens the OHC app (375px view), reviews the line items, and taps "Approve & Send."
  5. The customer receives a payment link to approve the quote and pay a deposit.

  **Next Actions for Engineering:**
  - **Step 1:** Define the core `Estimate` and `EstimateLineItem` schema with RLS and multi-tenant isolation.
  - **Step 2:** Implement the Sales Agent prompt and tool functions to draft estimates based on incoming unstructured text and the user's service catalog.
  - **Step 3:** Build the mobile-first (375px) Quote Review UI (glassmorphism cards, large touch targets) allowing the owner to edit or approve the draft.
  - **Step 4:** Integrate with Stripe Payment Links for the customer-facing deposit collection.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
