issue_title: "Implement Unified Omnichannel Customer Context & AI Memory Architecture"
issue_description: |
  # Research Report: Unified Omnichannel Customer Context & AI Memory Architecture

  ## Problem Statement
  Small business owners like Maya (Home Baker) and Carlos (Field Service Owner) interact with customers across multiple fragmented channels: Instagram DMs, WhatsApp, SMS, Web Forms, and in-person payments. Currently, OHC lacks a unified "Customer 360" data model that seamlessly merges interactions from all these touchpoints into a single, cohesive timeline. Without this, the Customer Assistant AI lacks historical context, leading to repetitive questions, lost leads, and a disjointed customer experience. We need an architectural foundation that unifies omnichannel identities and provides persistent, searchable context for our AI agents.

  ## Research Report
  ### Competitor Analysis
  - **HubSpot / Salesforce:** Provide robust unified customer views but are overly complex for micro-SMEs, requiring manual data entry and explicit rule configuration.
  - **Shopify:** Excellent at unifying purchase history but weak on integrating pre-sales chat (Instagram/WhatsApp) into the core customer profile natively.
  - **Tencent Workbuddy / WeCom:** Excels at merging WeChat interactions with CRM data invisibly, providing a unified feed for operators. This is the standard OHC must aim for.

  ### The Opportunity
  By building an autonomous Identity Resolution Engine and a Vector-Backed Customer Memory system, OHC can automatically link a WhatsApp inquiry to a past Stripe Terminal payment (using phone numbers or fuzzy matching) without requiring the owner to manually merge records.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CUSTOMER : owns
      CUSTOMER ||--o{ IDENTITY_LINK : has
      CUSTOMER ||--o{ INTERACTION_EVENT : participates_in
      INTERACTION_EVENT ||--o{ VECTOR_EMBEDDING : indexed_by
      IDENTITY_LINK {
          string channel "e.g., INSTAGRAM, WHATSAPP, EMAIL"
          string external_id
          boolean verified
      }
      INTERACTION_EVENT {
          string type "MESSAGE, PURCHASE, BOOKING"
          jsonb payload
          timestamp occurred_at
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View:** Maya opens the OHC app. She sees a new Instagram DM from "Sarah".
  2. **Contextual Sidebar/Drawer:** A single tap on Sarah's avatar slides up a bottom sheet (translucent glass styling). It shows Sarah's past custom cake order (from a web form) and a previous email inquiry.
  3. **AI Draft:** The Customer Assistant pre-fills a reply acknowledging Sarah's past order: "Hi Sarah! Good to hear from you again. Do you want the same chocolate cake as last month?"
  4. **Offline Resilience:** Customer profiles and recent events are cached locally using SQLite/Hive for instant loading on 3G networks.

  ### AI Agent Integration
  - **Customer Assistant (The Ambassador):** Uses the `VECTOR_EMBEDDING` table via pgvector to retrieve the top 5 most relevant past interactions before drafting any response.
  - **Operations Assistant (The Manager):** Listens to new `INTERACTION_EVENT` streams to automatically generate tasks (e.g., "Sarah asked for a quote, prepare estimate").
  - **Identity Resolution Worker:** A background job (using PostgreSQL SKIP LOCKED) that periodically scans unlinked `IDENTITY_LINK` records and uses LLM fuzzy matching to suggest merging profiles.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the backend foundation for the Unified Omnichannel Customer Context. Your task is to design and build the `customers`, `identity_links`, and `interaction_events` tables with row-level security (tenant isolation) in PostgreSQL. You must also create the API endpoints for the Customer Assistant to retrieve a unified timeline of interactions. Ensure that the database schema supports `jsonb` payloads for events and prepares for `pgvector` embeddings. Do not prescribe specific frontend state management yet, but ensure the API responses are paginated and optimized for a 375px mobile client. Acceptance criteria include full unit test coverage and E2E API tests demonstrating a tenant querying a merged customer profile.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
