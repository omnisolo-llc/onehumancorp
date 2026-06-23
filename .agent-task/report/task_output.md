issue_title: "Implement AI-Native Omnichannel Customer Context & Memory Graph Architecture"
issue_description: |
  ## Problem Statement
  Business owners currently struggle with a fragmented view of their customers. When Maya the Baker receives a DM on Instagram about a vegan cake, she doesn't immediately know if this customer has purchased before, if they have an upcoming booking, or if they had a previous complaint. Traditional CRMs are too manual and complex for our personas (Carlos, Priya, Leo). The gap is the lack of an invisible, auto-updating customer memory graph that unifies interactions across all channels (DMs, emails, POS, bookings) and surfaces relevant context to the owner proactively.

  ## Research Report
  - **Market Context**: Traditional CRMs like HubSpot or Salesforce require manual data entry, which small business owners (SMBs) abandon. Modern AI tools like Replit Agent or Claude Code focus on code, not customer context. Vertical tools like Vagaro or Slice have siloed customer data.
  - **Competitor Analysis**:
    - *Shopify*: Good customer profiles but limited cross-channel communication context (e.g., struggles with Instagram DMs unless heavily integrated with apps like Gorgias).
    - *HubSpot*: Overwhelming for micro-SMEs; requires manual logging or complex workflows.
    - *Breeze/Square*: Good transactional data but lacks the conversational "memory" of AI agents.
  - **The OHC Opportunity**: Create a "Zero-Data-Entry CRM." The AI agents (Customer Success, Sales, Operations) automatically listen to all channels and build a unified Knowledge Graph for each customer.

  ## Design Doc

  ### Architecture Design

  ```mermaid
  erDiagram
      CUSTOMER ||--o{ INTERACTION_EVENT : has
      CUSTOMER ||--o{ CONTEXT_SNIPPET : extracts
      CUSTOMER {
          uuid id PK
          uuid tenant_id
          string name
          vector embedding
          jsonb profile_summary
      }
      INTERACTION_EVENT {
          uuid id PK
          uuid tenant_id
          uuid customer_id FK
          string channel
          string raw_content
          timestamp created_at
      }
      CONTEXT_SNIPPET {
          uuid id PK
          uuid tenant_id
          uuid customer_id FK
          string category
          string extracted_value
          vector embedding
      }
  ```

  - **Data Model (Memory Graph)**:
    - Use PostgreSQL with `pgvector` to store conversational memory and interaction embeddings.
    - Entities: `Customer`, `InteractionEvent` (email, DM, POS purchase, booking), `ContextSnippet` (extracted preferences like "vegan", "allergic to nuts").
    - Strong tenant isolation via `tenant_id` on all tables with Row Level Security (RLS).
  - **Event Ingestion Pipeline**:
    - Centralized webhook ingestion (Stripe, Instagram, Email, Tap-to-Pay).
    - Asynchronous processing via AI Job Queue (PostgreSQL `SKIP LOCKED`).
  - **AI Department Coordination**:
    - *Data Ingestion Agent*: Parses raw events and extracts semantic entities (e.g., "Customer asked about vegan cake").
    - *Memory Consolidation Agent*: Periodically runs to summarize and compress interaction logs into a concise "Customer Profile Summary".
    - *Customer Relationship Assistant (The Ambassador)*: Queries the Memory Graph via RAG when drafting a reply to provide context-aware responses (e.g., "Hi [Name], I see you ordered a vegan cake last month...").
  - **Zero Trust & Security**: SPIFFE/SPIRE for inter-agent communication during data ingestion to ensure data provenance.

  ### Mobile UX Flow (375px First)
  1. Owner opens the OHC app.
  2. "Work Triage" feed shows a pending DM from "Alex".
  3. Tapping the DM opens the Chat View.
  4. At the top of the Chat View, a translucent "Context Card" (44px touch target) summarizes Alex: "Returning Customer • Vegan • 2 past orders • Last order: 30 days ago".
  5. The Customer Assistant suggests a drafted reply incorporating this context.

  ## Implementation Prompt
  **User Facing Outcome**: When an owner views a customer inquiry or profile, they instantly see a synthesized summary of all past interactions, preferences, and transactions without ever having manually entered data.
  **CUJ**:
  1. A new interaction event (e.g., an Instagram DM) is ingested.
  2. The backend extracts context and updates the customer's vector memory.
  3. The owner opens the app to the 375px Work Triage feed.
  4. The owner taps the message, and the UI displays a clean, unified "Context Card" alongside an AI-drafted reply that leverages the customer's history.
  **Acceptance Criteria**:
  - Define the PostgreSQL schema for the Customer Memory Graph, including `pgvector` embeddings and RLS.
  - Implement the background worker (AI Job Queue) that processes interaction events and updates the memory graph.
  - Create the API endpoint to fetch the unified Customer Profile Summary.
  - Build the mobile-first UI "Context Card" and integrate it into the message view.
  - Must include complete Playwright E2E tests validating the data flow from ingestion to UI display without mocking the database.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []