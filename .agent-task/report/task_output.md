issue_title: "[Architecture] Dynamic High-Performance AI Knowledge & RAG Engine"
issue_description: |
  # Dynamic High-Performance AI Knowledge & RAG Engine

  ## Problem Statement
  Nora (the agency principal) and Leo (the music tutor) have a scattered collection of policies, client contracts, past Q&A, and onboarding documentation. When a client or student asks a complex question ("What is the refund policy for missed lessons if I gave 24 hours notice?"), the AI assistant either guesses incorrectly, gives a generic response, or bothers the owner. The owner needs the assistant to instantly access and synthesize answers from their specific business documents without manually updating "agent instructions."

  Currently, the OHC memory and RAG (Retrieval-Augmented Generation) layer is either too slow, lacks multi-tenant isolation, or doesn't support automatic ingestion of uploaded documents (PDFs, Word docs, TXT) into a queryable knowledge graph. The owner shouldn't have to understand "vector databases" or "embeddings"—they just need to drop a document into a folder and have the AI immediately know it.

  ## Research Report
  - **Market Context**: Platforms like Notion AI, Microsoft Copilot, and custom OpenAI GPTs allow users to upload files to build a knowledge base. Wix and Shopify have limited RAG capabilities for store policies. Notion AI shines by seamlessly updating its index as users edit documents.
  - **Competitive Analysis (Shopify / Wix / Squarespace / GoDaddy)**: Shopify Sidekick can reference store policies but struggles with unstructured, multi-format document uploads. Wix's AI is mostly for site generation, not operational Q&A. Squarespace and GoDaddy lack any advanced RAG-based operational assistant.
  - **The Gap**: OHC needs a multi-tenant, edge-cached, highly accurate RAG pipeline that automatically chunk, embeds, and indexes any document the owner uploads, making it instantly available to the Customer & Relationship Assistant.
  - **The Opportunity**: Build a Zero-Trust, background-processed Document Ingestion Pipeline and a fast Vector Search capability that acts as the "Knowledge Assistant," allowing other agents to query the owner's brain effortlessly.

  ## Design Doc
  ### Mobile UX Flow (375px)
  1. **Knowledge Hub**: The owner opens the "Knowledge" tab. A clean, translucent glass UI shows a list of learned documents.
  2. **Upload/Add**: The owner taps the "+" button, selecting a PDF or taking a photo of a document (e.g., a paper menu).
  3. **Learning State**: A status indicator shows the Knowledge Assistant "learning" the document (animating briefly).
  4. **Verification Test**: The owner can tap a "Test Knowledge" button to ask a mock question. The AI answers using the newly uploaded document, citing the source.

  ### AI Agent Integration Points
  - **Knowledge Assistant**: Manages the indexing and chunking of documents.
  - **Customer Assistant**: Before drafting a reply to a customer, it automatically queries the Vector DB for relevant context and policies.
  - **Operations Assistant**: Uses the Knowledge base to pull up standard operating procedures (SOPs) when an anomaly is detected.

  ### Key Design Decisions
  - **Asynchronous Ingestion**: Document processing must happen in the background via the job queue to keep the mobile UI fast.
  - **Multi-Tenant Isolation**: Vector embeddings must be strictly partitioned by `tenant_id` to prevent data leakage between different business owners.
  - **Invisible AI**: No mention of "vectors," "embeddings," or "chunking" in the UI. It simply says "Assistant is learning."

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner App (Flutter)
      participant OHC API
      participant Job Queue
      participant Embedding Worker
      participant Vector DB / PGVector

      Owner App->>OHC API: Upload Document (tenant_id)
      OHC API->>Job Queue: Enqueue 'IngestDocument' Job
      OHC API-->>Owner App: Status: "Learning..."
      Job Queue->>Embedding Worker: Dequeue Job
      Embedding Worker->>Embedding Worker: Chunk Document text
      Embedding Worker->>Embedding Worker: Generate Embeddings (LLM API)
      Embedding Worker->>Vector DB / PGVector: Store vectors with strict tenant_id
      Embedding Worker->>OHC API: Update Document Status (LEARNED)
      OHC API-->>Owner App: Status: "Learned. Ready."
  ```

  ## Implementation Prompt
  Implement the backend Knowledge Ingestion Pipeline and the frontend Knowledge Hub UI.
  - **Backend (Go/Rust)**: Create a worker that picks up `IngestDocument` jobs, extracts text from files, chunks the text, calls an embedding model, and stores the results in the database (e.g., using pgvector) enforcing `tenant_id` isolation.
  - **API**: Add endpoints to upload documents and query the knowledge base.
  - **Frontend (Flutter)**: Build the Knowledge Hub screen following the 375px mobile-first and translucent glass design principles. Allow users to upload files and view learning statuses.
  - **CUJ**: An owner navigates to the Knowledge tab, uploads a text file of their store policy, waits for the "learning" state to complete, and then the Customer Assistant successfully answers a simulated question based on that policy.
  - **Acceptance Criteria**: E2E tests must prove a document is uploaded, ingested in the background, and correctly retrieved during an AI query, with zero data leakage across tenants. No mock data in the UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
