issue_title: "[Research] OneHumanCorp Global Search & Omnibox Architecture"
issue_description: |
  ## Problem Statement
  Currently, the OHC workspace lacks a unified global search and command palette (Omnibox). Business owners (like Maya the baker or Carlos the handyman) have their data scattered across Messages, Orders, Customers, Appointments, and Invoices. When a customer calls and says "Hi, it's John, I have a question about my deposit," the owner must click around multiple views to locate John, find the invoice, and check the appointment. This breaks the "One Assistant" promise and forces the owner to manually piece together context.

  ## Research Report
  - **Competitor Analysis:**
    - **Stripe Dashboard:** Uses `Cmd+K` / global search incredibly well. A single search bar finds customers, payments, subscriptions, and even nested settings.
    - **Linear / Superhuman:** Pioneered the "Omnibox" approach where search and action are unified. You can search "John" or type "New Invoice for John".
    - **Shopify:** Global search finds orders, products, customers, and help docs.
  - **Gap in OHC:** OHC has separate list views for different entities but no unified entry point to quickly jump to a customer's profile, a specific order ID, or a setting.

  ## Design Doc
  **Architecture (High Level)**
  - **Backend (Go + Bazel + Postgres):**
    - A new `SearchService` (gRPC / REST) that performs parallel queries or uses Postgres Full Text Search (`tsvector`) across core tables (customers, orders, messages, invoices) scoped to the `tenant_id`.
    - Returns a unified `SearchResult` list categorized by entity type.
  - **Frontend (Flutter PWA/Mobile):**
    - A floating Omnibox component triggered by a global shortcut (e.g., `Cmd+K` or `Ctrl+K`) or a persistent top-bar search input.
    - Real-time typeahead (debounced).
  - **Mobile UX Flow:**
    - On 375px viewports, the search bar is a prominent sticky header. Tapping it opens a full-screen overlay with recent searches and real-time results as the user types.

  **Mermaid Diagram:**
  ```mermaid
  sequenceDiagram
      actor Owner
      participant Omnibox UI (Flutter)
      participant Search API (Go)
      participant DB (Postgres)

      Owner->>Omnibox UI (Flutter): Types "John"
      Omnibox UI (Flutter)->>Search API (Go): GET /search?q=John
      Search API (Go)->>DB (Postgres): FTS Query (tenant_id, q="John") across tables
      DB (Postgres)-->>Search API (Go): Matched customers, orders, messages
      Search API (Go)-->>Omnibox UI (Flutter): Unified SearchResult
      Omnibox UI (Flutter)-->>Owner: Displays grouped results
  ```

  ## AI Agent Integration Points
  - **"Do" commands:** The Omnibox shouldn't just be search; it should feed into the Work Triage AI. If the user types "Create an invoice for John for $50", the Omnibox detects the intent and passes the context to the Sales & Revenue Assistant to draft the invoice, rather than just searching for it.

  ## Implementation Prompt
  Implement a unified Global Search / Omnibox for the OHC workspace.
  1. Add a Go backend search endpoint that queries customers, orders, and messages using Postgres text search, strictly scoped to the tenant.
  2. Build a responsive Omnibox UI component in Flutter (macOS-style translucent glass, accessible via Cmd+K or a tap on mobile) that fetches and displays these results.
  3. Ensure that tapping/clicking a result navigates the owner directly to the relevant record's detail view.
  4. Ensure 100% test coverage and E2E verification of the search journey.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
