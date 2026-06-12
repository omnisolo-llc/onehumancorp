issue_title: "Agentic Client Proposal & Automated Invoice Pipeline"
issue_description: |
  ## Title: Agentic Client Proposal & Automated Invoice Pipeline

  ### Problem Statement
  Nora, a small design agency principal, spends 30% of her week translating client intake emails into structured proposals, setting up project milestones, and chasing down overdue invoice payments. Existing tools like HelloSign, QuickBooks, and Asana are disconnected. Nora needs an intelligent work assistant that reads a client brief, drafts a professional proposal with milestones, and, upon client approval, automatically structures the work queue and schedules invoice reminders—all without requiring her to manually sync data between platforms.

  ### Research Report
  - **Competitor Systems Audit**:
    - **HoneyBook / Dubsado**: Excellent for freelancers, combining proposals, contracts, and invoicing. However, they lack strong AI-driven autonomous drafting and passive task generation. They require extensive manual templating.
    - **Notion AI**: Great for drafting text but disconnected from the financial ledger and payment gateways (Stripe).
    - **Shopify / E-commerce Platforms**: Geared towards physical/digital products, poor fit for B2B milestone-based agency services.
  - **OHC Gap**: OHC currently lacks a unified B2B project lifecycle capability. We need a fluid pipeline that connects "Work Intake" -> "Sales & Revenue" -> "Scheduling & Operations" seamlessly for service-based project work.

  ### Design Doc
  **Key Design Decisions**:
  - Treat a Proposal not just as a document, but as a state machine (`Draft`, `Sent`, `Approved`, `Rejected`).
  - When a Proposal transitions to `Approved`, the system should automatically generate a `Project` with associated `Tasks` and scheduled `Invoices`.
  - The UI must be fully functional on mobile (375px), allowing Nora to review and send a proposal while away from her desk.

  **Architecture Diagram**:
  ```mermaid
  erDiagram
      TENANT ||--o{ CLIENT : manages
      CLIENT ||--o{ PROPOSAL : receives
      PROPOSAL ||--o{ MILESTONE : contains
      PROPOSAL ||--o{ INVOICE : generates
      PROPOSAL ||--o{ PROJECT : converts_to
      PROJECT ||--o{ TASK : contains

      class PROPOSAL {
          string status
          float total_amount
      }
  ```

  **Mobile UX Flow (375px)**:
  1. **Intake Notification**: Nora receives an Action Card on her OHC Feed: "New project inquiry from Acme Corp. Draft proposal?"
  2. **Review Draft**: Nora taps the card. She sees a clean, glassmorphic 375px view of the generated proposal, including scope, milestones, and pricing (derived from her past similar projects).
  3. **Edit & Send**: She taps a milestone to quickly adjust the price using the native mobile keyboard. She taps "Approve & Send".
  4. **Approval Event**: Once the client approves via email link, Nora gets a push notification: "Acme Corp approved. 3 project tasks created, deposit invoice scheduled."

  **AI Agent Integration Points**:
  - **Sales Assistant**: Ingests unstructured client emails/forms, searches memory for similar past projects to estimate pricing, and drafts the proposal document.
  - **Operations Assistant**: Subscribes to the `proposal.approved` event. It parses the proposal milestones and generates actionable tasks assigned to Nora's contractors.
  - **Finance Assistant**: Subscribes to the `proposal.approved` event to generate the initial deposit Stripe Payment Link and schedules follow-up invoice reminders.

  ### Implementation Prompt
  **Feature Name:** Agentic Client Proposal & Automated Invoice Pipeline
  **Target Persona:** Nora the Agency Principal

  **Outcome:** An end-to-end B2B project intake flow. The system ingests a project inquiry, the Sales Agent drafts a proposal, and upon Nora's approval and the client's signature, the Operations and Finance Agents automatically create the project tasks and schedule the deposit invoice.

  **Critical User Journey (CUJ):**
  1. Log into the OHC mobile web app (375px view).
  2. Navigate to the new "Proposals" tab and tap "New from Inquiry".
  3. Paste a sample client email: "We need a new branding package, logo and brand guidelines. Budget is around $5k."
  4. The system (via AI) instantly drafts a proposal with 2 milestones (Logo Design, Brand Guidelines) totaling $5,000.
  5. Review the proposal on the mobile view (ensuring no horizontal scrolling, clear touch targets > 44px).
  6. Tap "Send to Client".
  7. Simulate client approval. Verify that a Project is automatically created with tasks for the milestones, and a draft deposit invoice is generated in the ledger.

  **Acceptance Criteria:**
  - The Proposal, Project, and Milestone entities must be implemented in the database with strict multi-tenant isolation (`tenant_id`).
  - Ensure visually premium, macOS Translucent Glass styling on the mobile proposal review screen.
  - Implement automated E2E Playwright tests verifying the flow from unstructured text to sent proposal to created project.
  - No direct implementation of the LLM logic is prescribed; use the established internal agent provider framework.

  ### Priority: P1
  ### Estimated Scope: Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
