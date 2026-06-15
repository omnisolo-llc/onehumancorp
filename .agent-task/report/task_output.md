issue_title: "[research] Architect Universal Agent Memory & Cross-Department Context Sync"
issue_description: |
  # Universal Agent Memory & Cross-Department Context Sync

  ## Problem Statement
  In the OneHumanCorp vision, multiple AI agents (Operations, Customer Success, Finance, Marketing) collaborate to run the business. Currently, if Maya (the baker) tells the `Work Triage Agent` via the app that she is out of strawberries, the `Customer Success Agent` responding to Instagram DMs may still accept strawberry cake orders. This lack of shared, persistent context creates operational friction and degrades trust in the AI assistant. Owners need the system to act as a single, unified intelligence where knowledge gathered by one agent is instantly accessible to all others.

  ## Research Report (Track 1 & Track 2)
  - **Competitor Analysis**:
    - **Shopify Sidekick/Wix AI**: Focused on narrow, isolated tasks (e.g., generating a product description or writing an email). They do not maintain a persistent, cross-functional understanding of the business's real-time state.
    - **Custom LLM Agents**: Often rely on naive RAG (Retrieval-Augmented Generation) that struggles with temporal relevance (e.g., distinguishing between an old policy and a newly enacted one).
  - **OHC Opportunity**: A Universal Agent Memory architecture where all agents read from and write to a shared, tenant-isolated Knowledge Graph. This ensures the entire system acts with unified context, eliminating hallucinations regarding operational facts (inventory, pricing, active policies).

  ## Design Doc (Track 3)
  ### Mobile UX Flow (375px)
  - **Memory Management Screen**: A simple, natural language interface where the owner can view and correct what the AI "knows."
    - *Example*: A list of learned facts: "We do not offer vegan cakes." "Delivery is free within 5 miles."
  - **Quick Updates**: The owner can simply type or speak: "Hey, we are out of strawberries until next week," and the system confirms: "Got it. I've updated inventory and will inform customers asking for strawberry items."

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Owner Input / System Event] --> B(Knowledge Ingestion API)
      B --> C[Fact Extraction & Embedding Engine]
      C --> D[(Vector Database / Knowledge Graph)]
      D -->|Read| E(Operations Agent)
      D -->|Read| F(Customer Success Agent)
      D -->|Read| G(Finance Agent)
      H[Customer DM: 'Strawberry cake?'] --> F
      F -->|Query Context: 'Strawberry availability'| D
      F -->|Response: 'Out of stock'| I[Draft Reply to Customer]
  ```

  ### Data Model & Invariants
  - **Fact Entity**: Represents a discrete piece of knowledge (e.g., Policy, Inventory State, Customer Preference).
  - **Tenant Isolation**: Every embedding and metadata record in the Vector DB must be strictly partitioned by `tenant_id`.
  - **Temporal Decay/Override**: Newer facts with overlapping conceptual domains must explicitly override older facts. The database must track the provenance (source and timestamp) of every fact.

  ### AI Department Coordination
  - **The Memory Engine (Background Process)**: Continuously evaluates incoming events (e.g., inventory updates, owner directives) and synthesizes them into discrete facts in the Vector DB.
  - **All Agents**: Before executing any action or drafting any reply, agents automatically query the Vector DB for context relevant to the task's entities.

  ## Implementation Prompt (Track 4)
  **Objective**: Build the Universal Agent Memory system and its cross-agent context injection mechanism.
  1. **Backend Infrastructure**: Implement a Vector DB layer (e.g., pgvector in PostgreSQL) with strict `tenant_id` partitioning. Define the schema for the `Fact` entity.
  2. **Ingestion Pipeline**: Create the API and background worker logic to extract facts from natural language owner inputs and system events, generate embeddings, and store them.
  3. **Agent Context Injection**: Modify the base Agent prompt architecture. Before an agent executes a task, it must generate a query to the Vector DB to retrieve relevant facts and inject them into its system prompt.
  4. **Frontend UI**: Build a 375px-optimized "Brain/Memory" screen where the owner can view, add, and delete learned facts in natural language.
  5. **Testing**: Write a Playwright E2E test: An owner adds the fact "We are closed this Friday." A simulated customer requests a booking for Friday. Verify that the Customer Success Agent drafts a reply declining the request and explaining the closure.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
