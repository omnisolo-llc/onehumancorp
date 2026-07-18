issue_title: "OHC Autonomous Client Intake & Smart Proposal Generation"
issue_description: |
  # Research Report: AI-Powered Client Intake & Automated Proposal Workflows

  ## Problem Statement
  Service-based small business owners and agency principals (like Nora) struggle with the manual overhead of client intake. When a new lead arrives via email or a web form, the owner must manually parse the request, schedule a discovery call, estimate effort, and draft a proposal. Existing platforms (like HoneyBook or Dubsado) offer templates but require manual data entry and lack autonomous AI agents that can actively negotiate or draft contextual proposals based on historical project data.

  ## Research Report
  - **Market Context**: Platforms like HoneyBook, Dubsado, and HelloSign are popular for client management and proposals. However, they rely on rigid templates. Shopify and Wix are not well-suited for high-touch service businesses.
  - **The OHC Opportunity**: OHC can differentiate by offering "Zero-Click Proposals". When a lead comes in, the Sales Agent can automatically draft a proposal based on similar past projects, generate a statement of work, and present it to the owner for one-tap approval before sending it to the client.
  - **Competitor Gaps**:
    - *HoneyBook / Dubsado*: Excellent workflow automation, but the initial drafting and estimation are entirely manual.
    - *Shopify*: Geared towards product sales; poor handling of service contracts or phased milestones.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Inquiry] -->|Webhook / API| B[Work Triage Agent]
      B -->|Classifies as Lead| C[Sales Agent]
      C -->|Queries Past Projects| D[Vector Store / DB]
      C -->|Drafts Proposal| E[Owner Feed]
      E -->|Owner Approves| F[Client Portal]
      F -->|Client Signs & Pays Deposit| G[Operations Agent]
      G -->|Creates Project Tasks| H[Project Board]
  ```

  ### Mobile UX Flow (375px)
  1. **Intake Notification**: Owner receives a push notification: "New Lead: Website Redesign for Acme Corp."
  2. **Review Draft Proposal**: The OHC app shows a card in the Agent Feed with a pre-drafted proposal, including estimated timeline, budget, and milestones.
  3. **Edit / Approve**: The owner can tap "Edit" to adjust numbers natively in a mobile-friendly form, or tap "Approve & Send".
  4. **Client View**: The client receives a polished, mobile-optimized link to review the proposal, e-sign, and pay the deposit via Stripe.

  ### AI Agent Integration
  - **Work Triage Agent**: Parses incoming messages/emails to identify intent (new project inquiry).
  - **Sales Agent**: Generates the proposal text, scope of work, and pricing by referencing similar past projects in the tenant's memory.
  - **Operations Agent**: Once the proposal is accepted, it automatically provisions the project, sets up tasks, and schedules invoice reminders.

  ## Implementation Prompt
  **Target Persona**: Nora (Agency Principal)
  **Outcome**: Nora receives an inquiry for a branding project. The Sales Agent automatically drafts a $5,000 proposal with 3 milestones. Nora reviews it on her phone, tweaks the timeline, and hits "Send". The client approves and pays a 50% deposit, which automatically spins up the project board for her contractors.

  **Acceptance Criteria**:
  1. Implement an endpoint to receive client inquiries and route them to the Sales Agent for proposal generation.
  2. Extend the `proposals` database schema to include `milestones` and `project_scope`.
  3. Create a mobile-first (375px) UI for owners to review and approve AI-drafted proposals in the Agent Feed.
  4. Implement a client-facing, read-only proposal view with Stripe integration for deposit collection.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
