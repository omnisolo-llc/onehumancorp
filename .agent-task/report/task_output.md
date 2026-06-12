issue_title: "Implement Autonomous Auto-Reply DM Agent Protocol"
issue_description: |
  # Research Report & Implementation Brief: Autonomous Auto-Reply DM Agent

  ## Problem Statement
  Based on the competitive gap analysis (`ohc_smb_market_report.md` and `ohc_owner_work_assistant_competitive_research.md`), a top pain point for SMBs (14%) is "Omnichannel Chaos: I missed an order because it was in my DMs." Maya (the baker persona) receives custom cake inquiries via Instagram DMs and needs an invisible agent to triage them, quote prices, and secure custom-order deposits while she sleeps, moving away from manual inbox management.

  ## Research Report
  - **Competitor Landscape**:
    - **Shopify**: Sidekick helps manage the store backend but relies on heavy integration and manual triage for inbound communication via Shopify Inbox.
    - **HubSpot Breeze/Intercom Fin**: Good at generic customer service but not tailored for transactional service booking or order deposits out of the box for SMBs.
  - **OHC Gap**: OHC currently handles quoting and booking via explicit APIs or web widgets. However, it lacks a true background "Auto-Reply Agent" that actively listens to omnichannel streams (like a unified inbox) and executes actions (like generating a quote or dropping a payment link) autonomously via the KAIROS orchestration engine.
  - **Proposed Capability**: The **Agentic Negotiator & Booker**. An agent that intercepts DMs, checks calendars or pricing rules, drafts a quote, and requests a deposit.

  ## Design Doc (Architecture & UX Flow)

  ### Architecture
  We need to introduce a new service within the `src/server/services/agent` or `src/server/services/ops` boundary: an `OmnichannelInterceptorService`.
  - **Triggers**: Webhook events from integrated channels (e.g., IG DMs, WhatsApp) are routed through the `chat` module to an event bus or PostgreSQL job queue (`SKIP LOCKED`).
  - **Processing**: The `AutoReplyAgentWorker` picks up the message, loads context (tenant preferences, inventory, calendar), and queries the LLM provider (using `minimax.reason` or generic OpenAI-compatible interface) to determine intent (e.g., `Intent::QuoteRequest`).
  - **Action**: If intent is a quote, the worker calls the internal `quoting` service to generate a secure Stripe-based deposit link and replies via the `chat` module.

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Inbox (Chat Service)
      participant AI Worker Queue
      participant AutoReply Agent
      participant Quoting Service
      participant Owner App (Mobile)

      Customer->>OHC Inbox: "Do you do vegan cakes? How much?"
      OHC Inbox->>AI Worker Queue: Enqueue InboundMessage
      AI Worker Queue->>AutoReply Agent: Dequeue Message
      AutoReply Agent->>AutoReply Agent: LLM Intent Check: Quote
      AutoReply Agent->>Quoting Service: Generate Deposit Link
      AutoReply Agent->>OHC Inbox: Draft/Send Reply with Link
      OHC Inbox->>Customer: "Yes! Here is a quote and link..."
      AutoReply Agent->>Owner App (Mobile): Push Notification: "Quote drafted for vegan cake."
  ```

  ### Mobile UX Flow (375px)
  - **Owner App > Unified Inbox**: The owner sees a chat thread. The agent's auto-reply is visually distinct (e.g., a `.glassmorphism` card with a green `#34C759` sparkle badge).
  - **Action Required**: If the agent is unsure, the message is placed in a "Needs Review" queue instead of auto-sending.
  - **Zero Setup**: Maya does not configure routing rules. During the 10-minute onboarding, she toggles "Let Assistant reply to quotes."

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the backend foundation for the "Autonomous Auto-Reply DM Agent" as described in the architecture above.
  1. Define a new AI worker queue or event handler in the backend (Rust) that intercepts inbound messages.
  2. Implement the LLM intent parsing logic to determine if a message is a quote request, general question, or complaint.
  3. Wire the handler to the existing `quoting` or `booking` service to generate actionable responses (like a quote link).
  4. Ensure all database accesses respect `tenant_id` isolation (Row Level Security principles).
  5. The feature must be completely invisible to the user until triggered; it should function automatically if the tenant has enabled the auto-reply feature in their settings.
  6. **Testing**: You must write comprehensive unit tests for the intent parsing and service integration, and write at least one Playwright E2E test verifying that a simulated inbound message correctly triggers an agent-drafted quote in the UI inbox.

  ## Metadata
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
