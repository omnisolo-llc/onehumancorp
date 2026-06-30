issue_title: "Research Report: OHC Field Service AI Operations (Carlos Persona)"
issue_description: |
  # Research Report: Field Service AI Operations

  ## 1. Problem Statement
  Field service owners like Carlos (Handyman) operate entirely from their phones, often without a formal website. Existing solutions fall into two extremes: complex scheduling/dispatch software designed for multi-truck fleets (Housecall Pro, Jobber) which are too expensive and complicated, or basic calendar apps that lack invoicing, quoting, and customer memory. Carlos needs a unified, mobile-first assistant that can turn a word-of-mouth lead into a quote, a scheduled visit, a routed map, and a collected deposit, all driven by AI agents.

  ## 2. Research Report
  - **Market Context**: The home services software market is heavily geared toward established businesses. Micro-businesses and solopreneurs rely on SMS, Venmo, and mental notes.
  - **Competitor Gaps**:
    - *Jobber/Housecall Pro*: Powerful but complex. Requires heavy initial data entry and configuration. Not truly "agentic"—they are traditional CRUD apps.
    - *Square Appointments*: Good for basic bookings, but poor for mobile field operations (like quoting based on photos).
  - **The OHC Opportunity**: Integrate a lightweight CRM, scheduling, quoting, and payments into a single mobile feed where the *Operations Agent* coordinates the workflow. A customer texts a photo of a broken pipe; the agent drafts an estimate, Carlos approves it on his 375px screen, the agent sends the link and collects the deposit, and schedules the route.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `ServiceRequest`: Captures the initial inbound lead (via SMS, WhatsApp, Web). Includes media attachments.
  - `Estimate`: Linked to a ServiceRequest. Includes line items, deposit requirements.
  - `FieldJob`: The scheduled work. Includes time, location (lat/lon), status (dispatched, en-route, completed).
  - `CustomerMemory`: Tracks past jobs, property details (e.g., "gate code is 1234").

  ### AI Integration
  - **Operations Agent (The Dispatcher)**: Monitors inbound `ServiceRequests`. Extracts intent, urgency, and location. Proposes available time slots based on Carlos's existing route (minimizing drive time).
  - **Sales Agent (The Estimator)**: Uses Gemini Vision to analyze photos submitted by clients to draft initial `Estimates`.

  ### Mobile UX Flow (375px)
  1. **Inbound Alert**: Carlos receives a notification card: "New Lead: Broken Pipe in Downtown. [View Photo & Draft Estimate]".
  2. **Estimate Approval**: The Sales Agent has pre-filled a $150 estimate. Carlos taps "Approve & Send".
  3. **Daily Route View**: A clean vertical feed showing today's jobs, ordered by optimal route, with 1-tap navigation and 1-tap "Collect Final Payment" buttons.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Field Service Operations Module
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos can manage a full job lifecycle—from photo-based lead to final payment—entirely through AI-assisted cards on a mobile device, without needing a desktop CRM.

  **Next Actions**:
  1. Implement the core Data Models (`ServiceRequest`, `Estimate`, `FieldJob`) with strict multi-tenant isolation.
  2. Develop the Vision AI integration for the Sales Agent to generate draft `Estimates` from uploaded images.
  3. Create the mobile-first Daily Route View UI, integrating the Operations Agent to suggest optimal scheduling based on location.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
