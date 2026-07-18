issue_title: "Agentic Omnichannel Customer Context Memory Architecture"
issue_description: |
  ### Problem Statement
  Owners like Maya (Baker) and Carlos (Handyman) interact with customers across multiple platforms—Instagram DMs, SMS, WhatsApp, and Web Intake Forms. Currently, these communication threads are heavily fragmented. There is no unified context, which causes agents to hallucinate, repeat questions, or drop critical context (e.g., "vegan requests" or specific repair constraints). For a truly intelligent and seamless "Workbuddy" assistant experience, OHC must remember past interactions inherently and contextually retrieve them without relying on the owner's manual data entry.

  ### Research Report
  - **Competitive Landscape**:
    - **Shopify Inbox & Ping**: Aggregates chat but heavily relies on structured order data rather than semantic conversation memory.
    - **GoDaddy Conversations & Wix Inbox**: Primarily unify channels at the UI level but lack deep AI reasoning over historical interactions.
    - **Intercom / HubSpot**: Provide excellent timeline views but are too complex ("admin portal" feel) for small business operators.
  - **Discovery**: Real-world operations show that operators manage contextual "memory" loosely in their heads or in unlinked notes apps.
  - **Conclusion**: A specialized "Agentic Context Graph" architecture within the tenant boundary is needed to automatically synthesize and retrieve omnichannel threads into a `jsonb` memory graph that is fully native to the AI Agents' prompt memory.

  ### Design Doc
  - **Architecture Details**:
    - **PostgreSQL Context Table**: A new `customer_memory_context` table using `ENABLE ROW LEVEL SECURITY`. Uses a `tenant_id` combined with `customer_id`.
    - **Schema Type**: Leverage PostgreSQL `jsonb` for semi-structured semantic memories, and `pgvector` for semantic similarity retrieval (RAG) directly at the database layer.
    - **LLM Pipeline**: Upon incoming message (via Webhooks), the AI Coordinator Department extracts "facts" and updates the `customer_memory_context`.
    - **Multi-Tenant Boundary**: All memory extractions and insertions are verified by `tenant_id` at the lowest repository levels, honoring the Zero Trust pattern.

  - **Mobile UX Flow (375px first)**:
    - **Screen**: Customer Detail View.
    - **Components**: A frosted glass (Translucent material) floating card titled "Assistant's Memory" that summarizes what the system knows about the customer, overriding disparate channel histories.

  - **AI Agent Integration Points**:
    - The **Customer & Relationship Assistant** capability will receive the vector-retrieved context automatically injected into its `system_prompt` variables during reply drafting.

  ### Implementation Prompt
  Implement the Agentic Omnichannel Customer Context Memory capability:
  1. Add the `customer_memory_context` table with `tenant_id`, `customer_id`, `context_graph` (`jsonb`), and RLS enabled.
  2. Implement the Go API repository layer to insert and retrieve context for a given tenant/customer seamlessly.
  3. Create a unified hook in the AI Job Queue (PostgreSQL `SKIP LOCKED`) to asynchronously run the memory synthesis on new messages.
  4. Display the synthesized context in the Flutter App on the 375px mobile Customer Detail View using the OHC Premium Token translucent glass styling.

  ### Priority & Scope
  - Priority: P1
  - Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
