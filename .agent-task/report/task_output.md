outcome: no_work
issue_title: "Implement Zero-Data-Entry AI CRM with Automated Customer Profile Insights"
issue_description: |
  I've searched the codebase to evaluate implementing the requested "Zero-Data-Entry AI CRM" feature.
  The issue asks to build a new `customer_profiles` entity, a `customer_insights` AI metadata table, and a mobile CRM view from scratch. It also specifies updating the Omni-Inbox service to trigger an Ambassador agent.
  However, the `customers` and `customer_memory_context` tables already exist (`src/server/db/migrations/120_customers_table.sql`, `src/server/db/migrations/216_customer_memory_context.sql`). They already implement the requested AI memory/graph capabilities via `src/server/workers/customer_memory_worker.rs`, which runs an LLM over interactions to build a JSON memory graph of preferences and status. Furthermore, an existing mobile CRM view already surfaces these insights (`src/ui/next/src/app/customer/memory-graph/page.tsx`).
  The active `RESEARCH.md` contract states: "Retain existing business modules; do not build another generic assistant, duplicate subsystem or broad ERP on the basis of this research." Additionally, the issue description demands we update "the Omni-Inbox service to trigger the Ambassador agent", but the actual `customer_success_agent.rs` handles Ambassador drafting behavior already, while the async worker extracts the memory context globally.
  Since the specific requested capabilities (automated profile memory extraction, persistence, and a mobile UI surfacing those AI insights) are already complete under the `customer_memory_context` architecture, there is no authorized work to perform without duplicating an existing subsystem. Thus this is reported as a `no_work` finding.
  Loaded skills:
  - skills/using-superpowers/SKILL.md
  Skill provenance: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
