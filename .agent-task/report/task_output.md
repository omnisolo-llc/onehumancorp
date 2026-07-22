issue_title: "[research] Autonomous AI Knowledge Base and Document Automation"
issue_description: |
  # Autonomous AI Knowledge Base and Document Automation

  ## Problem Statement
  Small business owners and operators like Nora (Agency Principal) and Leo (Creator and Tutor) juggle numerous documents, contracts, policies, and scattered notes. When a new client requests a proposal, or a student asks about cancellation policies, the owner has to manually search through Google Drive, email threads, or notes apps to find the relevant information, then copy, paste, and format it. This process is time-consuming, error-prone, and pulls them away from their core work. They need an intelligent system that not only stores this information but actively understands it and uses it to automatically draft responses, proposals, and contracts without manual intervention.

  ## Research Report
  **Competitive Analysis:**
  - **Notion AI:** Excellent for internal knowledge management and basic generation, but requires the user to proactively navigate to the workspace, select the right context, and prompt the AI. It's an active tool, not a proactive assistant.
  - **HubSpot / Zendesk AI:** Can use knowledge base articles to draft support replies, but they are expensive, complex to set up, and primarily focused on customer support rather than holistic business documentation (like proposals or contracts).
  - **Google Workspace (Gemini) / Microsoft Copilot:** Good at summarizing specific documents, but lack the context of the business's daily operations (CRM, bookings, inventory).

  **OHC Opportunity:**
  OHC can differentiate by creating a "Knowledge & Compliance Assistant" that seamlessly integrates with the "Work Triage" feed. This assistant acts as the business's centralized, semantic memory. When a client requests a proposal, the Knowledge Assistant automatically surfaces past similar proposals and company policies, providing the Sales Assistant with the exact context needed to draft a highly accurate response instantly. This turns a static repository into an active, revenue-generating asset.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Owner Notes / Docs / Emails] --> B(Ingestion Engine)
      B --> C[Document Parsing & Chunking]
      C --> D[Semantic Vector Embeddings]
      D --> E[(pgvector Database)]

      F[New Client Inquiry] --> G(Work Triage Agent)
      G --> H{Context Required?}
      H -- Yes --> I[Knowledge Assistant]
      I -->|Semantic Search| E
      E -->|Retrieve Context| I
      I --> J[Sales / CS Agent]
      J --> K[Drafted Proposal / Reply]
      K --> L[Mobile App Feed 375px]
  ```

  ### Mobile UX Flow (375px First)
  1. **Capture:** Nora takes a quick photo of a handwritten project scope or forwards an email thread to OHC. The Knowledge Assistant quietly processes and tags it in the background.
  2. **Inquiry:** A new client DM arrives in the OHC feed asking for a web design proposal.
  3. **Proactive Drafting:** The Sales Agent starts drafting. It queries the Knowledge Assistant, which instantly pulls pricing from the "2025 Rate Card" doc and terms from a similar past project.
  4. **Review:** Nora sees a card in her feed: "Drafted Proposal for Client X." She taps it. The proposal is 95% complete.
  5. **Context Validation:** A subtle "Sources used" link at the bottom shows which internal docs the AI referenced, building trust.
  6. **Approval:** Nora taps "Approve & Send".

  ### AI Agent Integration Points
  - **Knowledge & Compliance Assistant (The Librarian):** Constantly indexes incoming documents, chat histories, and notes into a semantic vector space (pgvector). Manages the organizational memory.
  - **Customer Success Assistant:** Queries the Librarian to answer complex customer questions (e.g., "What is your refund policy on custom cakes?").
  - **Sales Assistant:** Queries the Librarian to draft accurate proposals and quotes based on historical data and current rate cards.

  ### Key Design Decisions
  - **Vector Database (pgvector):** Essential for semantic search. We must be able to find documents based on *meaning*, not just keyword matching.
  - **Invisible Ingestion:** The owner shouldn't have to manually "tag" or "categorize" documents. The AI should auto-categorize based on content.
  - **Source Attribution:** To build trust, AI-generated drafts must optionally show which internal documents were used to create them.

  ## Implementation Prompt
  Implement the Knowledge Base ingestion and semantic retrieval pipeline.
  - **User-Facing Outcome:** Users can upload documents (PDF, txt, images of text) or forward emails to OHC. When a relevant inquiry arrives in the Work Triage feed, the AI automatically drafts a response using facts specifically pulled from those uploaded documents.
  - **CUJ (Critical User Journey):**
    1. User uploads a "Cancellation Policy" text document via the mobile app.
    2. A customer sends a message asking, "Can I cancel my lesson tomorrow?"
    3. The Knowledge Assistant retrieves the policy.
    4. The Customer Success agent drafts a reply based *only* on that policy.
    5. The user sees the drafted reply in their feed, with a small indicator showing the "Cancellation Policy" was referenced.
  - **Acceptance Criteria:**
    - Document text is extracted, chunked, and stored as embeddings in PostgreSQL (pgvector).
    - An internal API allows other agents to perform semantic similarity searches against the tenant's knowledge base.
    - The UI includes a simple "Upload Document" area that accepts files and shows processing status.
    - The AI accurately grounds its drafted responses in the retrieved documents, minimizing hallucination.
    - Complete multi-tenant isolation: Agent must never retrieve document chunks belonging to another tenant.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
