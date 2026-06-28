issue_title: "OHC Autonomous Quoting & Proposal Generator (The Estimator Agent)"
issue_description: |
  # Research Report: Autonomous Quoting & Proposal Generator

  ## 1. Problem Statement
  Service-based small business owners—such as Carlos (Field Service Handyman) and Nora (Agency Principal)—lose potential revenue due to the friction of manually generating quotes, estimates, and proposals. Customers increasingly expect immediate, accurate estimates. Delays caused by the owner being "on the job" or overwhelmed with admin work directly lead to lost leads. Existing platforms either require manual data entry into static forms, force the owner to build complex templates, or rely on expensive, fragmented third-party CRMs.

  ## 2. Research Report
  - **Market Context**: Platforms like HoneyBook and Dubsado are robust for client management but require extensive manual setup (creating templates, defining packages). They are "tools," not "assistants." HubSpot's Breeze AI aids in drafting, but the platform is overly complex for micro-SMEs. Square provides simple invoicing but lacks proactive AI drafting based on conversational intent.
  - **The OHC Opportunity**: By integrating an "Estimator Agent" natively alongside the core operations and payment infrastructure, OHC can close the gap between Lead Intake and Deposit Payment. The agent acts as an autonomous sales associate, turning casual inquiries into actionable quotes.
  - **Competitor Gaps**:
    - *HoneyBook/Dubsado*: High setup friction; passive system relying on owner initiation.
    - *Square Invoices*: Simple, but purely manual entry. No conversational AI drafting.
    - *Shopify*: Fundamentally built for products, not dynamic service quoting.

  ## 3. Design Doc
  ### Architecture & Data Model (PostgreSQL)
  - `ServiceCatalog`: Standardized pricing units and rules (e.g., hourly rates, base fees).
  - `Lead`: The customer's raw intent or inquiry (e.g., from an Instagram DM or Web Widget).
  - `Proposal`: The generated quote, linking `Lead` and `ServiceCatalog` items. States: `draft`, `sent`, `approved`, `rejected`.
  - `DepositSession`: Integration with Stripe Payment Intents for upfront commitment.

  ### AI Agent Coordination
  - **The Estimator Agent (Sales Department)**: Intercepts inbound messages (via "The Ambassador"), extracts the scope of work using Gemini, cross-references the `ServiceCatalog`, and drafts a `Proposal`.
  - **The Manager (Operations Department)**: Cross-checks availability to ensure the proposed work can be scheduled.

  ### Mobile UX Flow (375px)
  1. **Inbound Signal**: Customer sends a DM or form: "Need my kitchen sink fixed, it's leaking."
  2. **Owner Feed (Dashboard)**: Carlos receives a push notification and sees a new card in the OHC feed: "New Request: Kitchen Sink Repair."
  3. **Agent Draft**: The Estimator Agent presents a drafted proposal within the card: "Drafted Quote: $150 base + estimated $50 parts. Requires $50 deposit."
  4. **One-Tap Action**: Carlos taps "Approve & Send". The customer receives a polished SMS/Email with a mobile-friendly Stripe deposit link.

  ### System Architecture
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Ambassador as Ambassador Agent
      participant Estimator as Estimator Agent
      participant DB as PostgreSQL
      participant Owner as Owner (Mobile UI)

      Customer->>Ambassador: "Need kitchen sink fixed"
      Ambassador->>Estimator: Route intent (Plumbing Repair)
      Estimator->>DB: Fetch ServiceCatalog (Base Rates)
      Estimator->>DB: Draft Proposal & Deposit Link
      Estimator->>Owner: Push Notification (Action Required)
      Owner->>Owner: Review Drafted Proposal
      Owner->>DB: Tap "Approve & Send"
      DB->>Customer: Send SMS with Quote & Payment Link
  ```

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Quoting & Proposal Generator (The Estimator Agent)
  **Target Persona**: Carlos the Handyman, Nora the Agency Principal
  **Outcome**: An AI-driven quoting engine that intercepts customer requests, matches them against the owner's price list, drafts a proposal with a deposit link, and presents it for 1-tap owner approval.

  **Critical User Journey (CUJ)**:
  1. Carlos is working on-site when a customer submits a web request.
  2. The Estimator Agent parses the request, checks Carlos's `ServiceCatalog`, and drafts a $200 proposal.
  3. Carlos checks his OHC mobile app (375px view) and sees the drafted proposal in his "Action Required" feed.
  4. Carlos taps "Approve".
  5. The customer receives the quote, approves it, and pays the deposit via Stripe.
  6. The system automatically creates an operational task for the job.

  **Next Actions for Engineering Swarm**:
  1. **Data Model**: Implement the `Proposal` and `ServiceCatalog` entities with strict multi-tenant isolation.
  2. **Agent Capability**: Extend the Sales Agent capabilities to parse natural language scopes of work and generate structured quotes.
  3. **UI Implementation**: Build the mobile-first "Proposal Review" card component for the unified work feed, ensuring touch targets are at least 44x44px.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
