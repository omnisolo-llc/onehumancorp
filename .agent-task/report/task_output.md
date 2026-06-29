issue_title: "Agentic Quote Generator & Service Booking Flow for Field Services"
issue_description: |
  # Agentic Quote Generator & Service Booking Flow for Field Services

  ## Problem Statement
  Field service owners like Carlos (Handyman) rely heavily on word of mouth and often manage operations strictly from a mobile device (Android). They struggle with manually generating estimates, sending them back to clients in a timely manner, taking deposits, and booking time slots. The current platforms force them to juggle multiple disconnected tools (a calendar app, a PDF generator, an invoicing tool, and separate chat apps). This fragmentation leads to missed leads and delayed quotes, directly impacting their revenue and efficiency.

  ## Research Report
  - **Market Context**: Platforms like Jobber and Housecall Pro offer comprehensive solutions for field services, but they are often overly complex and have a steep learning curve for a solo operator or micro-SME. General website builders (Wix, Squarespace) offer booking capabilities but lack dynamic, integrated quoting workflows.
  - **The OHC Opportunity**: OHC can provide an integrated, conversational interface (The Assistant) that captures lead intent via chat/SMS, coordinates with the AI to generate a quote, and seamlessly handles the deposit and booking.
  - **Competitor Gaps**:
    - *Jobber / Housecall Pro*: Feature-heavy, expensive, and require significant manual configuration. They are not "assistant-first."
    - *Wix / Squarespace*: Do not natively handle dynamic quoting based on service requests without third-party apps.

  ## Design Doc
  ### Architecture & Data Model (PostgreSQL)
  ```mermaid
  erDiagram
      ServiceRequest {
          uuid id PK
          string description
          string status
          uuid customer_id FK
      }
      Quote {
          uuid id PK
          jsonb line_items
          decimal total_amount
          decimal deposit_required
          uuid service_request_id FK
      }
      Booking {
          uuid id PK
          timestamp scheduled_at
          string status
          uuid quote_id FK
          uuid resource_id FK
      }
      ServiceRequest ||--o{ Quote : generates
      Quote ||--o{ Booking : schedules
  ```
  - `ServiceRequest`: Captures the initial customer inquiry (photos, description, location).
  - `Quote`: The AI-generated or owner-drafted estimate (line items, total, deposit required) linked to the `ServiceRequest`.
  - `Booking`: The scheduled service appointment, linked to the approved `Quote` and a `Resource` (Carlos).
  - **Distributed Coordination**: The AI Job Queue manages the asynchronous processing of incoming requests, generating the quote drafts, and sending notifications to the owner.

  ### Mobile UX Flow (375px)
  1. **Owner Feed (Triage)**: Carlos sees a new card in his feed: "New Service Request from John (Leaky Faucet)".
  2. **Quote Review**: Tapping the card opens a unified view showing the customer's message/photo and an AI-drafted quote. Carlos can accept, edit line items, or regenerate.
  3. **Send & Book**: Once approved, the assistant sends a unified link to the customer via SMS/Email.
  4. **Customer View**: The customer receives a mobile-optimized page to review the quote, pay the deposit (via Stripe integration), and select an available time slot from Carlos's synchronized calendar.

  ### AI Integration Points
  - **Sales & Acquisition Agent**: Parses incoming messages/photos to understand the scope of work and drafts the initial `Quote` line items based on Carlos's historical pricing and standard service rates.
  - **Operations Agent**: Coordinates calendar availability and automatically schedules the `Booking` once the deposit is paid, resolving any conflicts.

  ## Implementation Prompt
  **Feature Name**: Agentic Quote Generator & Service Booking Flow
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos can receive a service request (e.g., via SMS or web form), review an AI-drafted quote on his Android phone, and send it to the customer. The customer can approve, pay a deposit, and book a time slot in a single unified flow.

  **Acceptance Criteria / Next Actions**:
  1. Implement the database schema for `ServiceRequest`, `Quote`, and `Booking`.
  2. Develop the mobile-first Owner Feed UI for reviewing and editing AI-drafted quotes.
  3. Create the Customer Quote & Booking UI (responsive web) with integrated Stripe deposit payments and calendar slot selection.
  4. Implement the Sales Agent capability to draft quotes from natural language service requests.
  5. Provide an end-to-end Playwright E2E test covering a customer submitting a request, the owner approving the quote, and the customer completing the deposit and booking.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []