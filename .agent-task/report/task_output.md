issue_title: "Implement Multi-Tenant Edge-Caching Conversational Commerce Engine"
issue_description: |
  # Problem Statement
  Small business owners rely heavily on social media platforms (Instagram, WhatsApp) for customer acquisition and conversational orders. High-latency APIs or centralized databases cause delayed AI agent responses, losing impulsive buyers. A multi-tenant, edge-cached conversational commerce engine is needed to allow AI agents to instantly verify inventory, quote prices, and generate localized checkout links natively within social channels with zero perceptible latency.

  # Research Report
  - **Current Architecture Limits:** OHC's current architecture relies on centralized cloud databases which introduce significant latency when AI agents need to check live inventory for Instagram DMs.
  - **Discovery:** Push read-heavy data (catalog, availability calendar) to edge nodes using a `HybridCache` allowing for ultra-fast, single-digit millisecond reads by the AI agent. The AI agent must run at the edge to provide instant conversational responses.

  # Design Doc
  See `docs/research/[architecture]_multi_tenant_edge_caching_conversational_commerce.md` for full design and architecture limits.

  # Implementation Prompt
  Implement a multi-tenant edge-caching layer for conversational commerce AI agents using `src/server/utils/cache.rs` `HybridCache`. Ensure the `src/server/api/agents/webhook.rs` handles webhooks instantly using the edge cache for inventory checking and generates a checkout link in under 500ms using a provider from `src/server/integrations/registry.rs`. Use the Stripe integration for checkout preference.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
