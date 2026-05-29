issue_title: "Implement Autonomous Client Retention & Winback Engine"
issue_description: |
  # Research Report: Autonomous Client Retention & Winback Engine

  ## Findings
  - **Gap**: OHC handles initial booking and purchasing well, but lacks a post-purchase automated retention loop. Small business owners lack time to identify dormant high-value customers and re-engage them.
  - **Competitor Analysis**: Shopify/Wix use basic time-delayed emails. Mindbody uses vertical-specific complex smart marketing.
  - **Solution**: An AI-driven background service that passively monitors the Universal Capacity & Inventory Ledger and Customer Identity Resolution Engine.

  ## Next Steps
  - Implement a background service that detects when a customer hasn't made a purchase in 1.5x their usual cadence.
  - Integrate with the Omnichannel Inbox to send context-aware, personalized winback messages via SMS/WhatsApp/IG.
  - Enable AI agents to negotiate and auto-complete bookings directly in the chat thread.
  - Build a simple "Grandmother Test" UI toggle to enable the feature, backed by a "recovered revenue" metric card.

  See full design doc at `docs/research/[architecture]_autonomous_client_retention_and_winback_engine.md` for Mermaid diagrams and detailed requirements.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
