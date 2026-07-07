issue_title: "Architecture Gap: Unified Global Search & Discovery System"
issue_description: |
  ## Problem Statement

  Currently, OneHumanCorp (OHC) lacks a unified, global search and discovery system that works seamlessly across all work entities (messages, customers, tasks, bookings, invoices). Owners like Maya, Carlos, and Nora are forced to navigate to specific tabs (Inbox, Customers, Finance) to find what they need. This violates the "Assistant-First" promise: the owner should be able to ask one assistant or use one search bar to pull up any context instantly.

  When Carlos needs to find "that customer who asked about the kitchen sink repair last month", he shouldn't have to guess if it's in a message, a draft quote, or a past booking.

  ## Research Report

  - **Tencent Workbuddy & DingTalk:** Both heavily rely on a unified command center/global search that indexes all organizational data. The search is not just text-matching; it's intent-driven and entity-aware.
  - **Notion AI & Microsoft Copilot:** They index entire workspaces and use vector embeddings to surface semantically relevant documents and tasks, rather than just keyword matches.
  - **SMB Context:** Small business owners are mobile-first and time-poor. A unified search bar on a 375px screen is often the primary navigation tool.

  ## Design Doc

  ### Architecture Highlights
  - **Global Indexed Search Service:** A unified search index (e.g., using PostgreSQL full-text search combined with pgvector for semantic search) that ingests changes from all domains (Inbox, CRM, Operations, Finance) via CDC (Change Data Capture) or the AI Job Queue.
  - **Multi-Tenant Search Boundary:** Strict row-level security and multi-tenant scoping within the search index to guarantee data isolation.
  - **Assistant Integration:** The AI Assistant uses this search index as its primary memory retrieval tool to answer owner queries ("What's the status of Nora's project?").

  ### Mobile UX Flow
  1. **The Owner's View:** From the OHC Home Screen (375px), a prominent "Search or Ask Assistant" bar is sticky at the top.
  2. **Intent Search:** Carlos types "kitchen sink". The results are instantly grouped by category: Customers (John Doe), Invoices (Invoice #102 - Sink Repair), Messages ("Can you fix my sink?").
  3. **Actionable Results:** Tapping a result doesn't just open it; it offers quick actions (e.g., "Draft Reply", "Send Invoice Reminder").

  ### System Diagram (Mermaid)

  ```mermaid
  graph TD;
      Domains[All OHC Domains: Booking, Inbox, etc.] -->|Write Events| Queue[Event Bus / Job Queue];
      Queue --> Indexer[Search Indexer Service];
      Indexer --> DB[(PostgreSQL + pgvector)];
      Owner[Owner Mobile App] -->|Query| SearchAPI[Search API / Assistant];
      SearchAPI --> DB;
      SearchAPI -->|Contextual Results| Owner;
  ```

  ## Implementation Prompt

  **To the Implementer:**
  Design and implement the `Search` domain and its integration into the core OHC platform.
  1. Create the `search.proto` definition for unified search queries and structured results.
  2. Implement the Search service in Go, utilizing PostgreSQL full-text search (and optionally pgvector) while strictly enforcing `tenant_id` boundaries via row-level security.
  3. Wire up the existing domains (Inbox, Booking, CRM) to emit indexable events (or build a CDC mechanism) to populate the search index.
  4. Build the mobile-first (375px) sticky search bar in the Flutter App that queries this new service and displays categorized results. Ensure it aligns with the OHC Premium Token library design system.
  5. Provide 100% test coverage and Playwright E2E tests for the "Global Search" CUJ.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
