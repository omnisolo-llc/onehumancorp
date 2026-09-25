issue_title: '🚀 Nova: Growth optimization blocked by lack of funnel evidence'
issue_description: |
  **Nova Audit Findings:**
  - Evaluated existing pricing model boundaries inside `src/server/pricing/budget.rs` and `src/server/api/proposals.rs` as well as the active business-capability map.
  - The `$99 subscription` and `300-step allowance` are confirmed to be suspended hypotheses, not validated segment demands.
  - No existing conversion loops (funnels) or real owner evidence was found for paid plan acquisition in the metered API model.
  - Therefore, implementing a new growth widget, soft paywall, or referral program is blocked due to the lack of actual customer funnel evidence and concrete pricing plans.
issue_priority: 'P2'
issue_category: 'Research'
issue_type: 'Audit'
issue_label: 'Pricing'
assignees: []
