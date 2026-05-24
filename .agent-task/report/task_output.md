issue_title: "[Architecture] Invisible Business Analytics and Growth Engine"
issue_description: |
  # Architecture Brief: Invisible Business Analytics and Growth Engine

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Leo (music tutor) are overwhelmed by complex dashboards with raw metrics (page views, bounce rates, CPC) in traditional platforms (like Shopify or Google Analytics). They suffer from "Financial Fog." They do not want to become data scientists; they just want actionable insights to grow their business. The current system lacks an underlying architecture to securely ingest events, compute growth metrics (LTV, churn, inactive customers), and feed this intelligence invisibly to the AI departments and the UI in plain language.

  ## Research Report
  - **Market Gap**: Platforms like Shopify Analytics, Wix Analytics, and Google Analytics provide raw numbers and charts, requiring owners to interpret data and act manually.
  - **SMB Need**: 35% of founders report "Financial Fog" as a major pain point. They need actionable advice ("Leo, 3 students haven't booked in a month, tap to send a follow-up offer"), not a chart of "Monthly Active Users."
  - **Architectural Requirement**: We need a high-performance, multi-tenant event ingestion pipeline that streams business events (page views, checkouts, booking cancellations, DMs) into an OLAP/timeseries store. This data must then be continuously analyzed by an AI engine to generate localized, plain-language insights and proactive growth actions.

  ## Design Doc

  ### Architecture and Entity-Relationship Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ BUSINESS_EVENT : "generates"
      CUSTOMER ||--o{ BUSINESS_EVENT : "performs"
      BUSINESS_EVENT ||--o{ INSIGHT_TRIGGER : "evaluates against"
      TENANT ||--o{ DAILY_BRIEFING : "receives"
      INSIGHT_TRIGGER ||--o{ PROACTIVE_ACTION : "spawns"

      BUSINESS_EVENT {
          uuid id
          uuid tenant_id
          uuid customer_id
          string event_type "e.g., checkout_completed, page_view, booking_cancelled"
          jsonb payload
          timestamp occurred_at
      }

      DAILY_BRIEFING {
          uuid id
          uuid tenant_id
          text plain_language_summary
          date briefing_date
      }

      PROACTIVE_ACTION {
          uuid id
          uuid tenant_id
          string action_type "e.g., winback_email, upsell_offer"
          string ai_department_owner "e.g., Marketing, CS"
          string status
      }
  ```

  ### Key Architectural Invariants
  1. **Zero-Trust Multi-Tenancy**: The event ingestion pipeline and analytical data store MUST strictly isolate data by `tenant_id` at the lowest level (e.g., Row-Level Security in Postgres/ClickHouse). No AI agent or query can cross tenant boundaries. Identity and access to the ingestion endpoints must be strictly verified using SPIFFE/SPIRE for inter-service authentication.
  2. **Invisible Intelligence**: The raw event data is NEVER shown to the merchant as a raw chart. It is exclusively consumed by the `AnalyticsEngine` and AI departments to synthesize the `DAILY_BRIEFING` and `PROACTIVE_ACTION` items.
  3. **High-Throughput Ingestion**: Events are ingested asynchronously via the event mesh (NATS) to ensure no performance degradation to the core transactional databases during high traffic (e.g., Maya's Instagram drop).

  ### UI Wireframes & Screen Flow (375px First)
  - **Daily Briefing Screen**: Adopts the macOS-style Translucent Glass materials with UniFi modular dashboard cards.
  - **Screen Flow**:
    1. Push Notification: "Good morning Priya! 3 quick updates on your boutique."
    2. Tap opens the app directly to the "Briefing" card.
    3. The card shows 3 plain-language bullet points (e.g., "Your new summer dresses are getting a lot of views, but few buys. Want to offer a 10% discount to those who looked?").
    4. A prominent, 1-tap "Do it" button is displayed next to the actionable insight. No charts are visible.
  - **Advanced Settings**: For the rare merchant who wants raw data, an "Export Raw Data (CSV)" button is hidden deep in the "Advanced Settings" menu.

  ### Performance and Offline Targets
  - **Latency:** The daily briefing payload must be pre-calculated and served to the mobile client in `<100ms` globally. Event ingestion must have a P99 latency of `<50ms` to the NATS mesh.
  - **Offline Capability:** The daily briefing must be cached locally on the device (e.g., via SQLite/PWA Cache) so it remains instantly readable even if the merchant drops to an offline state (e.g., while riding the subway).

  ### AI Agent Integration Points
  - **Finance AI Department**: Continuously monitors the event stream for revenue trends and flags cash-flow risks.
  - **Marketing & CS AI Departments**: Consume the `PROACTIVE_ACTION` triggers. If a customer is flagged as "churning" (e.g., Leo's inactive student), the CS Agent automatically drafts a polite SMS/email and queues it for Leo's 1-tap approval.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the "Invisible Analytics and Growth Engine".
  1. Create the `BusinessEvent` ingestion pipeline using our NATS event mesh, ensuring asynchronous, non-blocking writes to the datastore. Authenticate producers using SPIRE.
  2. Enforce strict `tenant_id` Row-Level Security on the event store.
  3. Build the background job that aggregates daily events for a tenant, passes the summary to the LLM service to generate the `DAILY_BRIEFING` in simple English, and persists it.
  4. Build the API endpoint to serve the `DAILY_BRIEFING` to the mobile client (375px optimized layout, `<100ms` latency).
  DO NOT implement complex charting libraries or raw data dashboards on the frontend. Focus purely on the ingestion and the generation of plain-language insights.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
