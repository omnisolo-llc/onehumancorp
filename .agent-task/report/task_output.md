issue_title: "Autonomous AI Bookkeeping & Dynamic Margin Analytics Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) operate with zero visibility into their real profit margins. They run their businesses out of a single bank account and use gut-feeling for pricing. Existing platforms (Shopify, Wix) offer complex analytics dashboards filled with graphs and charts that are overwhelming and unactionable for non-technical users. They act as passive data displays rather than active financial assistants. This results in businesses underpricing their services or products and failing due to invisible cash flow issues.

  ## Research Report
  ### Competitive Analysis
  - **Shopify:** Provides robust analytics but requires the user to understand e-commerce metrics (AOV, LTV, Conversion Rate). It doesn't track off-platform costs easily and requires installing third-party apps (e.g., Quickbooks integration) for true bookkeeping, adding monthly costs and integration complexity.
  - **Wix:** Basic reporting focused primarily on web traffic and simple sales numbers. Lacks deep margin analysis or predictive cash flow modeling.
  - **GoDaddy/Squarespace:** Surface-level dashboards.

  ### The OHC Opportunity
  OneHumanCorp (OHC) can abstract the entire concept of "bookkeeping" into an invisible background process. Instead of a dashboard, OHC uses an autonomous financial AI agent that ingests all platform activity (sales, refunds, inventory costs, time spent) and provides a daily/weekly plain-language text summary (e.g., "You made $400 this week. Your profit margin on vegan cakes dropped by 5% due to ingredient costs. Consider raising the price by $2."). This passes the "grandmother test" and aligns with the OHC AI Differentiation Manifesto.

  ## Design Doc
  ### Key Design Decisions
  1. **Zero-Dashboard UX:** Replace traditional charts with a conversational, plain-language financial feed.
  2. **Invisible Ledger:** Every action (booking, sale, material purchase) automatically creates dual-entry ledger records behind the scenes.
  3. **Multi-Tenant Data Isolation:** Ensure financial data is strictly partitioned using zero-trust SPIFFE/SPIRE identity concepts at the ledger level.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ LedgerEntry : owns
      LedgerEntry {
          string id
          string tenant_id
          decimal amount
          string type
          timestamp created_at
      }
      Tenant ||--o{ FinancialInsight : receives
      FinancialInsight {
          string id
          string tenant_id
          string plain_text_summary
          string suggested_action
          timestamp generated_at
      }
      Agent_FinanceDepartment ||--o{ FinancialInsight : generates
      LedgerEntry }o--|| Agent_FinanceDepartment : analyzed_by
  ```

  ### UI Wireframes / Mobile UX Flow (375px First)
  - **Screen 1 (Home/Feed):** Instead of a "Sales Dashboard", the user sees an activity feed of cards styled with macOS-style translucent glass. The top card is a "Financial Briefing".
  - **Screen 2 (Briefing Detail):** Plain text: "You earned $1,200 this week! After material costs, your profit is $850. The Custom Cake service is your most profitable."
  - **Screen 3 (Action Modal):** A contextual button under the briefing says "Review suggested price adjustments." Tapping opens a bottom-sheet to 1-tap approve price bumps.

  ### AI Agent Integration Points
  - **The Analyst Agent:** Triggers weekly (or on significant financial events), queries the internal ledger, and uses an LLM to generate the plain-text financial briefing.
  - **The Manager Agent:** Identifies low-margin products or services and drafts suggested pricing updates for the user to approve via 1-tap.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Implement the backend core of the Autonomous AI Bookkeeping Engine. The outcome should allow the system to ingest basic sales and cost events and surface them as actionable plain-text insights for the business owner.
  - **CUJ (Critical User Journey):** Maya logs into her OHC app on her iPhone on Sunday morning. She receives a plain-text notification: "Weekly Financial Summary: You netted $500 this week. Profit margin looks healthy at 60%." She does not need to configure any charts or connect a bank account to see this basic OHC platform margin.
  - **Acceptance Criteria:**
    1. A secure, multi-tenant background worker that aggregates sales data over a period.
    2. An AI service integration that takes aggregated data and produces a plain-text summary and suggested actions.
    3. The summary must be available via an API endpoint optimized for mobile-first consumption (small payload, <200ms response).
    4. Ensure strict tenant isolation so data is safely partitioned. Do not prescribe specific database schemas; design for scalability and offline-first syncing capabilities where appropriate.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: "P1"
issue_category: "research"
issue_type: "task"
issue_label:
  - "agent-report"
assignees: []
