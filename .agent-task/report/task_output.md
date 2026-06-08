issue_title: "Unified Agentic Knowledge Base & Policy Engine"
issue_description: |
  ## Problem Statement
  Small business owners generate crucial business context across multiple fragmented locations: DMs, email threads, random notes, PDFs of supplier contracts, and their own heads. Existing platforms lack a central "brain" where the user can dump raw unstructured information (e.g., "Vegan cakes need 48h notice, charge $10 extra") and have it instantly applied across all customer interactions and agent behaviors. Currently, if Maya wants to enforce a 48h notice for vegan cakes, she must manually update FAQs, product descriptions, and remember to tell customers in DMs.

  ## Research Report
  - **Market Context**: Platforms like Shopify and Wix rely on structured fields (shipping settings, product variants, FAQ apps). They require the owner to explicitly configure every rule in the correct specific UI location. There is no unified "policy engine."
  - **The OHC Opportunity**: OHC can differentiate by offering a "Knowledge Vault" where users upload documents or type natural language policies. The backend automatically chunks, embeds, and serves this knowledge to all active AI agents (Ambassador, Operations, Sales), ensuring consistent behavior without complex configuration.
  - **Competitor Gaps**:
    - *Shopify*: Rules must be coded in Flow or configured via specific apps.
    - *Wix/Squarespace*: Static FAQ pages only; no dynamic enforcement of policies in conversational commerce.
    - *Standalone RAG tools*: Disconnected from the commerce engine, requiring Zapier/Make to sync state.

  ## Design Doc
  ### Architecture
  ```mermaid
  graph TD
      A[Owner Input: Notes, Docs, Voice] --> B[Knowledge Ingestion Pipeline]
      B --> C{Chunking & Embedding}
      C --> D[(Vector Database - Chroma/pgvector)]
      D --> E[Unified Policy Engine]
      E --> F[Ambassador Agent: DMs]
      E --> G[Operations Agent: Booking]
      E --> H[Sales Agent: Quotes]
  ```

  ### Mobile UX Flow
  1. **Owner View (375px)**: A simple "Brain" or "Knowledge" tab with a large input field ("What should the assistant know?") and an upload button for docs/images.
  2. **Knowledge Cards**: Previously added facts are displayed as editable cards.
  3. **Agent Integration**: When an agent drafts a reply enforcing a policy (e.g., "Sorry, we need 48h notice for vegan cakes"), the UI includes a small badge indicating which knowledge card influenced the decision.

  ### AI Integration Points
  - **Ingestion**: An LLM cleans and normalizes raw user input into concise policy statements before embedding.
  - **Retrieval**: All agent prompts include a mandatory RAG step against the tenant's Knowledge Base, filtered by context relevance.

  ## Implementation Prompt
  **Target Persona**: Maya the Baker
  **Outcome**: Maya can type "Vegan cakes require 48h notice" into her Knowledge Base, and the Ambassador agent will immediately start enforcing this rule when replying to Instagram DMs.

  **Next Actions**:
  1. Create the `KnowledgeBase` and `KnowledgeChunk` PostgreSQL models with vector embeddings support (pgvector).
  2. Implement the Knowledge Ingestion API: accept text, process via LLM for normalization, embed, and store with tenant isolation.
  3. Update the core Agent Context Builder to perform semantic search against the tenant's Knowledge Base for every incoming message.
  4. Build the mobile-first "Knowledge Vault" UI (375px) to allow adding, editing, and deleting knowledge items.

  **Acceptance Criteria**:
  - Adding a new rule via the API instantly affects the behavior of the Ambassador agent in E2E tests.
  - UI must operate smoothly on a 375px viewport.

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []