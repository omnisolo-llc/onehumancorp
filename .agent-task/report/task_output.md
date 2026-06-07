issue_title: "Agent-Driven Proposal, Contract & Invoicing Workflow"
issue_description: |
  # Research Report: Agent-Driven Proposal, Contract & Invoicing Workflow

  ## 1. Problem Statement
  Service-based agency principals and independent professionals (like Nora the Agency Principal) operate with complex, multi-stage sales cycles. They do not sell "off-the-shelf" products. Instead, they rely on custom intake, proposal drafting, contract signing, and milestone-based invoicing. Currently, this requires stitching together multiple tools (e.g., HoneyBook, Dubsado, Notion, DocuSign, Stripe), resulting in a fragmented workflow that eats up administrative time and confuses clients.

  ## 2. Research Report (Track 1 & 2)
  - **Market Context**: Platforms like Shopify and Wix are designed for transactional B2C commerce. For B2B or custom service commerce, professionals must use specialized CRM/invoicing tools like HoneyBook or Dubsado. While powerful, these tools lack deep AI autonomy—they provide templates, but do not *actively* do the work.
  - **The OHC Opportunity**: By natively integrating proposals, contract approvals, and milestone invoicing into the OHC platform, and empowering the Sales and Finance AI Agents to handle the drafting and follow-up, OHC can replace expensive CRM suites and become the single "operating system" for agencies and freelancers.
  - **Competitor Gaps**:
    - *Shopify/Wix*: No native support for custom multi-stage proposals or milestone invoicing without heavily modified B2B apps.
    - *HoneyBook/Dubsado*: Excellent workflow tools but highly manual. The user must configure every template and trigger. The AI features are limited to basic grammar checks or passive suggestions.

  ## 3. Design Doc (Track 2 & 3)
  ### Architecture & Data Model (PostgreSQL)
  - `ProjectIntake`: Captured lead requirements, budget, and timeline.
  - `Proposal`: Agent-drafted document containing scope, deliverables, timeline, and pricing.
  - `Contract`: Legal agreement linked to a Proposal, supporting digital signature state (Pending, Signed).
  - `InvoiceMilestone`: Payment schedule tied to project phases, linked to Stripe Payment Intents.
  - `ProjectTask`: Internal operations tasks generated upon contract approval.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Fills Intake Form] -->|Creates ProjectIntake| B(Sales Agent: The Closer)
      B -->|RAG via Past Proposals| C[Drafts Proposal]
      C --> D{Owner Review on Mobile}
      D -->|Owner Edits/Approves| E[Proposal Sent to Client]
      E --> F{Client Reviews & Signs}
      F -->|Contract Signed| G[Stripe Deposit Invoice Triggered]
      F -->|Contract Signed| H[Operations Agent: The Manager]
      H --> I[ProjectTasks Generated]
      G --> J{Deposit Paid}
      J --> K[Finance Agent monitors InvoiceMilestones]
  ```

  ### AI Department Coordination
  - **Sales Agent (The Closer)**: Reads `ProjectIntake` data, references past successful proposals via RAG, and drafts a customized `Proposal`. It pushes a card to the owner's Agent Feed for review.
  - **Operations Agent (The Manager)**: Once a `Contract` is signed, it automatically generates a structured list of `ProjectTask` items for the team and sets internal deadlines.
  - **Finance Agent (The Accountant)**: Monitors `InvoiceMilestone` dates. It autonomously drafts and sends invoice reminders to the client based on project progress, integrating natively with Stripe Billing.

  ### Mobile-First UX Flow (375px)
  1. **Intake Notification**: Owner receives a push notification: "New project inquiry from Acme Corp."
  2. **Review Draft Proposal**: The Agent Feed shows a card: "Sales Agent drafted a proposal for Acme Corp based on their intake form. [Review & Send]".
  3. **Approval Interface**: Tapping the card opens a clean, full-screen mobile view of the proposal. The owner can tap any section to edit text (using native mobile keyboard) or simply hit the large primary action button: "Approve & Send to Client".
  4. **Client Experience**: The client receives a polished, mobile-optimized web link to review the proposal, sign the contract (touch signature), and pay the initial deposit via Stripe, all in one unified flow.

  ## 4. Implementation Prompt (Track 4)
  **Feature Name**: Agentic Proposal & Milestone Invoicing Engine
  **Target Persona**: Nora the Agency Principal
  **Outcome**: Nora receives a project inquiry and, within minutes, reviews and sends an AI-drafted proposal directly from her phone. Upon client signature, the system automatically schedules the deposit invoice and creates project tasks.

  **Critical User Journey (CUJ)**:
  1. Nora logs into the OHC mobile app.
  2. She taps on a new intake request in her unified inbox.
  3. The Sales Agent presents a drafted proposal containing scope and pricing based on the intake context.
  4. Nora taps "Approve & Send".
  5. The client views the proposal link, signs the agreement, and pays the 50% deposit via Stripe.
  6. The system transitions the proposal state to "Signed", and the Operations Agent automatically populates Nora's project board with the initial required tasks.

  **Next Actions for Engineering**:
  1. Implement the core Data Models (`ProjectIntake`, `Proposal`, `Contract`, `InvoiceMilestone`) with strict multi-tenant isolation.
  2. Develop the AI orchestration flow where the Sales Agent reacts to a new `ProjectIntake` event to generate a draft `Proposal`.
  3. Build the 375px mobile-first Proposal Review UI with a clear "Approve & Send" workflow.
  4. Integrate Stripe for milestone-based invoicing and deposit collection upon proposal acceptance.

  **Priority**: P1
  **Estimated Scope**: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
