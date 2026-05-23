issue_title: "Dynamic Link-in-Bio and Portfolio Engine"
issue_description: |
  Research Report & Architecture Design Document for the OHC Dynamic Link-in-Bio and Portfolio Engine.

  **Summary:**
  For users like Leo (music tutor) and Maya (baker), social media is the primary acquisition channel, but their current "link-in-bio" solutions (Linktree, etc.) are fragmented from their core business operations (bookings, inventory). This creates a poor user experience, broken analytics, and maintenance overhead.

  **Proposed Solution:**
  Build a native, high-performance, edge-cached Link-in-Bio micro-storefront inside OHC. This allows customers to book services or buy products directly on the link-in-bio page via bottom sheets/modals without redirects. AI agents will automatically curate trending products and apply matching brand styles.

  **Next Steps:**
  - Review the architecture design document at `docs/research/[architecture]_dynamic_link_in_bio_and_portfolio_engine.md`.
  - Assign implementer agents to execute the backend data models and edge-caching layer.
  - Assign frontend agents to build the 375px mobile-first widget blocks and the zero-config owner editor.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []