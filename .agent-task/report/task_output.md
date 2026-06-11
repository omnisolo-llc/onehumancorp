issue_title: "[Architectural Design] PowerSync Local-First Data Strategy & Unified Inbox"
issue_description: |
  # PowerSync Local-First Data Strategy & Unified Inbox for OHC

  ## Problem Statement
  Small business owners operate in highly variable environments—often with spotty network coverage (e.g., in a bakery kitchen, at a client's home, or working a food cart). Our personas (Maya, Carlos, Fatima) need uninterrupted access to their critical business data (orders, messages, schedule) and the ability to interact with the system seamlessly, offline or on weak connections. Currently, OHC lacks a robust, standardized local-first data architecture that guarantees offline operability and seamless background synchronization once connectivity is restored. Furthermore, the absence of a "Unified Inbox" means that messages across various channels (Instagram DMs, SMS, Email, web forms) are fragmented, violating the core promise of a coordinated "Agent Feed".

  ## Research Report
  - **Competitor Analysis:** Modern platforms like Linear and Notion have set the gold standard for "local-first" responsiveness. They achieve instant UI updates by writing to a local store first, then syncing asynchronously.
  - **OHC Persona Needs:** Fatima (Food Cart Operator) specifically requires an offline-tolerant flow due to slow mobile data. Carlos (Field Service) needs instant access to route notes and bookings even in areas with no cellular service.
  - **Technical Gap:** While `src/server/powersync.rs` contains basic key generation infrastructure, a comprehensive schema, client-side integration (Flutter + PowerSync SDK), and sync logic for a Unified Inbox and Agent Feed are missing.
  - **Proposed Capability:** Implement a structured local-first architecture leveraging PowerSync (or equivalent embedded database sync) combined with a Unified Inbox data model that aggregates multi-channel communications.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Flutter Mobile App (375px)] --> B[Local Embedded DB (SQLite/PowerSync)]
      B --> C[Offline UX: Instant Read/Write]
      A --> D[Agent Feed UI]
      D --> B
      B <-->|Background Sync (PowerSync Protocol)| E[OHC Backend API (Go/Rust)]
      E --> F[PostgreSQL (Row-Level Security)]
      E --> G[Agent Event Bus (Kafka/Redis)]
      G --> H[Unified Inbox Ingestion (IG, SMS, Email)]
      H --> F
  ```

  ### Mobile UX Flow (375px First)
  1. **Offline Mode:** The app launches instantly, loading data from the local DB. No loading spinners for cached data.
  2. **Unified Inbox View:** A single tab displaying grouped messages (IG DMs, SMS) and Agent Feed "Action Cards".
  3. **Interaction:** User approves an Agent-drafted response while offline. The action is recorded locally, the UI updates immediately, and the action is queued.
  4. **Sync Resolution:** When connectivity returns, the background sync pushes the queued action to the backend. The backend resolves conflicts and updates PostgreSQL.

  ### Data Model & Multi-Tenancy (High Level)
  - All synchronized tables must include `tenant_id`.
  - Row-Level Security (RLS) policies on PostgreSQL ensure PowerSync only replicates data belonging to the authenticated tenant.
  - **Unified Message Entity:** Standardized schema for communications, including `source_platform`, `external_id`, `content`, `status`, and `agent_draft_id`.

  ### AI Agent Integration
  - The Event Ingestion Pipeline (as described in `agent_feed_deep_dive.md`) populates the backend DB.
  - PowerSync pushes these new events (e.g., a drafted reply for an IG DM) to the local client seamlessly.
  - The local client renders an "Action Card" in the Agent Feed based on this synced data.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend sync configuration and the foundational Flutter client setup for the PowerSync local-first architecture.
  1. Define the SQL schema and replication rules (Sync Rules) required to synchronize the `unified_messages` and `agent_action_cards` tables to the mobile client securely, ensuring strict `tenant_id` isolation.
  2. Set up the basic Flutter provider/service that initializes the local SQLite database and connects to the sync endpoint using the configured keys.
  3. Create a simple 375px-optimized UI component that reads from the local database to display a list of pending action cards, demonstrating instant offline reads.
  *Note: Do not prescribe specific PowerSync SDK versions or minute implementation details—focus on achieving the local-first read/write CUJ for the offline owner persona.*

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
