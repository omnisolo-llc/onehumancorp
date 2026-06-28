issue_title: "Universal Agentic Knowledge & Memory Architecture"
issue_description: |
  ## Title: Universal Agentic Knowledge & Memory Architecture

  ## Problem Statement
  Small business owners and operators (like Nora the Agency Principal or Maya the Baker) have critical business knowledge scattered across PDFs, handwritten notes, WhatsApp threads, and old emails. When Nora needs to draft a new proposal, or Maya needs to recall a customer's specific allergy request, they must manually hunt down this information. Current SMB platforms (like Shopify or Wix) offer no built-in way to ingest, index, and autonomously retrieve this unstructured context. Owners need an assistant that instantly "remembers" everything about their business and applies it to daily tasks without manual data entry.

  ## Research Report
  - **Market Context**: Traditional CRMs and platforms require structured data entry. Notion AI provides a strong knowledge base but lacks integration with daily operational tools like billing, quoting, or customer service inboxes. HubSpot's Breeze integrates AI with CRM data but struggles with unstructured, multi-modal SMB inputs (e.g., snapping a photo of a new menu or a supplier invoice).
  - **The OHC Opportunity**: By building a unified, multi-tenant vector memory architecture that all OHC AI agents (Operations, Customer Success, Sales) can access, we eliminate the need for the owner to organize information. The system becomes an autonomous "second brain" that proactively surfaces relevant context during work triage.
  - **Competitor Gaps**:
    - *Shopify*: Has basic file uploads but no agentic retrieval for drafting emails or proposals.
    - *Notion AI*: Excellent unstructured knowledge management but disconnected from transactional business operations (bookings, payments).
    - *HubSpot*: Enterprise-focused, requires significant structured data mapping, not friendly for a mobile-first user taking a photo of a policy.

  ## Design Doc
  ### Architecture & Data Model
  - **Vector Storage (pgvector)**: Use PostgreSQL with the `pgvector` extension for storing text embeddings, maintaining strict row-level security (RLS) via `tenant_id`.
  - **Document Processing Pipeline**: A background worker queue (PostgreSQL `SKIP LOCKED`) that ingests text, PDFs, and images (using Gemini Pro Vision for OCR/extraction), chunks the content, generates embeddings (via Minimax or OpenAI), and stores them in the `knowledge_chunks` table.
  - **Unified Memory API**: A gRPC/REST interface used by all OHC agents to query context.

  ### Architecture Diagram
  ```mermaid
  graph TD
      Owner[Owner (Mobile)] --> |Uploads Photo/Doc| Gateway[API Gateway]
      Gateway --> Worker[Background Ingestion Worker]
      Worker --> |Extract & Chunk| LLM[LLM/OCR Provider]
      LLM --> |Embeddings| PG[PostgreSQL + pgvector]

      Customer[Customer Inquiry] --> Triage[Work Triage]
      Triage --> SalesAgent[Sales/CS Agent]
      SalesAgent --> |Context Query| PG
      SalesAgent --> |Draft Reply| Owner
  ```

  ### Mobile UX Flow (375px)
  1. **Ingestion View**: A simple "+" FAB on the Owner Dashboard. Tapping it opens the camera or file picker. The owner snaps a photo of a vendor price list.
  2. **Processing State**: A brief translucent toast notification: "Extracting knowledge..."
  3. **Retrieval in Action (CUJ)**: When reviewing a new quote request from a client in the Work Feed, the owner sees an AI-drafted reply. A small "Context" chip indicates that the AI used the recently uploaded vendor price list to calculate the quote.

  ### AI Agent Integration
  - **Knowledge & Compliance Assistant**: Automatically tags and categorizes ingested documents.
  - **Sales Assistant**: Queries the vector store when drafting proposals to ensure accurate pricing and past client preferences are included.
  - **Operations Assistant**: Uses ingested policy documents (e.g., "no refunds after 24 hours") to automatically flag or handle customer disputes.

  ### Key Design Decisions
  - **pgvector over Pinecone**: Keeps the stack simple, self-contained, and allows easy enforcement of multi-tenant RLS alongside transactional data.
  - **Implicit Retrieval**: Owners should never have to manually search the knowledge base. Agents must implicitly query it based on the current task context.

  ## Implementation Prompt
  **Feature Name**: Universal Knowledge Ingestion & Agentic Retrieval
  **Target Persona**: Nora the Agency Principal
  **Outcome**: Nora can upload previous PDF proposals and contractor rate sheets. When a new project request comes in, the Sales Agent autonomously drafts a new proposal using the pricing and tone from the uploaded documents.

  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1. Nora logs into the OHC mobile UI and uploads a PDF document ("2025 Contractor Rates").
  2. The backend pipeline successfully extracts text, generates embeddings, and stores them in `pgvector` with correct `tenant_id` isolation.
  3. Nora receives a mock client inquiry via the inbox.
  4. The Customer Success Agent drafts a reply. The E2E test must verify that the drafted text incorporates specific pricing information found *only* in the uploaded PDF.
  5. The UI must display the AI-drafted reply with a clear indication (e.g., an attribution chip) of the source document used.

  **Next Actions**:
  1. Add the `pgvector` extension to the PostgreSQL schema and create the `knowledge_chunks` table.
  2. Implement the ingestion API and background worker for document processing and embedding generation.
  3. Update the agent prompt architecture to automatically perform RAG (Retrieval-Augmented Generation) queries against the vector store before drafting replies.
  4. Add the mobile-first upload UI and attribution chips in the work feed.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []