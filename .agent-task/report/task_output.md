issue_title: "Chatwoot Retirement: Implement Native Rust Omnichannel Chat Inbox System"
issue_description: |
  # Native Rust Omnichannel Chat Inbox System

  ## Problem Statement
  OneHumanCorp (OHC) is replacing the external third-party dependency Chatwoot with a 100% native Rust-based omnichannel messaging architecture to provide a cohesive owner work assistant experience. Owners like Maya (baker), Carlos (handyman), and Priya (boutique operator) need all incoming messages across Instagram, WhatsApp, Email, Web Chat, and SMS to be seamlessly unified into one assistant-led inbox without requiring a complex multi-tool setup.

  ## Research Report
  - **Goal**: Full retirement of Chatwoot as an external service.
  - **Source Audit (Chatwoot)**: Chatwoot heavily relies on Ruby models for `Account`, `Inbox`, `Conversation`, `Message`, and multiple channel types (`WebWidget`, `WhatsApp`, `Email`, `Instagram`, `FacebookPage`, etc.).
  - **Competitive Analysis**: High-performing SMB systems like Wix and Shopify provide integrated native unified inboxes rather than stitching external tools. Our multi-tenant SaaS architecture needs rigorous tenant boundary enforcement using PostgreSQL RLS (row level security).
  - **Architecture Alignment**: The OHC Backend uses Rust, and the prompt strongly dictates implementing a **native Rust omnichannel chat system inside `onehumancorp/mono`** in `src/server/ohc`.

  ## Design Doc
  ### Data Model (Rust / DB Schema definition)
  - `Inbox`: Represents a collection bin for one or more channels. Scoped by `tenant_id`.
  - `Channel`: The source of the message (e.g. `WebWidget`, `Email`, `WhatsApp`).
  - `Conversation`: A threaded discussion between a contact and the owner/agents. Links to an `Inbox`.
  - `Message`: Individual messages within a conversation.
  - `Contact`: The end-user (customer) communicating with the owner.

  ### Multi-Tenancy & Isolation
  - All DB entities must include a `tenant_id`.
  - Use Row Level Security (RLS) on PostgreSQL.
  - Lock synchronization via Redis (Redlock) keyed by `tenant_id` and `conversation_id`.

  ### Mobile UX Flow (375px first)
  1. **Triage Feed**: Owner sees a unified list of active conversations. Unread messages are bolded.
  2. **Conversation View**: Tapping a conversation opens a mobile-optimized chat view with quick-action AI buttons (e.g. "Draft Reply", "Create Booking").
  3. **Seamless Handoff**: The owner can seamlessly hand off to an AI agent for automated resolution.

  ### AI Agent Integration
  - **Customer & Relationship Assistant**: Drafts replies by reading context from the `Conversation` and previous `Message`s.
  - **Work Triage**: Analyzes new `Message`s to categorize them as tasks or urgent actionable items on the owner's dashboard.

  ## Implementation Prompt
  Implement the core domain data models and basic service traits for the Native Rust Omnichannel Chat system.
  1. In `src/server/ohc/domain`, create modules for `inbox.rs`, `conversation.rs`, `message.rs`, and `contact.rs`.
  2. Define the core structs for these entities, ensuring each has a `tenant_id` string for strict multi-tenant isolation.
  3. Ensure the structs are serializable and implement basic constructors.
  4. Integrate these modules into `src/server/ohc/domain/mod.rs`.
  5. Provide 100% unit test coverage for the new domain structs.

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
