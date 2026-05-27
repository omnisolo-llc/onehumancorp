issue_title: "[Architecture] Universal Cross-Channel Identity Resolution Engine"
issue_description: |
  # Research Report
  The current small business landscape forces owners to manually stitch together customer identities across different channels (Instagram DMs, Web storefront, Tap-to-Pay). Competitors like Shopify and Square handle this partially but fail to provide a unified, invisible cross-channel identity graph.

  ## Findings
  - We need an automated engine that deterministically and probabilistically resolves customer identities.
  - The engine must integrate smoothly with AI Agents for seamless CS and Marketing.
  - OHC is positioned uniquely to handle this because it natively owns the POS, the Web Storefront, and Social Agent integrations.

  ## Proposed Next Steps
  - Implement an event ingestion service to collect customer signals.
  - Create the AI-driven Identity Graph.
  - Enforce strict multi-tenant and zero-trust policies for PII handling.
  - Ensure the Mobile Merchant app presents this unified identity seamlessly in a 375px viewport with a Translucent Glass layout.

  Please see `docs/research/[architecture]_universal_cross_channel_identity_resolution_engine.md` for the full design documentation.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []