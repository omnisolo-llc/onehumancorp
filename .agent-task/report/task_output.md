issue_title: "[Research] Distributed Lock Sync and Multi-Tenant Isolation for Local Edge RAG Operations"
issue_description: |
  # Research Report: Distributed Lock Sync and Multi-Tenant Isolation for Local Edge RAG Operations

  ## Problem Statement
  OneHumanCorp (OHC) agents rely heavily on Retrieval-Augmented Generation (RAG) to provide contextual, personalized automation for the business owner. Whether it's drafting a quote, answering customer queries, or summarizing daily operations, these agents need up-to-date access to tenant-specific documents, policies, and historical data.

  The current RAG synchronization process lacks strict multi-tenant isolation guarantees and a distributed locking mechanism. When multiple edge nodes or localized POS instances attempt to sync or query RAG indices simultaneously, there is a risk of:
  1. Data leakage across tenants (violating Zero Trust constraints).
  2. Index corruption due to concurrent writes.
  3. Stale data being served while sync operations overlap without locks.

  From the perspective of **Carlos (Field Service Owner)** or **Nora (Agency Principal)**, RAG data integrity is paramount. If Carlos's quotes mistakenly reference another tenant's pricing policy, or Nora's project intake assistant hallucinates based on overlapping syncs, trust in the "Invisible AI Automation" is broken.

  ## Research Report (Track 1 & Track 2)

  **Competitor & Industry Practices:**
  - **Pinecone / Weaviate / Qdrant:** Enterprise vector databases solve this by sharding by `tenant_id` natively and employing strong distributed consensus (Raft) for index updates.
  - **Shopify / Stripe:** Use Redis-backed distributed locks (Redlock algorithm) for handling concurrent state modifications (e.g., inventory deduction or payout calculation) per tenant.

  **The OHC Architecture Gap:**
  - OHC's current `rag_sync.rs` (observed in `src/server/rag_sync.rs`) maintains basic `SyncStatus` but lacks the Redis Redlock pattern documented elsewhere in our architecture guidelines (`ohc:lock:{tenant_id}:{resource_type}:{resource_id}`).
  - Multi-tenant isolation for vector sync operations must be hardened. The sync job queue (PostgreSQL `SKIP LOCKED` pattern) must be combined with a Redis-backed distributed lock during the actual vector embedding and storage phase to ensure only one worker processes RAG updates for a specific tenant's document at a time.

  ## Design Doc (Track 3)

  ### Architecture & Data Flow
  1. **Document Update Event:** A new policy or note is added by the owner.
  2. **Job Queue:** A `RagSyncJob` is inserted into PostgreSQL.
  3. **Dequeue:** A worker pulls the job using `SELECT ... FOR UPDATE SKIP LOCKED`.
  4. **Distributed Lock:** Before embedding, the worker attempts to acquire a Redis lock: `ohc:lock:{tenant_id}:rag_sync:{document_id}`.
  5. **Isolation Check:** The worker verifies the `tenant_id` context via SPIFFE/SPIRE identity before proceeding.
  6. **Vector Ops:** The document is embedded and upserted into the tenant-partitioned namespace of the vector store.
  7. **Lock Release:** Redis lock is released; Job is marked `Synced`.

  ### AI Agent Integration
  - **Knowledge Assistant:** Relies on the updated `SyncStatus` to know when new documents are ready for querying, preventing it from serving partial or stale answers.

  ### Mobile UX Flow
  - 375px viewport: The user views their "Knowledge & Documents" tab.
  - A status indicator (using OHC Translucent Glass styling) displays "Syncing..." when documents are being processed, turning green when ready. No technical jargon (e.g., "Vector embedding...") is shown.

  ## Implementation Prompt (Track 4)

  **To the Implementer:**
  Your objective is to implement a robust distributed locking mechanism for RAG synchronization using Redis (Redlock pattern) and enforce strict multi-tenant isolation checks during the sync process.

  **User-Facing Outcome:** The business owner can upload multiple documents or policies simultaneously from their mobile device without causing system instability or seeing stale/mixed data in subsequent agent chats. The UI should reflect truthful sync states.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. As a user, I navigate to the Knowledge section on my mobile app and upload 3 new PDF policies simultaneously.
  2. The frontend displays a "Syncing..." status.
  3. **Backend Requirement:** The RAG sync worker must acquire a Redis lock (`ohc:lock:{tenant_id}:rag_sync:{document_id}`) before processing each document.
  4. **Backend Requirement:** If another worker attempts to sync the same document concurrently, it must respect the lock and back off.
  5. **Isolation Requirement:** The worker must explicitly validate the `tenant_id` against the current execution context before writing to the vector store.
  6. Once processing is complete, the UI updates to show the documents are active and ready for the Knowledge Assistant.

  **Verification:** Implement at least 5 unit/integration tests ensuring that concurrent sync requests for the same tenant/document correctly queue/lock, and that cross-tenant access is explicitly rejected. Add a Playwright E2E test to verify the upload and status update flow.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
