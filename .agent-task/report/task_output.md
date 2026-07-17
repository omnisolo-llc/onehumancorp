issue_title: "Implement Autonomous Professional Services Proposal, Milestone SOW Contract, and Smart Escrow Engine"
issue_description: |
  # 🔬 RESEARCH REPORT: Autonomous Professional Services Proposal, Milestone SOW Contract, and Smart Escrow Engine

  ## 1. Title
  Autonomous Professional Services Proposal, Milestone SOW Contract, and Smart Escrow Engine

  ## 2. Problem Statement
  For professional service agency principals like Nora, managing client projects, contractors, and cash flow is a constant administrative struggle. Currently, if Nora takes on a new design project, she must manually onboard the client, draft a custom proposal and Statement of Work (SOW), send it via third-party digital signature tools (e.g., DocuSign), and manually follow up on deposit invoices. Once signed, she must coordinate contractors, map deliverables to milestones, manually monitor task completion, and chase down clients for invoice payments at each stage. This highly manual, fragmented desktop-centric process causes delayed payments, project scope creep, and administrative fatigue. Nora needs an invisible, mobile-first system where AI agents autonomously manage the entire lifecycle: generating client-facing proposals, executing milestone-based contracts, coordinating contractors, and automating escrow-secured payments.

  ## 3. Research Report & Competitor Analysis
  **Competitor Landscape & Market Gaps:**
  - **HoneyBook / Bonsai / Dubsado:** These are solid business management suites for independent professionals. However, they are desktop-first, require merchants to manually create and maintain complex proposal/invoice templates, and lack any autonomous capability to draft customized proposals from client intake responses. Crucially, they do not facilitate multi-party contractor tracking, leaving agency owners to manually sync work completion with invoice releases.
  - **Upwork Enterprise:** Offers escrow-secured milestones and contractor management, but operates as a closed-marketplace monolith. It isolates the agency, clients, and contractors within Upwork's platform, charging high transaction fees (5%-20%) and separating workflow execution from the agency's custom brand, domain, and unified OHC command center.
  - **Shopify / Squarespace:** Geared strictly towards traditional physical and digital eCommerce. They have no native support for multi-party escrow, client proposals, or milestone-based service contracts, forcing service providers to stitch together expensive, loose integrations.

  **The OHC Advantage:**
  By introducing the Autonomous Agency Engine, OHC delivers an end-to-end, mobile-first (375px) workflow. OHC's unique position—controlling the client storefront, the multi-tenant database ledger, and the native AI agent departments—allows it to unify contract execution, task tracking, and milestone escrow seamlessly. The AI agents act as Nora's Virtual COO:
  1. **Intake to SOW:** The Operations Agent parses client project briefs to draft a structured, legally compliant Statement of Work (SOW) with milestones.
  2. **Escrow Safeguarding:** Client payments are held securely in milestone-based escrow accounts using a multi-tenant database ledger.
  3. **Contractor Coordination:** Task assignments are monitored automatically; when a contractor submits a deliverable, the AI validates it and drafts the milestone release approval for Nora's 1-tap confirmation.

  ---

  ## 4. Design Doc & Architecture

  ### 4.1 System Architecture Diagram
  The system leverages OHC's multi-tenant Go backend, PostgreSQL ledger, and Redis-driven AI Agent Swarm.

  ```mermaid
  graph TD;
      subgraph Client & Merchant Mobile View (375px Viewport)
          App[OHC Mobile PWA] -->|1. Submit Brief / Approve Draft| API[OHC Gateway API];
          App -->|2. E-Sign & Fund Escrow| EscrowService[Stripe/Escrow Service];
      end

      subgraph OHC Multi-Tenant Core
          API -->|Tenant Isolation RLS| SOWService[SOW & Proposal Service];
          API -->|Secure Transaction Log| LedgerService[Postgres Multi-Party Ledger];

          SOWService --> DB[(Postgres Cloud Database)];
          LedgerService --> DB;
      end

      subgraph OHC AI Agent Swarm
          SOWService <-->|Trigger Agents| Mesh[Teammate Mesh / Redis];
          Mesh --> OpsAgent[Operations Agent: Task Alignment];
          Mesh --> SalesAgent[Sales Agent: SOW & Proposal Generation];
          Mesh --> FinanceAgent[Finance Agent: Escrow Nudge Sequences];
          Mesh --> ProjectAgent[Project Advisory Agent: Daily Work Summaries];
      end
  ```

  ### 4.2 Data Model & Invariants
  ```mermaid
  erDiagram
      TENANT ||--o{ AGENCY_PROJECT : operates
      AGENCY_PROJECT ||--|| PROPOSAL : governs
      PROPOSAL ||--|{ MILESTONE : "comprises"
      MILESTONE ||--|| CONTRACTOR_ASSIGNMENT : delegates
      MILESTONE ||--|| ESCROW_LEDGER : secures

      PROPOSAL {
          string id PK
          string tenant_id FK
          string status "draft, sent, signed, active, completed"
          string client_email
          jsonb metadata "SOW details, line items, deliverables"
      }

      MILESTONE {
          string id PK
          string proposal_id FK
          string title
          decimal amount
          string status "pending_funding, funded, in_progress, submitted, released, disputed"
          timestamp due_date
      }

      CONTRACTOR_ASSIGNMENT {
          string id PK
          string milestone_id FK
          string contractor_id
          string status "assigned, working, completed, approved"
          decimal payout_amount
      }

      ESCROW_LEDGER {
          string id PK
          string milestone_id FK
          string transaction_hash
          string holding_status "idle, held, released, refunded"
      }
  ```

  *Data Isolation & Zero-Trust Invariant:*
  Every database query and service API interaction strictly isolates state using the `tenant_id` claims retrieved from the authenticated OIDC token, validated through SPIFFE/SPIRE workload identities internally. Direct database writes or query overrides are strictly forbidden to enforce row-level safety across all organizations.

  ### 4.3 AI Agent Department Coordination
  The background automation utilizes four specific AI agent departments collaborating via the Teammate Mesh:
  1. **Sales & Client Relationship Agent:**
     - *Action:* Automatically parses raw customer intake questionnaires, transcripts, or email threads.
     - *Output:* Autonomously drafts a complete, customized Statement of Work (SOW) proposal including precise deliverables, milestone descriptions, and pricing.
  2. **Operations & Coordination Agent:**
     - *Action:* Extracts contractor capabilities and monitors delivery status.
     - *Output:* Translates the approved SOW milestones into individual tasks, assigns them to Nora's contractors, and checks daily task completion state against deadlines.
  3. **Finance & Invoicing Agent:**
     - *Action:* Listens to payment triggers.
     - *Output:* Generates escrow invoice requests, verifies when the client funds a milestone, and schedules automated email/SMS nudge sequences for delayed funding or outstanding approvals.
  4. **Project Advisory Agent (Virtual COO):**
     - *Action:* Aggregates multi-channel milestones and contractor progression metrics daily.
     - *Output:* Compiles plain-language, jargon-free briefings for Nora (e.g., "Contractor Leo completed the logo draft. Tap here to approve and release $500 from Escrow.").

  ---

  ## 5. Mobile UX Flow (375px Viewport first)

  To ensure the engine is intuitive and accessible to non-technical users, all screens utilize translucent glass materials and follow mobile-first touch metrics (>= 44x44px).

  ### 5.1 Nora's Daily priority Dashboard (Merchant View)
  - **Visual Design:** A high-end macOS-style blurred background. A central card with dynamic micro-interactions.
  - **Top Section:** "Today's Priorities" header with an alert: 🔴 *Loose End Detected*.
  - **Priority Card:** "Client Sarah has signed the Branding Project SOW. Milestone 1 ($2,500) requires funding before Contractor Alex can begin."
  - **Primary CTA:** A large, easily tappable button `[Draft Funding Request]` (48px height, native focus states, translucent glass border).
  - **Secondary Actions:** `[View SOW Contract]` and `[Remind Client]`.

  ### 5.2 Dynamic Client Checkout & Sign (Client View)
  - **Visual Design:** Clean, simple, iOS Safari optimized layout.
  - **Step 1: Sign SOW:** Displays the AI-generated SOW in a readable, simplified card. Client signs with their finger in an interactive canvas block.
  - **Step 2: Fund Milestone:** Tap-to-pay block with Stripe integration.
  - **UI Feedback:** Translucent toast notification: "Contract Signed & Milestone 1 ($2,500) Funded in Escrow."

  ### 5.3 Milestone Release Panel (Merchant View)
  - **Visual Design:** Unified operational card view.
  - **Context:** Contractor Alex uploads the deliverable `brand_assets.zip`. The AI scans the file, runs a basic integrity check, and updates the OHC dashboard.
  - **Review Card:** Nora sees the review screen on her phone: "Contractor Alex submitted Milestone 1 assets. Review and approve release of $1,500."
  - **Interactive Trigger:** Slider control `Swipe to Approve & Release Funds` to prevent accidental tapping.
  - **Post-Action State:** Screen dynamically updates with a subtle haptic-feeling transition showing: "Funds Released. Milestone 1 Complete. Milestone 2 (Web Development) is now Active."

  ---

  ## 6. Implementation Prompt
  **Task Objective:** Design and implement the backend database schema, API services, and mobile-first frontend interfaces for the Autonomous Professional Services Proposal, SOW, and Milestone Escrow Engine.

  **User Journey (CUJ):**
  1. Nora (Merchant) logs into her mobile dashboard and requests the AI Sales Agent to generate a project proposal based on a customer brief.
  2. The AI drafts the proposal SOW with three milestones and sends it to the client.
  3. The client reviews, signs, and funds Milestone 1 via a unified mobile checkout screen.
  4. Nora receives a dashboard notification that the milestone is active and contractor tasks are auto-generated.
  5. Once a contractor completes the milestone, Nora reviews the submission on her phone and swipes to release the milestone payment from the escrow ledger.

  **Backend Acceptance Criteria:**
  - Create the PostgreSQL database schemas for `proposals`, `milestones`, `contractor_assignments`, and `escrow_ledger` tables, securing them with Row-Level Security (`tenant_id`).
  - Implement the API controllers in Go/Rust for proposal creation, SOW e-signing, milestone funding, and escrow payout execution.
  - Implement transactional validation: releasing funds from escrow must fail if the corresponding milestone status is not `funded` or if the signature event is missing.
  - Ensure all endpoints validate workload identity via SPIFFE/SPIRE.

  **Frontend & Mobile Acceptance Criteria:**
  - Build the 375px mobile-first responsive screens for the Merchant Priority Dashboard, Client SOW Signing & Checkout page, and Merchant Milestone Release Panel.
  - Apply macOS-style Translucent Glass material cards (`backdrop-filter: blur(20px) saturate(200%);`) with precise dark/light borders and clean spacing tokens.
  - All touch targets (buttons, input fields, checkboxes) must be at least 44x44px.
  - The Milestone Release Panel must use an interactive swipe slider or double-confirmation pattern.
  - Ensure ZERO hardcoded mockup records exist; the dashboard must load live empty states, loading indicators, or actual transactional data.

  ## 7. Priority & Scope
  - **Priority:** P1 (High)
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
