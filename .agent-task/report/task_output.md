issue_title: "Agent-Driven Unified Inbox & CRM Engine"
issue_description: |
  # Research Report: Agent-Driven Unified Inbox & CRM Engine

  ## 1. Problem Statement
  Small business owners (e.g., Maya the Baker, Nora the Agency Principal) are overwhelmed by communication fragmentation. They receive customer inquiries across Instagram DMs, WhatsApp, email, and web forms. Missing a message means losing a sale, but manually monitoring 4-5 different apps while running the business is impossible. Furthermore, these messages lack context—when a customer DMs, the owner doesn't instantly know their order history or preferences.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify rely on third-party apps (e.g., Gorgias, Zendesk) to unify communications. These apps are expensive ($50-$300+/month) and complex to set up. Wix offers a basic unified inbox but lacks deep agentic capabilities. Social media native tools (Meta Business Suite) only handle their own ecosystem and are disconnected from the actual commerce/inventory system.
  - **The OHC Opportunity**: A built-in, multi-channel Unified Inbox that acts as the core of the Customer Relationship Assistant capability. By intercepting messages *before* the owner sees them, the AI can classify intent, link the message to a customer record, query inventory/bookings, and draft a context-aware reply.
  - **Competitor Gaps**:
    - *Shopify/Wix*: Require manual response or simple keyword auto-responders; no true AI understanding of intent combined with store data.
    - *Gorgias*: Too expensive and complex for micro-SMEs; built for support teams, not solopreneurs.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)

  ```mermaid
  erDiagram
      CUSTOMER ||--o{ CONVERSATION : "participates in"
      CUSTOMER {
          string id
          string tenant_id
          string name
          string contact_info
          string preferences
      }
      CONVERSATION ||--o{ MESSAGE : "contains"
      CONVERSATION {
          string id
          string tenant_id
          string customer_id
          string status
      }
      MESSAGE ||--o| DRAFT_REPLY : "generates"
      MESSAGE {
          string id
          string tenant_id
          string conversation_id
          string channel
          string direction
          string content
      }
      DRAFT_REPLY {
          string id
          string tenant_id
          string message_id
          string content
          string status
      }
  ```

  ### Agent Coordination Sequence

  ```mermaid
  sequenceDiagram
      participant C as Customer (Channel)
      participant I as Ingestion Webhook
      participant DB as Database
      participant A as Ambassador Agent
      participant O as Owner Mobile App

      C->>I: Sends Message (e.g., "Do you have vegan cakes?")
      I->>DB: Save Message
      I->>A: Trigger Intent Classification
      A->>DB: Query Customer History & Inventory
      A->>DB: Save DraftReply
      A->>O: Push Notification: "Draft Ready"
      O->>DB: Fetch Conversation & Draft
      O->>O: Review & "Approve"
      O->>DB: Update Draft Status to Approved
      DB->>C: Send Approved Reply
  ```

  ### Mobile UX Flow (375px)
  1. **Unified Feed**: The OHC app opens to a feed where new inquiries appear as actionable cards, not just a list of chats.
  2. **Approval UI**: A card shows the customer's message, their basic context (e.g., "Returning Customer: 3 past orders"), and the AI's drafted reply.
  3. **Action**: The owner can tap "Approve & Send", tap to "Edit", or tap "Reply Manually". Huge, touch-friendly targets (44x44px).

  ## 4. Implementation Prompt
  **Feature Name**: OHC Agent-Driven Unified Inbox
  **Target Persona**: Maya the Baker
  **Outcome**: Maya receives an Instagram DM asking "Do you have vegan options?". The OHC app pushes a notification. When she opens it, she sees the DM, alongside an AI-drafted reply ("Yes! We have vegan chocolate and vanilla. Would you like a booking link?") ready for her one-tap approval.

  **Next Actions**:
  1. Implement the core Data Models (`Customer`, `Conversation`, `Message`, `DraftReply`) with strict multi-tenant isolation.
  2. Develop the AI Intent and Draft Generation service that listens for new messages and populates `DraftReply`.
  3. Create the Mobile-First (375px) Inbox UI, featuring the "Approval Card" paradigm for reviewing and sending AI drafts.

  **Priority**: P0
  **Estimated Scope**: Large

  ## 5. Codebase Audit: Top 5 Confusing Areas to Optimize Later
  1. **Scattered Integration Webhooks**: `src/server/integrations` contains many folders for different vendors, but a unified incoming webhook gateway might simplify intent processing.
  2. **Playwright Seed Script Overlap**: Multiple places (like `e2e-seed.sql`) create test tenants, which might drift from real domain code schemas.
  3. **Complex Shared Tasks / Queues**: Background worker queue logic has several migration files (e.g. `001_shared_tasks.sql`, `003_swarm_tasks.sql`, `022_swarm_tasks_tenant_id.sql`) making the exact state of job queuing opaque.
  4. **Stripe Billing Webhook Complexity**: `src/server/api/billing_webhook.rs` is very long and has inline business logic that should perhaps be in `domain/ledger_repo.rs`.
  5. **Legacy Next.js Prototype References**: The README mentions `src/ui/next` is deprecated and to use Tauri, but it still exists and could confuse new contributors working on UI.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
