issue_title: "Implement Offline-First AI Field Service Routing & Invoicing"
issue_description: |
  ## 1. Problem Statement
  Service-based owners like Carlos the Handyman spend a significant part of their day on the road with intermittent cell service. Existing platforms (Shopify, Wix) are built for stationary retail or e-commerce and completely fail in offline field-service scenarios. Carlos needs a system that intelligently routes his day, allows him to view job details offline, generates quotes on-site, and accepts tap-to-pay when he's back online or using offline card processing, all managed by an AI assistant that minimizes his screen time.

  ## 2. Research Report
  - **Market Context**: Legacy field service apps (ServiceTitan, Jobber) are incredibly powerful but overly complex and expensive ($150+/mo) for a solo operator. They require extensive manual data entry. Consumer e-commerce platforms offer no route planning or offline-first job management.
  - **The OHC Opportunity**: By leveraging local-first architecture (SQLite/PowerSync) on the mobile client (Tauri/Flutter) and coordinating with the Operations Agent, OHC can offer a Zero-Friction field service experience. The agent pre-caches Carlos's daily route and job context before he loses signal.
  - **Competitor Gaps**:
    - *Jobber*: Too much manual data entry, lacks proactive AI.
    - *Shopify/Wix*: No concept of geographic routing, offline job execution, or service time windows.
    - *Square*: Good for payments, but weak on proactive route and job state management.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Offline First] -->|PowerSync / SQLite| B(Local Job & Customer Data)
      A -.->|Network Restored| C[OHC Cloud Backend - Go/Rust]
      C --> D[(PostgreSQL Central Ledger)]
      C --> E[Operations Agent - Route Optimizer]
      C --> F[Stripe API - Invoicing & Tap-to-Pay]
      E -.->|Pre-caches next day's jobs| A
  ```

  ### Mobile UX Flow (375px First)
  1. **Morning Briefing (Online)**: Carlos opens the app. The Operations Agent presents a clean, touch-friendly single-column list of today's jobs, optimized for driving distance.
  2. **Job Execution (Offline)**: Carlos taps a job. Large buttons (44x44px min) allow him to view customer notes, add photos of the repair, and adjust the final price.
  3. **Payment & Completion (Offline/Online)**: Carlos taps "Complete & Pay". If offline, the invoice is queued locally. If online, he can immediately use Stripe Terminal Tap-to-Pay. The UI handles the transition seamlessly.
  4. **Agent Action**: Once synced, the Customer Success Agent automatically emails the receipt and a request for a review.

  ### AI Agent Integration Points
  - **Operations Agent**: Analyzes upcoming bookings, calculates optimal geographic routing, and ensures the mobile client pre-caches the required data.
  - **Customer Success Agent**: Drafts follow-up messages and review requests automatically upon job completion sync.

  ### Key Design Decisions
  - **Local-First Data**: Essential for field workers. All daily job data must be fully available offline via local SQLite.
  - **Agentic Pre-computation**: The cloud backend computes the route and required context the night before, minimizing expensive on-the-fly calculations on the mobile device.

  ## 4. Implementation Prompt
  **Feature Name**: Offline-First Field Service & Routing
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos can view his daily optimized route, execute jobs, take notes, and queue invoices entirely offline. When connection is restored, the system syncs, processes payments, and triggers AI-driven customer follow-ups.

  **Acceptance Criteria**:
  1. Define the data model for `ServiceJob`, `RouteNode`, and `OfflineQueue` with strict tenant isolation.
  2. Implement the local-first caching mechanism ensuring today's jobs are available without network.
  3. Build the 375px mobile UI for the Daily Route and Job Execution screen with large touch targets.
  4. Integrate the Operations Agent to generate the daily route based on geographic data of bookings.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []