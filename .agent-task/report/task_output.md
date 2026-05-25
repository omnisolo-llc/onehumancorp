issue_title: "[Architecture] Autonomous Hardware & Equipment Leasing Engine"
issue_description: |
  # Research Report
  ## Problem Statement
  For physical service-based personas like **Carlos (handyman)**, managing equipment rentals, leases, and hardware tracking is a massive pain point. Currently, they either buy expensive equipment outright or manage messy, paper-based rental agreements with local hardware stores. There is no unified system to lease, track, and expense hardware directly within their primary business operating system.

  ## Competitive Analysis
  - **Shopify/Wix:** Completely lack native hardware leasing or equipment tracking.
  - **Home Depot/Sunbelt Rentals:** Separate silos. Carlos has to leave his business OS, negotiate a rental, and manually reconcile the expense.
  - **The OHC Differentiator:** An integrated, agentic leasing engine. The OHC Finance & Operations agents automatically identify when Carlos books a job requiring specialized equipment (e.g., a commercial power washer), source the rental locally, secure the deposit via his OHC ledger, and track the expense against the specific job's profitability.

  ## Design Doc
  ### High-Level Architecture
  ```mermaid
  graph TD;
      Booking[Customer Books Job] --> KAIROS[KAIROS Hub];
      KAIROS -->|Analyzes Job Type| OpsAgent[AI Operations Agent];
      OpsAgent -->|Identifies Equipment Need| Sourcing[Hardware Sourcing API];
      Sourcing -->|Finds Local Rental| FinanceAgent[AI Finance Agent];
      FinanceAgent -->|Secures Deposit/Lease| Ledger[(OHC Unified Ledger)];
      FinanceAgent -->|Approves Lease| App[OHC Mobile App: 1-Tap Approve];
  ```

  ### Core Capabilities
  - **Predictive Sourcing:** Agent scans upcoming booked services and suggests equipment rentals based on job requirements.
  - **1-Tap Leasing:** Carlos sees a notification: "You need a trench digger for Friday's job. Rent from local supplier X for $150/day?" -> Taps 'Approve'.
  - **Automated Expensing:** The rental cost is automatically deducted from the job's final payout, ensuring accurate profit margins.
  - **Mobile-First UX:** 375px native cards displaying equipment, pickup locations, and return deadlines.

  ## Implementation Prompt
  **Objective:** Design the Autonomous Hardware Leasing Engine.
  **CUJ:** Carlos books a "Concrete Pouring" job. The Operations Agent triggers an equipment review, identifies a cement mixer is needed, and presents a 1-tap leasing option in his OHC feed. Upon approval, the Finance Agent processes the transaction and links the expense to the specific booking.
  **Acceptance Criteria:** Must integrate with the existing unified ledger for zero-touch expensing and provide clear, high-contrast UI for equipment pickup/return workflows on mobile.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
