issue_title: "Epic: Native Rust Dynamic Canned Responses & AI Workflow Engine"
issue_description: |
  # Epic: Native Rust Dynamic Canned Responses & AI Workflow Engine

  ## 1. Problem Statement
  For busy owner-operators like Maya (baker), Carlos (handyman), or Priya (boutique operator), responding to hundreds of routine customer inquiries each day (e.g., "Do you offer vegan cakes?", "What is your hourly rate?", "Are these shirts in stock?") is a major time drain that takes them away from their core work. While fully autonomous AI agents can handle drafts, owners insist on maintaining 100% accuracy, personal warmth, and total operational control.

  The retired third-party Chatwoot dependency possessed static "Canned Responses" and "Macros" to speed up workflows. However, OmniSolo (OHC) currently lacks a native, secure, and multi-tenant equivalent within its Rust-based chat platform. The gap: OHC needs a native, premium, high-performance Canned Response and Automated Workflow Engine that enables operators to trigger templated replies (dynamic customer/business context expansion) and complex state modifications in a single tap, backed by an AI semantic matching engine.

  ---

  ## 2. Research Report & Competitive Benchmarking

  ### Competitive Feature Gap Matrix
  *   **Chatwoot**: Offers simple prefix shortcuts (e.g., `/hello`) expanding to static text. Includes basic "Macros" to combine actions, but lacks native semantic AI search, multilingual translation auto-detection, and is completely retired from OHC's stack.
  *   **Shopify Inbox / Sidekick**: Supports saved replies with basic variable expansion, but lacks multi-action macros (e.g., changing status + reassigning in 1-tap) and has no local-first offline capabilities.
  *   **Wix & Squarespace Chat**: Provides disconnected email and chat templates that must be manually copied or selected from deep menus, with no mobile-first shortcut composer integration.
  *   **OmniSolo (OHC) Unfair Advantage**: By implementing this natively in Rust, OHC delivers:
      1.  **Ultra-fast Local Liquid/Mustache Parsing**: In-memory template rendering of tenant business profiles, contact records, and custom attributes directly inside the Go/Rust boundary.
      2.  **AI Semantic Matcher (pgvector)**: Analyzes incoming customer signals using vector embeddings (via MiniMax/LocalLLM) and instantly recommends the most relevant canned responses/macros on the operator's workspace feed.
      3.  **Local-First & Multi-Tenant Safe**: SQLite and PostgreSQL parity with strict, cryptographically verified Row-Level Security (RLS) policies, fully synchronized to mobile devices via PowerSync.

  ---

  ## 3. Design Doc: High-Level System Architecture

  ### Architecture Sequence (Mermaid.js)
  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      actor Operator
      participant WH as Omnichannel Webhook Gateway
      participant SV as Rust ChatService
      participant AI as AI Matcher Engine (Minimax/LocalLLM)
      participant DB as PostgreSQL (with pgvector)
      participant UI as Next.js/Tauri Operator Inbox

      Customer->>WH: Sends message "Hi, do you offer vegan cakes?"
      WH->>SV: Ingest signal & sanitize
      SV->>DB: Persist chat_message (RLS Enforced)
      SV->>AI: Trigger Semantic Canned-Response Match
      AI->>DB: Query chat_canned_responses with cosine similarity (vector)
      DB-->>AI: Return top matching canned responses & scores
      AI->>SV: Match found: "/vegan-options" (Score: 0.94)
      SV-->>UI: Push AI Recommendation over WebSocket / PowerSync
      UI->>Operator: Show macOS Glassmorphic suggestion card "Use /vegan-options?"
      Operator->>UI: Taps "Approve & Apply"
      UI->>SV: Request rendered template for contact_id & tenant_id
      SV->>DB: Load contact name, business details
      SV-->>UI: Return rendered content: "Hi Maya, yes we do!..."
      Operator->>UI: Taps "Send" (1-tap dispatch)
  ```

  ### Entity-Relationship Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      chat_inboxes ||--o{ chat_canned_responses : "scoped_by"
      chat_inboxes ||--o{ chat_macros : "scoped_by"
      chat_conversations ||--o{ chat_messages : "contains"
      chat_canned_responses {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string short_code
          text content
          vector embedding
          timestamp created_at
      }
      chat_macros {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string name
          jsonb actions
          timestamp created_at
      }
  ```

  ### UI Wireframes & 375px Mobile Screen Flow
  Adopting the premium macOS-style **Translucent Glass** visuals (`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.05)`) and strict **44x44px touch targets**:

  1.  **Composer Pop-over (Fuzzy Search Canned Responses)**:
      *   When the operator types `/` in the mobile composer, a translucent overlay slides up above the keyboard.
      *   Fuzzy search filters canned responses instantly.
      *   Each row features a `44px` height with high-contrast text: e.g., `[ /vegan ] Custom Vegan Cake Details`.
  2.  **AI Suggested Action Banner**:
      *   At the top of the mobile active chat screen, a premium floating glass banner appears:
          *   `✨ AI Suggested Reply: "Yes, we do offer vegan cake variants..."`
          *   Features a secondary translucent button: `[ Apply ]` (Touch target: `44x48px`).
  3.  **Macro Quick-Sheet Drawer**:
      *   Tapping the `⚡` icon next to the composer slides up a native iOS-like modal.
      *   Displays lists of configured multi-action macros: e.g., `[ ⚡ Close & Archive ]` or `[ ⚡ Escalate to Jun ]`.
      *   Tapping executes actions immediately, presenting a micro-loader inside the sheet, preventing double taps.

  ### AI Agent Department Coordination
  *   **CS Department (Agent)**: Monitors inbound messages. Triggers MiniMax or LocalLLM embedding extraction of incoming query texts.
  *   **Operations Department**: Inspects matching thresholds. If confidence is above `0.90` and `auto_reply_policies` allow, executes the canned response autonomously. Otherwise, sends a high-priority draft to the operator feed.
  *   **Security Guardrail**: Model-generated content and variable substitution are sanitised inside the Rust backend, preventing prompt-injection attacks from overriding tenant business profiles.

  ### Key Design Decisions
  1.  **Durable Multi-Tenant Isolation**: RLS policies are enabled on all tables (`chat_canned_responses`, `chat_macros`) using the session's active `app.current_tenant_id` context. No query can bypass this.
  2.  **In-Memory Rust Parsing**: Using a lightweight, high-performance Rust parsing crate (like `minijinja` or pure regex replacement) to render template variables (`{{contact.name}}`, `{{business.name}}`) to maintain extremely low latencies.
  3.  **Local SQLite Parity**: The database schema must work seamlessly in Cloud-native PostgreSQL (using `pgvector`) and Desktop SQLite (using local BM25 keyword-fuzzy search fallback) to maintain perfect desktop standalone parity.

  ---

  ## 4. Implementation Prompt (For Implementer Agent)

  ### User-Facing Outcome & CUJ
  "Implement the Native Dynamic Canned Response and Multi-Action Macro Engine inside OHC's native chat system.

  **Critical User Journey (CUJ)**:
  1.  Maya the baker logs in, navigates to `/inbox`, and receives a customer message: "Hey! Do you have gluten-free cakes?".
  2.  An AI Suggestion banner appears instantly at the top of the message window suggesting: `Apply GF options (/gf)`.
  3.  Maya clicks `/` in the composer, which displays a fuzzy search list of canned responses. She types `gf` and selects `/gf`.
  4.  The composer is filled with: "Hi [Customer Name], yes we do! We offer 6-inch and 8-inch gluten-free variants...".
  5.  Maya taps the `⚡` icon to trigger the Macro sheet, selects `Assign to Kitchen & Pending`, which automatically changes the conversation status to `pending`, assigns the conversation to Jun, and triggers a system notification.

  ### Acceptance Criteria
  1.  **No Mock Data**: All customer, contact, business, and canned response records must flow directly from the database and real-time backend APIs.
  2.  **Translucent Glass Design**: Deliver premium UI panels utilizing `backdrop-filter` and glassmorphic css styling in the Next.js `/inbox` and Tauri workspace.
  3.  **RLS Verified**: Write unit tests proving that a tenant cannot fetch or execute another tenant's canned responses or macros, even with direct ID manipulation.
  4.  **100% Test Coverage**: Implement unit tests in Rust for the template parsing and database repository contracts, and at least 3 comprehensive Playwright E2E tests verifying mobile composer overlays, variable expansion, and macro state transitions."

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
