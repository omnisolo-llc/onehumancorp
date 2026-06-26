issue_title: "OHC Autonomous Project Intake & Proposal Generation Engine"
issue_description: |
  # Research Report: Autonomous Project Intake & Proposal Generation Engine

  ## 1. Problem Statement
  Service-based independent professionals and small agency principals (e.g., Nora the Agency Principal) spend a disproportionate amount of time on the administrative overhead of client intake. The process of gathering project requirements, estimating effort, drafting proposals, and tracking client approvals is highly manual and fragmented across emails, Google Docs, and disparate CRM tools. This friction delays revenue realization and creates loose ends. Existing tools like HoneyBook or Dubsado offer workflow automation, but require complex manual setup and do not leverage AI to *actively* draft the proposal based on context.

  ## 2. Research Report
  - **Market Context**: Platforms like Notion AI and Microsoft Copilot assist in document generation but are not deeply integrated into a business's operational flow (intake -> proposal -> task creation -> invoicing). CRM tools like HubSpot or specialized agency tools (HoneyBook, Dubsado) provide templates and pipelines, but still rely on the user to write the content and move the deal forward.
  - **The OHC Opportunity**: OHC can uniquely combine Work Intake (forms/emails), Sales Assistant capabilities (proposal drafting), and Operations (task assignment) into a seamless, agent-driven flow. By transforming an inbound request directly into an AI-drafted proposal and subsequent project tasks, OHC eliminates the "blank page" problem for owners.
  - **Competitor Gaps**:
    - *HoneyBook/Dubsado*: Powerful, but require extensive manual template building and setup. No proactive, context-aware AI drafting.
    - *Notion AI*: General-purpose text generation; disconnected from invoicing and task execution.
    - *HubSpot*: Too complex and enterprise-focused for a small agency principal.

  ## 3. Design Doc

  ### Data Model (PostgreSQL)
  - `ProjectIntake`: Represents an inbound request (source, raw content, client info).
  - `Proposal`: The structured document drafted by the AI, linked to a Customer and Intake, with state (draft, sent, approved, rejected).
  - `ProposalItem`: Line items within a proposal (service description, estimated cost).
  - `ProjectTask`: Actionable items created upon proposal approval.

  ### AI Integration
  - **Work Triage / Intake Agent**: Monitors designated intake channels (email, web forms), extracts key requirements, and creates a `ProjectIntake` record.
  - **Sales & Revenue Assistant**: Triggered by a new `ProjectIntake`. Uses RAG against the agency's past proposals, pricing guidelines, and service catalog to draft a `Proposal`.
  - **Operations Assistant**: Upon client approval of the proposal, automatically generates `ProjectTask` records and assigns them to available staff/contractors.
  - **Finance Assistant**: Schedules invoice reminders based on the approved proposal's payment terms.

  ### Mobile UX Flow (375px)
  1. **Owner Feed (Triage)**: Nora opens the OHC app. The Assistant Shell highlights a new project request: "New Intake from Acme Corp. Proposal drafted."
  2. **Proposal Review**: Nora taps the notification. She sees the AI-drafted proposal in a clean, scrollable card layout (macOS Translucent Glass style). Line items, costs, and timeline are easily editable with large touch targets.
  3. **Approve & Send**: A prominent "Send for Approval" button dispatches the proposal to the client via email/SMS.
  4. **Post-Approval Action**: Once the client approves (via a web link), Nora receives a push notification, and the dashboard updates to show the newly created project tasks and scheduled initial invoice.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Project Intake & Proposal Generation
  **Target Persona**: Nora the Agency Principal
  **Outcome**: Nora receives an inbound client request. The OHC system automatically captures the request, drafts a detailed project proposal (with pricing and timeline), and presents it to Nora for a one-tap review and send. Upon client approval, the system generates project tasks and schedules the first invoice.

  **Next Actions for Engineering**:
  1. Implement the Data Models (`ProjectIntake`, `Proposal`, `ProposalItem`) with strict row-level multi-tenant isolation.
  2. Develop the AI Sales Assistant prompt chain to ingest intake data, retrieve pricing context (RAG), and generate structured proposal content.
  3. Create the Mobile-First Proposal Review UI, allowing easy editing of AI drafts and one-tap sending to clients.
  4. Implement the state transition logic: upon client approval, trigger the Operations Agent to generate tasks and the Finance Agent to schedule invoices.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []