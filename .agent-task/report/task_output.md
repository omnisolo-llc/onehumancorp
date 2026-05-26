issue_title: "[Architecture] Invisible Milestone & Escrow Ledger"
issue_description: |
  # Issue Brief: Invisible Milestone & Escrow Ledger

  ## Problem Statement
  Trust Paralysis & Deposit Friction: For high-ticket services (e.g., Carlos the handyman building a deck, Leo tutoring a 6-month masterclass), asking for a 50% upfront deposit or full payment in advance creates friction with the customer. On the flip side, starting work without payment guarantees puts the business owner at severe risk of non-payment. Traditional escrow or milestone payment platforms are too complex, requiring separate logins, legal agreements, and manual approvals for non-technical small business owners.

  ## Research Report
  - **Competitor Landscape**: Platforms like Upwork have escrow built-in but take 10-20% cuts and are marketplace-locked. Shopify lacks native service milestone support. Stripe requires custom development to build multi-party escrow workflows via Connect.
  - **User Needs**: Service providers need a way to confidently start work knowing funds are secure, and customers need the assurance that their money is only released upon agreed milestones, completely frictionlessly.
  - **AI Differentiation**: OHC uses an Invisible Escrow Ledger managed by an AI Finance Agent. When a quote is accepted, the customer funds the escrow via a secure link. As work progresses (e.g., Carlos texts "Finished the framing"), the Finance Agent autonomously validates the milestone, notifies the customer, and triggers the partial payout—with zero dashboard interaction.

  ## Business Journey Mapping
  - **Acquisition**: The professional creates a high-ticket quote (>$1,000) using natural language or an AI prompt. The invisible escrow feature serves as a trust signal on the quote, increasing conversion.
  - **Onboarding**: Zero accounting or separate banking setup required. The AI auto-provisions a multi-tenant escrow ledger when the quote is created.
  - **Activation**: Customer clicks the quote link and pays the total upfront securely via Apple Pay/Google Pay. Funds move to an FBO account.
  - **Retention**: Milestones seamlessly unlock funds with a single tap from the customer upon seeing a photo update from the owner.
  - **Revenue**: The platform earns a small basis point margin on the escrow float or transaction fee, while the owner gets instant treasury access upon milestone approval.
  - **Referral**: Trust is maximized for the customer, leading to higher word-of-mouth referrals.

  ## Design Doc
  ### High-Level Architecture

  ```mermaid
  graph TD;
      Owner[Business Owner SMS/App] -->|Photo/Text| OpsAgent[Operations Agent];
      OpsAgent -->|Verify Milestone| FinAgent[Finance Agent];
      FinAgent -->|1-Tap Approval| Customer[Customer SMS/Web];
      Customer -->|Approve| FinAgent;
      FinAgent -->|Release Trigger| Escrow[Invisible Escrow Ledger];
      Escrow -->|Payout| Wallet[Treasury Wallet];
  ```

  ### Data Model & Invariants

  ```mermaid
  erDiagram
      ProjectEscrow ||--o{ Milestone : has
      ProjectEscrow ||--o{ LedgerTransaction : records
      ProjectEscrow {
          string id
          string tenant_id
          float total_amount
          string fbo_account_id
          string status
      }
      Milestone {
          string id
          float release_amount
          string status
          string proof_required
      }
      LedgerTransaction {
          string id
          float amount
          string timestamp
          string from_account
          string to_account
      }
  ```

  ### AI Department Coordination

  ```mermaid
  sequenceDiagram
      participant Owner
      participant OpsAgent
      participant FinAgent
      participant Customer
      participant Ledger
      Owner->>OpsAgent: "Finished framing. Here's a pic."
      OpsAgent->>OpsAgent: Verify pic matches Milestone 1
      OpsAgent->>FinAgent: Request Milestone 1 Release
      FinAgent->>Customer: SMS: "Carlos finished framing. Release $1500?"
      Customer->>FinAgent: Tap "Approve"
      FinAgent->>Ledger: Execute transfer to Owner
      Ledger->>Owner: "Milestone 1 funded: $1500"
  ```

  ### Mobile UX Flow (375px First)
  1. **Quote Creation**: Owner creates a quote, AI suggests splitting into 3 milestones. (Clean card-based layout, translucent glass materials)
  2. **Customer Funding**: Customer pays the total upfront into the invisible escrow via Apple Pay/Google Pay.
  3. **Milestone Update**: Owner snaps a photo of progress -> AI asks "Release Milestone 1?" -> Owner taps "Yes".
  4. **Customer Approval**: Customer gets an SMS with the photo -> Taps "Looks great! Release Funds".
  5. **Instant Payout**: Funds hit the owner's autonomous treasury wallet instantly.

  ### Performance & Offline Targets
  - **Latency**: Operations and Finance agents must process inputs and dispatch the SMS approval within 2 seconds. Ledger transactions must commit under 100ms.
  - **Offline Capability**: Owner updates (e.g. photos taken in a basement with no service) are queued locally and synchronized via the OHC-HA Hybrid Architecture event mesh when connection is restored.

  ### Zero Trust & Security
  - **Multi-Tenant Isolation**: The Escrow Ledger is strictly multi-tenant. Tenant boundaries are enforced at the database level using RLS (Row Level Security).
  - **Workload Identity**: All inter-agent communications (e.g., Ops Agent -> Finance Agent) and API calls to the Escrow Ledger are cryptographically signed and mTLS validated using SPIFFE/SPIRE, ensuring absolute zero-trust verification.

  ## Implementation Prompt
  Implement the "Invisible Milestone & Escrow Ledger". Design the multi-tenant secure sub-ledger system to hold customer funds upon quote acceptance. Integrate the Finance Agent to monitor milestone completion signals (e.g., from user text/photo uploads) and autonomously handle the customer approval and fund release workflows. Ensure the entire process requires zero traditional accounting configuration from the user.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
