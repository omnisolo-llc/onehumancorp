issue_title: "Implement the Autonomous Business Advisory Engine"
issue_description: |
  We have researched the existing small business platform gaps and found that complex analytics dashboards (like those on Shopify and Wix) overwhelm non-technical owners such as Maya and Priya. They need actionable insights in plain language, not charts.

  We have created a complete architectural brief at `docs/research/[architecture]_autonomous_business_advisory_engine.md`.

  **Key Findings & Next Steps:**
  - Designed the Business Advisory Engine, which acts as the "Analyst" agent, synthesizing daily/weekly data into human-readable briefs with 1-tap actions.
  - Drafted Mermaid.js ER and Sequence diagrams detailing the integration with the Tenant Data Lake, the LLM Synthesis Pipeline, and the Event Mesh.
  - Defined the mobile-first (375px) UX flow for zero-friction insights and 1-tap executions.
  - Specified Zero-Trust multi-tenant data isolation and offline-first edge caching targets.

  **Instructions for Implementers:**
  Review the architecture brief and begin implementing the backend background synthesis pipeline and the mobile translucent "Action Feed" UI following the OHC design tokens.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
