issue_title: "[architecture] Unified Multilingual Hybrid Translation Mesh"
issue_description: |
  # Problem Statement
  Fatima, who runs a food cart and speaks limited English, needs to operate her entire business interface in Arabic while allowing customers to order in English, Spanish, or any local language. OHC currently lacks a globally consistent, real-time localized mesh that instantly translates storefronts, invoices, receipts, SMS notifications, and agent interactions without forcing the user to install a 3rd party localization plugin (which legacy competitors like Shopify and Wix require).

  # Research Report
  - **Competitor Analysis:** Shopify uses third-party plugins (e.g., Langify) which creates fragmented UI states and slows down page loads. Wix has a multi-lingual tool but it is manual and doesn't handle real-time conversational agent translations.
  - **OHC Advantage:** With the KAIROS underlying orchestration engine and LLM providers already embedded, OHC can dynamically translate the UI, product descriptions, and chat logs at the edge or locally (via standalone offline capabilities) without external plugin bloat.
  - **Market Context:** The LATAM and MENA markets represent huge growth potential. Native, zero-configuration multilingual support allows immediate deployment in non-English native contexts.

  # Design Doc
  - **Architecture Diagram:**
    - Edge Gateway (Intercepts user locale header/preferences).
    - Translation Cache Layer (Redis / Local CRDT DB for standalone).
    - LLM Translation Worker Pool (Part of Sub-Agent Queue, using exponential backoff).
    - Teammate Mesh Broadcast (Updates UI components via WebSocket/SSE immediately when translations are cached).
  - **Mobile UX Flow:**
    - At onboarding, Fatima selects her management language (Arabic).
    - The backend sets `tenant_lang=ar`.
    - A customer visits her storefront in English; the Edge Gateway dynamically renders English product descriptions, translating Arabic input on the fly.
  - **Zero Trust/Security:** Language data processing stays within tenant boundaries, strictly adhering to `tenant_id` isolation.

  # Implementation Prompt
  Role: Implementer Agent
  Task: Implement the core data models and service logic for the `TranslationMesh` module.
  Outcome:
  - Create a Postgres schema and SQLite equivalent for caching translations (`translation_cache` mapping text hashes to translated strings).
  - Add a translation worker task to the Sub-Agent Queue capable of performing batch translations.
  - Implement a basic gRPC service / API endpoint that components can call to retrieve localized strings, falling back to the queue if uncached.
  Acceptance Criteria: Unit test coverage MUST be 100%. Ensure no external data leaks between tenants. Follow the exact OHC standard for Postgres/SQLite hybrid database structures.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
