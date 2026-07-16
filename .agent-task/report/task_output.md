issue_title: "Implement Zero-Trust Omni-Channel Identity Validation API"
issue_description: |
  ## Title
  Implement Zero-Trust Omni-Channel Identity Validation API

  ## Problem Statement
  Small business owners need to ensure that their customers' identities are verified across all channels (e.g., online checkout, in-store POS, Instagram DMs) to prevent fraud and provide a seamless, secure experience. Currently, there is no unified, zero-trust identity validation mechanism that spans all touchpoints. This forces owners to rely on siloed, channel-specific verification methods, increasing the risk of fraud and degrading the customer experience.

  ## Research Report
  **Findings & Industry Context:**
  - **Shopify/Wix:** Rely on standard email/password authentication or third-party SSO providers, which are often disjointed across different sales channels.
  - **Stripe:** Provides robust identity verification, but it is primarily focused on financial compliance rather than a unified customer identity graph.
  - **OHC Opportunity:** By implementing a Zero-Trust Omni-Channel Identity Validation API, OHC can leverage its unified customer graph to provide a seamless, secure identity verification process across all touchpoints, powered by the Customer Success Agent (The Ambassador).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Interaction (Online, POS, DM)] --> B(Omni-Channel API Gateway)
      B --> C{Zero-Trust Identity Validator}
      C -->|Valid| D[Unified Customer Graph DB]
      C -->|Invalid/Suspicious| E[Operations Agent (Fraud Alert)]
      E --> F[Owner Action Required Feed]
      D --> G[The Ambassador Agent (Contextual Reply)]
  ```

  ### Data Model (PostgreSQL)
  - `customer_identities`:
    - `id` (UUID, Primary Key)
    - `tenant_id` (UUID)
    - `channel` (Enum: 'email', 'phone', 'instagram', 'whatsapp')
    - `identifier` (String)
    - `verification_status` (Enum: 'pending', 'verified', 'flagged')
    - `trust_score` (Integer)
    - `last_verified_at` (Timestamp)

  ### Implementation Prompt
  **Feature Name**: Zero-Trust Omni-Channel Identity Validation API
  **Goal**: Implement a unified identity validation API that verifies customer identities across multiple channels using zero-trust principles.
  **CUJ**:
  1. A customer interacts via a new channel (e.g., Instagram DM).
  2. The system queries the identity validator.
  3. If the identity is unrecognized or suspicious, a verification workflow is triggered.
  4. The Operations Agent flags suspicious activity to the owner.
  **Acceptance Criteria**:
  - `customer_identities` table is created with proper tenant isolation.
  - Identity validator service is implemented.
  - Fraud alerts are routed to the owner's feed.

  ## Priority
  P1 (High)

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
