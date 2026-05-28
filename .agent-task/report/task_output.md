issue_title: "[Architecture] Autonomous AI Customer Reactivation Mesh"
issue_description: |
  # Issue Brief: Autonomous AI Customer Reactivation Mesh

  ## Problem Statement
  Small business owners are incredibly busy handling day-to-day operations and frequently lose track of dormant customers. Identifying lapsed buyers, canceled subscribers, or inactive clients—and reaching out to win them back—is historically a manual, time-consuming task requiring complex CRM filtering. The "win-back" marketing motion is often forgotten by our core personas (like Leo the music tutor or Maya the baker), leaving significant revenue on the table.

  ## Research Report
  - **Competitor Landscape**: Platforms like Shopify, Wix, and Mailchimp offer rudimentary "abandoned cart" or "win-back" email sequences, but they require the merchant to manually design workflows, write copy, and configure triggers. They are reactive and tool-based rather than outcome-based.
  - **User Needs**: Users need a system that autonomously detects when a customer has drifted away, formulates a contextual, personalized re-engagement strategy based on past interactions, and surfaces it for a 1-tap approval.
  - **AI Differentiation**: OHC's Autonomous AI Customer Reactivation Mesh shifts the paradigm from "build a workflow" to "approve an action." The AI acts as a dedicated retention manager that works invisibly in the background.

  ## Design Doc
  ### High-Level Architecture
  - **Trigger**: The Background Event Mesh listens for temporal and behavioral anomalies across the Universal Identity and Ledger services (e.g., >90 days since last purchase, canceled recurring booking).
  - **Data Model & Invariants**:
    - **Entities**: `CustomerLifecycleState`, `ReactivationProposal`, `OutboundInteraction`.
    - **Isolation**: Strict multi-tenant Zero Trust boundaries ensured via SPIFFE/SPIRE identity tokens. Data from one tenant's customer pool never informs another's models directly.
  - **AI Department Coordination**:
    - *Operations AI* detects the lapsed state.
    - *Marketing AI* formulates the optimal offer (e.g., 10% off, free consultation).
    - *Customer Success (CS) AI* drafts the contextual message (SMS, WhatsApp, or Email) matching the business's tone.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant EventMesh as Background Event Mesh
      participant OpsAI as Operations Agent
      participant MktgAI as Marketing Agent
      participant CSAI as CS Agent
      participant Mobile as Mobile Dashboard (375px)
      participant Customer as End Customer

      EventMesh->>OpsAI: Detects Lapsed Customer (90 days inactive)
      OpsAI->>MktgAI: Request Retention Strategy
      MktgAI->>CSAI: Strategy: 10% Discount Offer
      CSAI->>Mobile: Push: Drafted Win-Back Message for Approval
      Mobile->>CSAI: Merchant Taps "Approve"
      CSAI->>Customer: Delivers Omnichannel Message (SMS/WhatsApp)
  ```

  ### Mobile UX Flow (375px First)
  1. **Dashboard Card**: "AI Retention Manager: You have 12 lapsed customers this month."
  2. **Review Screen**: Tapping the card opens a translucent glass modal showing AI-drafted messages tailored for each customer. Example: "Hi Sarah, Maya's Bakery misses you! Here's 10% off your next custom cake order."
  3. **Action**: A large, thumb-friendly primary button at the bottom: "Approve & Send to All". An "Advanced" toggle allows the merchant to review individual messages.
  4. **Performance Targets**: The UI must load under 200ms and support offline queueing (syncing approval when connectivity returns).

  ## Implementation Prompt
  Implement the Autonomous AI Customer Reactivation Mesh. First, construct the background worker process that tracks customer lifecycle states based on transaction history and engagement. Next, wire this to the AI department (Operations, Marketing, CS) to generate personalized retention proposals. Finally, build the 375px mobile-first UI component that aggregates these proposals into a simple, 1-tap approval card on the merchant's home dashboard. Ensure strict tenant data isolation.

  ## Priority
  P1

  ## Estimated Scope
  Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
