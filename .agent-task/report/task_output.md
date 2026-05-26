issue_title: "[architecture] Universal Customer Context Graph & AI Memory Mesh (UCMG)"
issue_description: |
  ## Title
  [architecture] Universal Customer Context Graph & AI Memory Mesh (UCMG)

  ## Problem Statement
  Currently, when a small business owner like Maya (the baker) interacts with a customer across multiple channels—Instagram DMs, email, phone, and in-person POS—the context of those interactions is fragmented. Her AI agents handling Instagram might not know that the customer already paid a deposit via a payment link sent via email. This forces the business owner to manually synthesize information, coordinate between disconnected systems, and potentially provide redundant or contradictory responses to the customer. Small business owners need an invisible, unified memory layer where all AI departments (Sales, Support, Operations) and channels converge onto a single, omni-channel customer identity.

  ## Research Report
  ### Findings
  - **Context Fragmentation:** AI agents in OHC currently process events linearly or per channel, lacking a unified customer context graph.
  - **Competitor Gap:**
    - Shopify: Relies on disparate apps (e.g., Gorgias for support, native for orders) without deep agentic shared memory.
    - Wix/Squarespace: Basic CRM features but no intelligent, proactive AI memory that acts autonomously.
  - **Opportunity:** By creating a Universal Customer Context Graph & AI Memory Mesh (UCMG), OHC can provide an "invisible CRM" that all AI agents can query in real-time. This guarantees that an Instagram auto-reply agent knows about a failed payment or an upcoming booking from another channel.

  ## Design Doc
  ### Architecture Diagram

  ```mermaid
  erDiagram
      CUSTOMER ||--o{ IDENTITY_LINK : "has"
      IDENTITY_LINK {
          string channel_id "e.g., ig_123, email, phone"
          string platform "Instagram, WhatsApp, Email"
      }
      CUSTOMER ||--o{ MEMORY_NODE : "generates"
      MEMORY_NODE {
          string node_type "Transaction, Conversation, Booking"
          string content "Vectorized memory data"
          timestamp created_at
      }
      TENANT ||--o{ CUSTOMER : "manages"
  ```

  ```mermaid
  sequenceDiagram
      participant User as Customer (IG DM)
      participant Channel as IG Webhook
      participant AI_Ops as Operations Agent
      participant UCMG as Memory Mesh (UCMG)
      participant AI_Sales as Sales Agent

      User->>Channel: "Did my deposit go through?"
      Channel->>AI_Ops: Trigger IG DM
      AI_Ops->>UCMG: Query customer context (ig_123)
      UCMG-->>AI_Ops: Return merged identity (ig_123 = email@domain.com) + recent deposit success
      AI_Ops->>User: "Yes, Maya received your $50 deposit!"
  ```

  ### UI/UX & Mobile-First Flow (375px)
  - **Zero-Config CRM:** Maya does not configure this. The UCMG builds the graph invisibly in the background.
  - **The "Customer Card" (Mobile 375px):**
    - When Maya clicks on a customer from an order or an inbox message, she sees a clean, frosted-glass card.
    - **Header:** Customer name & synthesized profile picture.
    - **Summary Pill:** "VIP Customer" / "Awaiting Payment" (AI-generated based on memory mesh).
    - **Activity Timeline:** A unified vertical feed merging Instagram DMs, email threads, POS transactions, and agent summaries into a single scroll.
    - **Quick Actions:** "Send Payment Link", "Book Appointment", floating above the timeline.
    - **Accessibility:** Large tap targets (min 44px), legible typography (min 16sp), passing the grandmother test. No complex data joins visible to the user.

  ### Security & Integrity
  - **Multi-Tenant Isolation:** The UCMG strictly partitions customer graphs by `tenant_id` (`organization_id`). AI memory queries are scoped to the authenticated tenant context.
  - **Agent Access:** Agents query the UCMG via zero-trust SPIFFE/SPIRE authenticated gRPC/mTLS channels.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your objective is to implement the backend data structures, APIs, and the mobile-first React/Tauri UI for the Universal Customer Context Graph & AI Memory Mesh (UCMG).

  **Acceptance Criteria:**
  1. An omni-channel activity feed on the Customer Card UI must display events from at least two different sources (e.g., an order and a simulated chat message) merged chronologically.
  2. The AI Context API must be able to return a synthesized summary of a customer's state given an identity identifier (like a phone number or IG handle).
  3. The UI must render perfectly on a 375px viewport width, utilizing our translucent glass design tokens and modular dashboard cards.
  4. All data queries must enforce strict tenant isolation (`organization_id`).

  Design the database schema (Postgres/vector DB) and API contracts as you see fit to satisfy these requirements. Ensure the UI feels invisible and premium—do not expose the complexity of the graph or vector memory to the end user.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
