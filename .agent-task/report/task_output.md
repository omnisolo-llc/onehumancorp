issue_title: "[CRM] Autonomous Client Retention & Reactivation Engine"
issue_description: |
  # Full Research Report and Findings

  ## Identified Gap
  Small business owners, specifically service providers like Leo (Music Tutor) and Carlos (Handyman), lose up to 40% of their recurring revenue because they lack the time and tooling to manually follow up with inactive clients. They suffer from "leaky bucket syndrome." Existing platforms (Shopify, Wix) require manual segmentation and campaign drafting.

  ## Proposed Solution
  The **Autonomous Client Retention & Reactivation Engine**. A silent background orchestrator that:
  1. Detects churn risk based on historical frequency (e.g., missed typical booking cycles).
  2. Drafts a highly contextual, casual message matching the owner's tone.
  3. Sends it via the client's preferred channel (SMS/WhatsApp).
  4. Autonomously processes the re-booking if the client replies affirmatively.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CLIENT : has
      CLIENT ||--o{ INTERACTION_LEDGER : generates
      CLIENT {
          string id
          string preferred_channel
          datetime last_interaction_date
          int average_frequency_days
          float churn_risk_score
      }
      TENANT {
          string id
          boolean ai_reactivation_enabled
          string industry_type
      }
      CHURN_PREDICTION_AGENT ||--o{ CLIENT : monitors
      CHURN_PREDICTION_AGENT {
          string agent_id
          schedule cron_interval
      }
      OUTREACH_AGENT ||--o{ CHURN_PREDICTION_AGENT : triggered_by
      OUTREACH_AGENT ||--o{ INBOX_MESH : sends_via
      INBOX_MESH {
          string channel
          string status
      }
  ```

  ```mermaid
  sequenceDiagram
      participant DB as Client Ledger & Memory
      participant PA as Churn Prediction Agent
      participant OA as Outreach Agent
      participant Client as End Customer
      participant BA as Booking/Sales Agent

      PA->>DB: Scan for anomalies (e.g., frequency drop)
      DB-->>PA: Flagged Client (e.g., Tommy's mom)
      PA->>OA: Trigger Reactivation Context
      OA->>DB: Fetch context (Past purchases, preferred tone)
      OA->>Client: Send personalized SMS (Zero-touch)
      Client-->>OA: "Yes, book Tuesday 4PM"
      OA->>BA: Escalate intent to Booking Agent
      BA->>DB: Create appointment & charge deposit
      BA-->>Client: Confirm booking
  ```

  ### UI Wireframes & Screen Flow (375px Mobile First)
  *   **Screen 1 (Home/Vitality Dashboard)**:
      *   *Translucent glass* card at the top. "✨ Leo, your AI recovered 3 inactive students this week. $450 in recurring revenue secured."
      *   Clean Ubiquiti-style modular layout.
  *   **Screen 2 (Reactivation Hub)**:
      *   List of "Clients at Risk" (e.g., "Sarah (Tommy's mom) - 3 weeks late").
      *   Toggle switch: "Autopilot On" (Default: ON).
      *   If Autopilot is ON, shows a timeline of the AI's autonomous actions.
  *   **Screen 3 (Interaction Detail)**:
      *   Message preview: "AI sent SMS: Hey Sarah, want to grab Tommy's usual slot on Tuesday?"
      *   Status badge: "✅ Re-booked".
      *   "Advanced Settings" (hidden behind a cog): Edit LLM tone, adjust cron schedule, modify frequency thresholds.

  ### Key Design Decisions & Why
  1.  **Zero-Touch Default**: The system defaults to automatically contacting at-risk clients. The business owner shouldn't have to click "Approve" for every email.
  2.  **Multi-Modal Channel Routing**: The Outreach Agent automatically selects the channel where the client is most responsive (WhatsApp > SMS > Email).
  3.  **Conversational Reactivation**: Sending a plain-text, human-like SMS instead of generic graphic emails.
  4.  **Strict Tenant Isolation**: Data queries for churn prediction are strictly scoped by `organization_id` to ensure absolute Zero-Trust multi-tenancy.

  ## Implementation Prompt
  Implement the Autonomous Client Retention & Reactivation Engine.

  **User-Facing Outcome:** Business owners will see a new "Client Reactivation" card on their mobile dashboard showing how much revenue the AI recovered this week. They can tap it to see the AI's conversations with inactive clients.

  **Core User Journey (CUJ):**
  1. System identifies a client whose time-since-last-purchase exceeds their historical average by 1.5x.
  2. AI Outreach Agent sends a casual, personalized re-engagement SMS.
  3. Client replies "Yes, let's do it."
  4. AI Booking Agent finalizes the transaction.
  5. Owner sees "Re-booked" notification on their phone.

  **Acceptance Criteria:**
  - Background worker correctly identifies churn-risk clients using isolated tenant data.
  - LLM provider generates contextual SMS/WhatsApp message.
  - Full mobile-first UI parity (375px) using the design system's glassmorphism and modular cards.
  - "Grandmother Test" pass: zero manual configuration required to activate.
  - Data structures maintain strict SPIFFE/SPIRE identity and tenant isolation.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
