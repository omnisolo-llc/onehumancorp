issue_title: "Agentic Field Service Dispatch & Mobile Estimating Engine"
issue_description: |
  ## Title
  Agentic Field Service Dispatch & Mobile Estimating Engine

  ## Problem Statement
  Carlos (the field service owner/handyman) relies exclusively on his Android phone. Existing platforms are desktop-first; mobile apps are just analytics dashboards, lacking robust capabilities for routing, multi-job capacity planning, and zero-touch dispatch. Carlos spends countless hours every week mapping out routes, recalculating arrival times manually when a job goes long, and preparing simple quotes using disconnected tools. He needs a cohesive, agent-driven assistant that optimally calculates routes offline, dispatches estimates with one tap, and manages his schedule efficiently.

  ## Research Report
  - **Market Context**: Platforms like Shopify or Wix are heavily oriented toward physical products or static portfolios. Although they support bookings via apps, these apps do not understand dynamic travel times or routing.
  - **Competitors**: Field service platforms like ServiceTitan or Housecall Pro are powerful but extremely complex and expensive, alienating micro-operators like Carlos.
  - **Gap**: There is a missing "middle layer" — a system that is simple enough for a non-technical solopreneur but smart enough to automate logistics and estimation natively on a mobile device.
  - **Proposed Paradigm Shift**: OHC's "Agentic Departments" will fill this gap. A dedicated Operations Agent will handle travel time pads between jobs natively, caching the route offline on Carlos's device to ensure he never loses his itinerary in poor connectivity zones. Concurrently, a Salesperson Agent will parse free-text customer requests to generate a 1-tap quote.

  ## Design Doc
  ### 1. Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Service Request via OHC Inbox] --> B{Intent Classifier}

      B -->|Quote Request| C[Salesperson Agent]
      C --> D[1-Tap Estimate Draft on Mobile]
      D --> E[Customer SMS/WhatsApp Quote Link]

      B -->|Booking/Job Scheduled| F[Operations Agent]
      F --> G[Travel Time Calculation]
      G --> H[Calendar Block with Padding]
      H --> I[Offline Sync to Mobile Edge Device]

      I --> J[Carlos's Mobile 'Today's Route' Card]
  ```

  ### 2. Mobile UX Flow (375px First)
  - **The "Today's Route" Interface**: A single macOS Translucent Glass card displaying upcoming stops sequentially. Uses Apple macOS/UniFi curves (16px container radius, 8px button radius).
  - **Interaction**: Features a large, thumb-friendly "Start Next Job" and "Complete Job" button (min 44x44px touch targets).
  - **Offline Indicator**: A small "Offline Ready" green badge ensures confidence when Carlos descends into a signal-less basement.
  - **1-Tap Estimate**: From the inbox, an AI-drafted quote card shows estimated labor + materials. A single "Approve & Send" button transmits the quote to the customer via SMS.

  ### 3. AI Agent Integration Points
  - **Salesperson Agent**: Synthesizes customer intents into formal, localized estimates based on previously completed jobs and standard hourly rates.
  - **Operations Agent**: Interfaces with mapping APIs to calculate travel distance, inserting padding into the scheduling calendar to prevent overlapping or physically impossible commitments.

  ### 4. Key Design Decisions
  - **Offline-First Resilience**: Mobile clients will use an edge-cached data layer (e.g., SQLite via Tauri) so Carlos's dispatch itinerary remains intact without network coverage.
  - **Unified Calendar Invariant**: Travel time between geographical points is treated as "busy" time, preventing double-booking natively.
  - **Zero-Touch Execution**: The user is presented with simple cards for approval. They do not navigate complex multi-tab forms.

  ## Implementation Prompt
  **For Implementer Agent**:
  Implement the foundation for the `FieldServiceDispatchEngine`.
  - Define data entities for `Estimate`, `Job`, `ServiceStop`, and `RouteItinerary` in PostgreSQL, ensuring row-level tenant isolation.
  - Create the API endpoints (gRPC/REST) for the Operations Agent to retrieve and update daily itineraries, returning payloads optimized for mobile offline caching.
  - Construct the backend logic that calculates and inserts travel-time blocks into the calendar upon job scheduling.
  - Build the Mobile Flutter/Tauri UI for the "Today's Route" card following the 375px width, 44px minimum touch target, and macOS Translucent Glass (`border-radius: 16px`) design tokens.
  - Include comprehensive unit and Playwright E2E tests validating the end-to-end booking, travel time padding, and mobile UI presentation (even simulating offline behavior where possible).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
