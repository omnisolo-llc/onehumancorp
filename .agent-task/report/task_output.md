issue_title: "Implement Multi-Tenant Agentic Context Memory Layer"
issue_description: |
  # Research Report: Multi-Tenant Agentic Context Memory Layer

  ## 1. Problem Statement
  Currently, OHC agents are largely stateless between sessions unless explicitly referencing past specific messages. Small business owners like Maya and Nora need an assistant that intrinsically remembers context across all interactions—customer preferences, internal operational rules, and project states. Crucially, this memory MUST be strictly isolated per tenant (Workspace). Without a dedicated, multi-tenant memory layer, agents will repeat mistakes or ask the owner for the same information repeatedly.

  ## 2. Research Findings
  Our research into "Multi-Tenant Agentic Context Memory" indicates that simply dumping past interactions into a standard prompt window is inefficient and error-prone. A structured, vector-based or hierarchical memory store (similar to the AutoDream Pipeline referenced in KAIROS documentation) is required. This memory store must:
  1. Have an absolute hard boundary on `tenant_id`.
  2. Be searchable by the main agent for relevant context before executing an action.
  3. Be user-editable and auditable by the owner (so they can correct the AI).

  ## 3. Design Doc

  ### Architecture
  We will introduce a `MemoryLayer` into the backend and UI.
  - **Data Model**:
    - We will need a storage representation for memories that supports semantic search (e.g. embeddings) and strict tenant isolation via row-level security. Every access must be filtered by `tenant_id`.
  - **Agent Coordination**:
    - **Retrieval**: Before drafting a response or creating a task, the Triage agent queries the MemoryLayer for relevant facts about the customer or the task type.
    - **Consolidation**: A background job (AutoDream) periodically reviews recent interactions and summarizes them into new memories.

  ### Mobile UX Flow (375px)
  - The owner has a "Memory" section in their main settings or dashboard.
  - They see a list of "Facts I Remember" categorized clearly (e.g., "Customer: Carlos prefers texts", "Operations: We don't deliver on Sundays").
  - The owner can add new facts manually, edit existing ones, or delete them.

  ## 4. Implementation Prompt
  **To the Implementer:**
  Your task is to implement the Multi-Tenant Agentic Context Memory Layer.
  1.  **Database Migration**: Create the necessary database schema, ensuring strict `tenant_id` scoping and row-level security.
  2.  **Backend Services**: Create Go services for operations on memories, ensuring `tenant_id` is always passed and validated.
  3.  **Agent Integration**: Modify the base agent prompt construction to include relevant memories retrieved based on the current context.
  4.  **UI Updates**: Build the "Memory" management screen in the Flutter mobile-first UI. Ensure it works flawlessly on 375px screens and uses our Translucent Glass styling.
  5.  **Tests**: Write Playwright E2E tests verifying that a memory added by the owner is correctly utilized by the agent in a subsequent interaction, and unit tests enforcing tenant isolation.

  ## 5. Priority & Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
