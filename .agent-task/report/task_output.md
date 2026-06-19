issue_title: "Implement Agentic Universal Helpdesk & Autonomous Dispute Resolution Engine"
issue_description: |
  # Research Report: Agentic Universal Helpdesk & Autonomous Dispute Resolution Engine

  ## 1. Problem Statement
  Customer disputes and support requests (e.g., "The service took longer than expected," "The dress arrived damaged," or "I need to reschedule") represent a massive operational burden for micro-SMB owners. Traditional platforms provide a "unified inbox," but they still force the owner to manually investigate the issue, open a separate POS or scheduling tab, process a refund, adjust inventory, and write an apologetic reply. For personas like Carlos (Handyman) and Priya (Boutique Owner), this context-switching on a mobile device disrupts their day-to-day work and leads to delayed responses, frustrated customers, and lost revenue.

  ## 2. Research & Market Landscape
  - **Traditional Ecosystems (Shopify, Wix):** Provide integrated inboxes but require human intervention to execute complex workflows (e.g., combining a partial refund with a restock and a drafted apology).
  - **AI Leaders (Intercom Fin, Zendesk AI):** Excel at answering FAQs autonomously, but struggle to securely execute cross-domain operational tasks (like modifying a booking schedule + refunding a Stripe charge) without complex developer-built integrations.
  - **The OHC Opportunity:** By leveraging the Agent Feed and inter-agent coordination, OHC can create an "Autonomous Resolution Engine." Instead of just drafting a reply, the platform can propose a comprehensive, multi-step resolution package directly in the owner's mobile feed for one-tap approval.

  ## 3. Deep Dive Architecture Design

  ### Data Model & Invariants
  - **Dispute Entity:** Captures the customer context, the related order/booking ID, and the initial sentiment classification.
  - **ResolutionProposal Entity:** A composite action plan drafted by the AI. It contains the proposed message, the financial action (e.g., 10% refund), and operational actions (e.g., mark item as damaged in inventory, propose new time slot).
  - **Invariants:** All actions must be tentative (staged) until the owner explicitly taps "Approve." Financial transactions must utilize idempotency keys.

  ### AI Department Coordination
  - **Customer Success Agent:** Classifies the incoming message as a dispute/issue. Drafts the apologetic response.
  - **Finance Agent:** Calculates a suggested refund or credit based on the tenant's predefined policies.
  - **Operations Agent:** Checks inventory or calendar availability to propose a replacement or reschedule.
  - **Orchestration:** The agents collaborate (via Redis locks and the msgbus) to construct a single `ResolutionProposal` payload.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant WebhookLayer
      participant CustomerSuccessAgent
      participant FinanceAgent
      participant OperationsAgent
      participant AgentFeed
      participant OwnerApp

      Customer->>WebhookLayer: Sends complaint (DM/Email)
      WebhookLayer->>CustomerSuccessAgent: Ingest Event
      CustomerSuccessAgent->>CustomerSuccessAgent: Classify Intent (Dispute)
      CustomerSuccessAgent->>FinanceAgent: Request Refund Proposal
      FinanceAgent-->>CustomerSuccessAgent: Proposal: 10% Refund
      CustomerSuccessAgent->>OperationsAgent: Request Ops Action
      OperationsAgent-->>CustomerSuccessAgent: Proposal: Reschedule
      CustomerSuccessAgent->>AgentFeed: Publish Combined ResolutionProposal
      AgentFeed->>OwnerApp: Push Action Card (375px viewport)
      OwnerApp->>AgentFeed: Owner taps "Approve"
      AgentFeed->>FinanceAgent: Execute Refund (Stripe)
      AgentFeed->>OperationsAgent: Update Booking
      AgentFeed->>Customer: Send Drafted Reply
  ```

  ### Mobile-First UX Flow (375px)
  1. **Notification:** "New dispute from John D. regarding order #102."
  2. **Action Card UI:** The card displays:
     - **Summary:** "John received a damaged item."
     - **Proposed Reply:** [Editable text block with drafted apology].
     - **Proposed Actions:** [Toggle] Issue $15 refund. [Toggle] Mark 1 unit as damaged in inventory.
     - **Primary CTA:** A prominent "Approve & Resolve" button (≥ 44x44px).
  3. **Interaction:** The owner can toggle actions off, edit the text, and tap "Approve". Network resilience ensures the action is queued if offline.

  ## 4. Implementation Prompt
  **Feature Name:** Agentic Dispute Resolution Engine
  **Target Personas:** Carlos (Handyman) and Priya (Boutique Owner)

  **Outcome:** Deliver a cohesive backend workflow and mobile UI where a customer complaint automatically generates a multi-action `ResolutionProposal`.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. **Ingestion & Generation:** When a dispute event enters the system, the Customer Success Agent must coordinate with the Finance and Operations Agents to generate a composite `ResolutionProposal` (Message + Refund/Credit + Ops Action).
  2. **UX Presentation:** The proposal must appear as a single, actionable card in the owner's Agent Feed on a 375px viewport.
  3. **Owner Control:** The owner must be able to view, edit, or toggle the individual components of the proposal.
  4. **Execution:** Upon tapping "Approve", the system must atomically execute the selected actions (e.g., process refund via Stripe integration, update internal inventory) and send the reply.
  5. **Verification:** Implement full Playwright E2E tests validating the end-to-end flow from event ingestion to UI approval and backend state change.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
