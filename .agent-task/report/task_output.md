issue_title: "Implement Unified Knowledge & Compliance Memory System (RAG)"
issue_description: |
  ## Problem Statement
  Small business owners (e.g., Nora the Agency Principal, Maya the Home Baker) possess critical business knowledge scattered across PDFs, pricing sheets, past email threads, and informal notes. Existing platforms fail to turn this unstructured data into usable intelligence. Without a centralized "Memory System," AI agents (like The Ambassador or Operations Assistant) cannot answer customer inquiries accurately or draft contextual proposals, forcing the owner to constantly intervene and provide context manually.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify & Wix**: Rely heavily on structured data (product catalogs, variants). Their AI (e.g., Shopify Sidekick) can query orders and products but cannot ingest a PDF of a custom catering menu or a refund policy document to answer specific customer questions.
  - **Custom GPTs / Notion AI**: Offer document Q&A but are disconnected from the actual business execution layer (e.g., cannot draft a quote based on the document and send it via Stripe).
  - **OHC Opportunity**: Implement a tenant-isolated Retrieval-Augmented Generation (RAG) system. This "Knowledge Assistant" capability will allow owners to upload documents or paste notes, which are then chunked and embedded into a vector database. Other OHC agents will automatically query this Memory System to resolve context before drafting replies or proposals.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Owner Uploads PDF/Note] -->|Mobile UI 375px| B(Knowledge API Gateway)
      B --> C[Document Processing Worker]
      C -->|Text Extraction & Chunking| D
      D -->|Embedding Generation| E(Gemini Embedding Model)
      E --> F[(Tenant-Isolated Vector DB)]

      G[Customer DM via The Ambassador] --> H[Agent Feed]
      H --> I{Context Retrieval}
      I -->|Query| F
      I -->|Context Injected| J[Gemini Pro Intent Resolution]
      J --> K[Drafted Action Card]
      K --> L[Mobile App Feed 375px]
  ```

  ### Mobile UX Flow (375px)
  1. **Upload View**: A simple, translucent-glass styled card allowing the owner to snap a photo of a document, upload a PDF, or paste text. Touch targets for upload buttons are >44x44px.
  2. **Processing State**: A truthful, animated indicator showing the document is being "Learned" by the system.
  3. **Memory Management**: A feed of "Learned Documents" with a simple 1-tap delete option.

  ### AI Agent Integration
  - The Knowledge & Compliance Assistant manages the vector database.
  - All other agents (Customer Relationship, Sales, Operations) use a standardized internal API to query the Knowledge Assistant before taking action, ensuring all drafted replies and decisions respect the owner's custom policies and unstructured data.

  ### Key Design Decisions
  - Strict tenant isolation using `tenant_id` at the vector database level to prevent data leakage between businesses.
  - Background processing for document ingestion to prevent blocking the mobile UI.

  ## Implementation Prompt
  **Feature Name**: OHC Unified Knowledge Memory System (RAG)
  **Target Persona**: Nora (Agency Principal) / Maya (Home Baker)
  **Outcome**: The owner can upload a custom catering menu or agency pricing PDF. When a customer asks about specific pricing, the Customer Relationship agent retrieves this unstructured data, drafts an accurate quote, and presents it to the owner for 1-tap approval in the Agent Feed.

  **Next Actions for Engineering:**
  1. Implement the API endpoints for document upload and unstructured text ingestion.
  2. Create a background worker (PostgreSQL SKIP LOCKED queue) to extract text, chunk it, and generate embeddings.
  3. Integrate a vector storage solution with strict tenant isolation.
  4. Develop the internal service layer allowing other agents to query the memory system.
  5. Build the mobile-first (375px) UI for uploading and managing knowledge documents, adhering to the OHC Premium Token glassmorphism design.

  Do NOT prescribe specific vector databases or text extraction libraries here; design the interfaces and integration points.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
