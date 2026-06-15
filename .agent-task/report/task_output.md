issue_title: "Implement Unified Customer Identity Graph & Omnichannel Context Engine"
issue_description: |
  # Unified Customer Identity Graph & Omnichannel Context Engine

  ## Problem Statement
  Small business owners like Carlos (Handyman) and Priya (Boutique Operator) interact with customers across multiple disconnected channels: Instagram DMs, WhatsApp, Email, and in-person POS transactions. Currently, OHC lacks a unified system to link a single customer across these identities. When a customer DMs Priya on Instagram asking about a past order, the Ambassador Agent lacks the determinative context connecting their Instagram handle to their email-based order history. Without a centralized Customer Identity Resolution Graph, the AI agents provide generic responses rather than highly personalized, context-aware service, forcing the business owner to manually intervene and cross-reference data.

  ## Research Report
  - **Shopify/Wix Limitations**: Legacy platforms treat online orders, POS transactions, and chat messages as separate silos. Apps like Shopify Inbox lack deep, automatic identity resolution linking a social profile to an email address unless explicitly provided during a chat.
  - **Enterprise Solutions (CDPs)**: Platforms like Segment or Zendesk offer identity resolution but are overwhelmingly complex and expensive for SMBs.
  - **OHC Opportunity**: By implementing an invisible, agentic Customer Identity Graph, OHC can automatically merge identities based on heuristic matching (e.g., phone numbers, fuzzy name matching, implicit session links). The Ambassador and Operations agents can then RAG over this unified context to deliver magical, personalized customer service autonomously.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Ingestion Layer
          IG[Instagram DM Webhook]
          WA[WhatsApp Webhook]
          Email[Email Parser]
          POS[POS Transaction]
      end

      subgraph Identity Resolution Engine
          Match[Heuristic Matching Service]
          Merge[Identity Merge Conflict Resolver]
      end

      subgraph Storage
          DB[(PostgreSQL - Unified Customer Graph)]
          Vector[(pgvector - Interaction Embeddings)]
      end

      subgraph AI Agents
          Ambassador[The Ambassador: CS Agent]
          Manager[The Manager: Ops Agent]
      end

      IG --> Match
      WA --> Match
      Email --> Match
      POS --> Match

      Match --> DB
      Match -.-> Merge
      Merge --> DB

      DB --> Ambassador
      Vector --> Ambassador
      Ambassador -.-> |Drafts Reply| UserFeed[Mobile Agent Feed]
  ```

  ### Mobile UX Flow (375px First)
  1. **Customer Profile Card**: When Maya views a customer, she sees a unified timeline on a 375px viewport consolidating their Instagram DMs, last 3 orders, and upcoming bookings in one scrollable feed.
  2. **Agent Notification**: Maya receives a notification: "Drafted reply to Sarah (Insta DM) regarding her cake order #1029."
  3. **1-Tap Approval**: Tapping the notification shows the drafted reply with the underlying context (Sarah's previous order details) explicitly linked, allowing Maya to approve with one touch.

  ### AI Agent Integration Points
  - **Identity Resolution Agent (Background)**: Continuously scans orphaned identities and suggests merges to the owner if heuristics (like identical names but different phone numbers) are uncertain.
  - **The Ambassador (Customer Success)**: Queries the unified PostgreSQL schema and `pgvector` interaction history to draft highly contextual responses spanning multiple channels.

  ### Key Design Decisions
  - **Probabilistic vs. Deterministic Matching**: The engine will use deterministic matching (exact email/phone) automatically, and probabilistic matching (fuzzy name + location) flagged for owner review via the Agent Feed.
  - **Zero-Trust Multi-Tenancy**: The Identity Graph strictly partitions graph nodes by `tenant_id` to ensure absolute data isolation.
  - **Offline/Async Resilience**: Webhook payloads are queued (e.g., via Redis or PG SKIP LOCKED) before resolution to handle bursts and network partitions gracefully.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Goal**: Implement the Core Customer Identity Graph and Resolution Engine.
  **CUJ**:
  1. An event arrives via the Instagram DM webhook from a new social handle.
  2. The customer provides their phone number in the chat.
  3. The Identity Resolution Engine intercepts the data, looks up the phone number in the `Customers` table, and automatically links the Instagram handle to an existing customer record.
  4. The Ambassador Agent queries the unified context to summarize the customer's lifetime value and past orders.
  **Acceptance Criteria**:
  - Create the PostgreSQL schema for the Customer Identity Graph (Nodes/Edges or Unified Table with JSONB identity mappings) ensuring multi-tenant isolation.
  - Implement the `IdentityResolutionService` in the backend to handle deterministic merges based on provided phone/email.
  - Ensure the Ambassador Agent retrieves the complete omni-channel history when generating a draft.
  - No complex merge UI for the user—merges are automatic for high-confidence matches.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
