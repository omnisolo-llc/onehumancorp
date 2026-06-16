issue_title: "[Research] Mobile-First Client Proposal & Approval Workflow Architecture"
issue_description: |
  # Research Report: Mobile-First Client Proposal & Approval Workflow Architecture

  ## Executive Summary
  This report details an architectural design for a "Mobile-First Client Proposal & Approval Workflow" within the OneHumanCorp (OHC) platform. It directly addresses the "Nora (Agency Principal)" persona, filling a critical gap in the platform's ability to seamlessly manage project intake, draft proposals, and track client approvals exclusively from a mobile interface without complex desktop SaaS tools.

  ## 1. Persona & Business Need
  **Persona Focus:** Nora (Agency Principal, 39). She runs a small design studio with contractors and clients.
  **The Pain Point:** Currently, drafting proposals, getting client sign-offs, and triggering invoices involve jumping between Google Docs, Docusign, and an accounting tool. This workflow is highly frictional on a 375px mobile screen. Nora needs a unified, AI-assisted flow where a client request immediately turns into an approved project and initial deposit without leaving the OHC assistant interface.

  ## 2. Market Mapping & Competitor Discovery (Track 1)
  - **HoneyBook / Dubsado:** Strong in creative agency workflows but often feel bloated with too many features and complex CRM configurations that don't translate well to quick mobile actions.
  - **Bonsai:** Good mobile app for freelancers, but lacks the deep, invisible AI agent coordination (Operations + Finance + Legal) that OHC promises.
  - **Notion AI:** Excellent for drafting, but lacks native payment gateways, structured approval statuses, and multi-tenant isolation.

  **The OHC Advantage:** By utilizing the "Sales Agent" to draft the proposal and the "Operations Agent" to automatically create tasks upon approval, OHC can condense a 4-tool process into a single chat-like mobile feed action.

  ## 3. Deep Dive Architecture Design (Track 2)

  ### Data Model & Invariants
  - **`quotes` / `proposals` Table (PostgreSQL):** Extended to handle document state (`Draft`, `Pending_Approval`, `Approved`, `Rejected`), client signatures, and deposit requirements. Must enforce strict Row-Level Security (RLS) via `tenant_id`.
  - **`project_intakes` Table:** A new entity to capture unstructured client requests and map them to structured project parameters (budget, timeline, deliverables).

  ### Mermaid ER Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ QUOTE : owns
      QUOTE ||--o{ QUOTE_LINE_ITEM : contains
      QUOTE }o--|| CLIENT : sent_to
      CLIENT ||--o{ PROJECT_INTAKE : submits
      PROJECT_INTAKE ||--|| QUOTE : generates

      QUOTE {
          UUID id PK
          UUID tenant_id FK
          String status "Draft, Pending, Approved"
          DateTime expires_at
          String signature_hash
          BigInt required_deposit
      }
      PROJECT_INTAKE {
          UUID id PK
          UUID tenant_id FK
          Text raw_request
          Jsonb parsed_requirements
      }
  ```

  ### AI Department Coordination
  - **Sales & Revenue Agent:** Receives the `PROJECT_INTAKE`, drafts the `QUOTE` using historical tenant pricing and context, and presents it to Nora for 1-tap mobile approval.
  - **Knowledge & Legal Agent:** Ensures the proposal includes the correct standard terms and conditions based on the tenant's stored policies.
  - **Operations Agent:** Once the `QUOTE` is marked `Approved` by the client, this agent automatically creates the associated project tasks and assigns contractors.
  - **Finance Agent:** Automatically fires the deposit invoice (Stripe Payment Link) upon proposal approval.

  ## 4. Mobile-First UX & Technical Integrity (Track 3)
  - **Mobile Flow (375px):**
    1. Nora receives an alert in her Work Triage feed: "New Intake: Branding for ACME Corp."
    2. She taps the alert. The Sales Agent displays a pre-drafted Proposal Card.
    3. The card uses standard OHC Translucent Glass styling. Nora can tap "Edit Items" (full-screen modal with native keyboard) or "Send for Approval".
    4. Client receives a mobile-optimized web link (PWA). The client views the proposal and signs using a simple touch-signature component (minimum 44x44px touch targets).
  - **Security:** The client approval link must use a cryptographically secure, time-bound token to prevent unauthorized access. No client login is required, ensuring zero friction.

  ## 5. Implementation Prompt for Engineering Swarm

  **Feature Name:** AI-Assisted Proposal Drafting & Mobile Approval Flow

  **Target Persona:** Nora (Agency Principal)

  **User Facing Outcome:** Nora can receive a project request, have an AI agent draft a detailed proposal, review it on her phone, send it to the client, and receive approval + deposit—all within a single continuous mobile feed.

  **Critical User Journey (CUJ) to Implement & Test:**
  1. (Setup) Nora's tenant receives a new `PROJECT_INTAKE` via an API/Webhook.
  2. The Sales Agent drafts a `QUOTE` with line items and a required deposit based on the intake.
  3. Nora views the `QUOTE` in the mobile UI, taps "Send".
  4. The client opens the secure approval link on a mobile browser, views the proposal, and taps "Approve & Pay Deposit".
  5. The `QUOTE` status updates to `Approved`, and the Finance Agent generates the invoice.

  **Acceptance Criteria:**
  - Database schema handles the new state transitions securely with RLS.
  - The UI component for the Proposal Card is fully responsive down to 375px.
  - The secure client link functions without requiring a platform login.
  - An E2E Playwright test verifies the full flow from Intake to Approved.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
