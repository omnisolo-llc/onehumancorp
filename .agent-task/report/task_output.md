issue_title: "Agentic Proposal & Invoice Generator for Service Agencies"
issue_description: |
  **Title**: Agentic Proposal & Invoice Generator for Service Agencies

  **Problem Statement**:
  Nora, a small design studio agency principal, struggles with the administrative overhead of client intake, proposal drafting, and invoice follow-ups. Traditional tools like HubSpot or specialized agency software are overly complex, requiring manual setup of pipelines and templates. Nora needs an AI work assistant that captures client requests, autonomously drafts personalized proposals based on past successful projects, coordinates client approval, and automatically tracks and follows up on invoice payments.

  **Research Report**:
  - **Market Context**: Platforms like Shopify are heavily product-focused and unsuitable for service agencies. CRM tools like HubSpot or Monday.com require heavy manual configuration and act as passive databases. Specialized tools like HoneyBook or Bonsai offer good proposal flows but lack deep AI agency that can draft based on contextual memory and actively manage the pipeline.
  - **The OHC Gap**: OHC currently lacks a unified proposal-to-invoice pipeline tailored for service-based businesses like agencies. Without this, users like Nora cannot easily turn a client inquiry into a structured proposal, project task list, and automated payment request within a single, agent-managed flow.
  - **Competitor Analysis**: HoneyBook does well at tying proposals to invoices, but the user must still create the proposal manually. Wix provides basic invoicing but lacks proposal generation and workflow automation. OHC can differentiate by using its Sales and Operations Agents to draft proposals from brief client DMs or emails, utilizing the Knowledge Assistant to recall past project pricing.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    sequenceDiagram
        participant Client
        participant Nora (Owner)
        participant WorkTriageAgent
        participant SalesAgent
        participant Ledger (DB)

        Client->>WorkTriageAgent: Inquiry (Email/Form)
        WorkTriageAgent->>SalesAgent: Trigger Proposal Draft
        SalesAgent->>Ledger: Fetch Past Pricing & Templates
        SalesAgent->>Nora: Push Notification (Draft Proposal)
        Nora->>SalesAgent: Approve & Send
        SalesAgent->>Client: Send Proposal via Email/SMS
        Client->>SalesAgent: Approve Proposal
        SalesAgent->>Ledger: Generate Invoice & Payment Link
        SalesAgent->>Client: Send Invoice
    ```
  - **Mobile UX Flow (375px)**:
    1. **Work Feed**: Nora sees a high-priority card: "New Inquiry: ACME Corp Redesign. Proposal drafted."
    2. **Proposal Review**: Nora taps the card. A clean, single-column view shows the drafted proposal with AI-suggested pricing and timeline.
    3. **Action Buttons**: Large touch targets (>= 44x44px) at the bottom: "Edit", "Approve & Send", "Reject".
    4. **Invoice Tracking**: A "Financials" tab shows pending invoices with progress bars and autonomous follow-up statuses (e.g., "Reminder scheduled for tomorrow").
  - **AI Agent Integration**:
    - **Work Triage Agent**: Parses incoming inquiries and categorizes them as "New Project Request".
    - **Sales & Revenue Assistant**: Drafts the proposal using tenant-scoped memory of past projects and standard rates. Generates a Stripe Payment Link upon proposal acceptance.
    - **Finance Assistant**: Monitors invoice payment status and schedules autonomous reminders.

  **Implementation Prompt**:
  - **User-Facing Outcome**: Nora can receive a new client inquiry, review an AI-drafted proposal on her phone, and send it with one tap. Upon client approval, an invoice is automatically generated and tracked.
  - **Critical User Journey (CUJ)**:
    1. Client submits an inquiry.
    2. Nora receives a push notification and opens the OHC app.
    3. Nora reviews the AI-generated proposal draft.
    4. Nora taps "Approve & Send".
    5. The client approves via a web link.
    6. The system autonomously generates and sends an invoice, tracking it in the ledger.
  - **Acceptance Criteria**:
    - Implement a proposal and invoice data model with strict multi-tenant isolation.
    - Create the mobile-first (375px) proposal review UI with translucent glass styling and clear action buttons.
    - Integrate the Sales Agent to generate proposal drafts based on inquiry context.
    - Automate the transition from approved proposal to generated invoice with Stripe integration.
    - Ensure zero mock data in the UI; all drafts and invoices must reflect real backend state.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
