issue_title: "[Architecture] Omnichannel Conversational Commerce Engine"
issue_description: |
  **Research Report**:
  Our non-technical personas (like Maya the baker and Leo the tutor) conduct significant portions of their business via Instagram DMs and WhatsApp. Currently, they manually negotiate, schedule, and invoice via chat, leading to lost sales and burnout. Existing tools (Shopify Inbox, ManyChat) are either too complex or rely on brittle, rules-based logic.

  **Proposed Architecture**:
  We must integrate OHC's Agentic Core directly into Meta Graph API / Google Business Messages. Incoming messages will be normalized and routed to the Sales Agent, which will use our Hybrid RAG protocol to fetch real-time catalog/calendar data, negotiate with the customer in natural language, and generate Zero-Trust checkout links.

  **Next Steps**:
  Implement webhook receivers for Meta/WhatsApp, build event normalization pipelines, extend the Sales Agent to query Hybrid RAG for in-chat quoting, and develop the 375px-first `UnifiedInboxView` for the merchant to supervise and take over AI conversations.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []