issue_title: "Implement Agent-Driven Field Service & Dynamic Resource Scheduling Architecture"
issue_description: |
  # Research Report: Agent-Driven Field Service & Dynamic Resource Scheduling Architecture

  ## Executive Summary
  This report investigates the architecture required to support dynamic field service scheduling, specifically for owner/operator personas like Carlos (the handyman). Current solutions often separate booking systems from customer communications, leading to disconnected workflows, double-booking, and missed revenue opportunities. The proposed architecture introduces a multi-agent system that seamlessly connects lead capture, automated quoting, optimized resource scheduling, and resilient offline capabilities.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Jobber, Housecall Pro, and ServiceTitan offer field service management tools but are often complex to set up and lack deep, invisible AI-agent integration. They require the owner to manually piece together scheduling, quoting, and customer follow-up. While Wix and Squarespace offer basic booking, they lack dynamic, location-aware resource allocation and proactive agentic workflows.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Carlos (Field Service Owner, Handyman). Uses an Android phone primarily.
  - **The Gap:** OHC currently lacks a unified architecture that handles location-aware, dynamic booking with integrated, automated quoting. Carlos needs an assistant that takes service requests (e.g., via SMS or a simple web form), instantly generates a quote based on location/complexity, locks the calendar slot to prevent collisions, and follows up if the lead goes cold—all without him navigating complex scheduling dashboards.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & System Integrity
  - **Dynamic Booking Ledger (PostgreSQL):** A unified table structure supporting multi-resource allocation (time, personnel, equipment). Strong multi-tenant isolation (`tenant_id`) enforced via RLS.
  - **Distributed Locks (Redis Redlock):** Critical for preventing double-booking during the quoting and confirmation phases. When a lead requests a specific time, a Redlock (e.g., `ohc:lock:{tenant_id}:booking_slot:{time_id}`) is acquired to tentatively hold the slot during quote negotiation.
  - **Offline/Resilient Booking State:** Field service often occurs in low-connectivity areas. The mobile client must support offline status updates (e.g., "Job Started", "Invoice Generated") utilizing eventual consistency and idempotency keys to sync with the central ledger upon reconnection.

  ### Multi-Agent Coordination
  - **Sales Agent (The Estimator):** Parses incoming service requests (via chat or form), queries the pricing rules, and drafts a contextual quote.
  - **Operations Agent (The Dispatcher):** Calculates travel time (using location heuristics), checks calendar availability, and coordinates with Redis to temporarily lock slots while the quote is pending.
  - **Customer Success Agent (The Follow-up):** Triggers if a quote remains unaccepted for X hours, sending a gentle SMS/Email nudge or offering a dynamic discount to close the deal.

  ### Mobile-First Implementation
  - Target: 375px viewport (Android-first for Carlos).
  - UI Flow: A unified "Today's Work" feed where incoming requests, drafted quotes awaiting approval, and today's schedule are presented in clean, actionable cards using the OHC premium translucent design language. Large, thumb-friendly touch targets (≥ 44x44px) are mandatory.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** OHC Agentic Field Service Scheduling & Quoting

  **Target Persona:** Carlos the Handyman

  **Outcome:** Carlos receives a lead on his phone, the Sales and Operations agents auto-draft a quote and tentatively reserve the time block. Carlos taps "Approve Quote," and the system handles the customer communication and final calendar booking invisibly.

  **Critical User Journey (CUJ):**
  1. Customer submits a "Leaky Pipe" repair request via Carlos's OHC booking link, selecting preferred times.
  2. The Sales Agent drafts a quote based on predefined "plumbing" rates and local travel estimates.
  3. The Operations Agent queries Postgres for availability and applies a 10-minute Redis Redlock on the preferred time slot.
  4. Carlos opens the OHC mobile app (375px view), sees a new "Quote Ready for Review" card in his unified feed.
  5. Carlos taps "Approve & Send". The Customer Success agent texts the customer the booking link, and upon customer confirmation, the Redis lock transitions into a finalized Postgres booking record.

  **Next Actions for Engineering:**
  - **Step 1:** Implement the Redis Redlock booking slot reservation service to handle temporary locks during quote generation.
  - **Step 2:** Refine the PostgreSQL schema (e.g., `unified_booking_resources`, `interactive_proposals`) to tightly couple quotes with tentative calendar holds.
  - **Step 3:** Extend the Operations and Sales Agents to collaboratively parse service requests, estimate costs, and execute the slot-locking logic before presenting the drafted quote to the owner.
  - **Step 4:** Ensure E2E Playwright tests cover the scenario where two customers request the same slot simultaneously, verifying the Redlock correctly prevents double-booking.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
