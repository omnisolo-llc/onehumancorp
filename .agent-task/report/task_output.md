issue_title: "[Architecture] Universal Autonomous Client Portal & Unified Collaboration Hub"
issue_description: |
  # Title: Universal Autonomous Client Portal & Unified Collaboration Hub

  ## Problem Statement
  For professional service owners like Nora (Agency Principal) and Leo (Creator/Tutor), managing client relationships is highly fragmented. They use email for proposals, Stripe for invoices, Google Drive for assets, and text messages for quick approvals. This scatter-shot approach creates friction: Nora loses track of which client approved what design, and Leo’s students forget where their lesson links or homework are.

  Competitors like HoneyBook or Dubsado offer client portals but they are heavy, require technical setup, and feel like logging into a tax software rather than a branded, premium experience. OHC currently excels at transaction and booking, but lacks a persistent, secure, and beautiful "Home Base" for the client—a unified portal where proposals, invoices, shared documents, and agent-assisted chat coexist. We need an autonomous client portal that is instantly generated per client, requires zero setup from the owner, and serves as the single source of truth for the entire client lifecycle.

  ## Research Report
  - **Codebase & Docs Audit**: OHC’s current architecture supports bookings and quotes, but lacks a dedicated persistence layer for client-facing shared spaces (`ClientPortal`, `SharedDocument`, `ApprovalThread`). Multi-tenancy must be extended to support securely scoped external client access without full OHC user accounts.
  - **Competitor Systems Audit**:
    - *HoneyBook/Dubsado*: Powerful but high friction; requires complex workflow building.
    - *Notion*: Great for docs, terrible for native payments/invoicing.
    - *Basecamp*: Good for communication, lacks deep commerce/booking integration.
  - **Identify Gaps**: OHC needs a "Zero-Setup Client Portal". When Nora adds a client, the OHC Operations Agent should automatically spin up a secure, branded portal link. This portal aggregates their chat history, pending invoices, shared project files, and active tasks.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CLIENT : manages
      CLIENT ||--|| CLIENT_PORTAL : "accesses via magic link"
      CLIENT_PORTAL ||--o{ SHARED_DOCUMENT : contains
      CLIENT_PORTAL ||--o{ INVOICE : tracks
      CLIENT_PORTAL ||--o{ APPROVAL_THREAD : hosts
      CLIENT_PORTAL ||--o{ AGENT_INTERACTION : records

      AI_OPERATIONS_AGENT ||--o{ CLIENT_PORTAL : "auto-generates & organizes"
      AI_FINANCE_AGENT ||--o{ INVOICE : "posts & reminds"
  ```

  ### Mobile-First UX Flow (375px)
  - **Owner View (Nora)**:
    - Nora opens a client profile and taps "Share Portal".
    - The Operations Agent generates a secure magic link and drafts an SMS/Email: "Hi John, here is your project home base."
  - **Client View (John)**:
    - John taps the link on his phone. No password required (magic link / OTP).
    - He sees a beautiful, glassmorphism-styled dashboard branded with Nora’s logo.
    - **Top Card**: "Action Required: Approve Design v2" (with a simple thumbs up/down).
    - **Middle Card**: "Pending Invoice: $500" (with a 1-tap Apple/Google Pay button).
    - **Bottom Section**: "Project Files" & "Chat History".
  - **AI Agent Integration**:
    - The Customer Relationship Assistant monitors the portal. If an invoice sits unpaid for 3 days, the agent drafts a gentle nudge in the chat thread.
    - If John uploads a document, the Knowledge Assistant automatically indexes it and summarizes it for Nora’s daily feed.

  ### Zero Trust & Security Guarantees
  - **External Access Scoping**: External clients are NOT OHC system users. They access the portal via short-lived JWTs (magic links) scoped explicitly to `tenant_id` and `client_id`.
  - **Data Isolation**: All queries executed on behalf of a portal token must enforce row-level security ensuring they can only read/write to `CLIENT_PORTAL` entities tied to their specific `client_id`.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the Universal Autonomous Client Portal feature.
  1. **Data Model**: Design the database schema for `ClientPortal`, `PortalAccessSession` (for magic links), and related entities, ensuring strict multi-tenant and client-level isolation.
  2. **API Layer**: Create the external-facing REST/gRPC endpoints that allow clients to view their portal, approve items, and make payments, authenticated via magic links.
  3. **AI Department Hooks**: Wire the Customer Relationship Assistant to automatically generate a portal when a new client is onboarded and draft the welcome message.
  4. **Mobile UX**: Implement the 375px mobile-first client portal view in the frontend using the OHC Premium Design System (Translucent Glass materials, UniFi layouts). Ensure the "Action Required" and "Payments" cards are highly prominent.
  5. **Acceptance Criteria**: A real owner (via UI testing) can create a client, generate a portal link, and the simulated client can access the link, view an invoice, and approve a document without logging into the main OHC dashboard.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
