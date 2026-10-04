outcome: blocked
issue_title: "[Architecture] Edge-Native Real-Time Storefront Inventory Sync"
issue_description: |
  The requested feature (edge-native storefront inventory sync via Redis/KV) relates to retail, inventory, and POS expansion paths. According to RESEARCH.md, "Field services add physical work, routing and materials; retail/manufacturing add inventory, POS and supply-chain correctness. These remain expansion paths, not parallel launch requirements." The instruction "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require explicit evidence and authorization via the expansion gate in RESEARCH.md; otherwise, return a no_work or blocked finding." applies.

  No such authorization or evidence currently exists in RESEARCH.md for this feature. Furthermore, the issue mentions `storefront_delivery.go` but the codebase uses `storefront_delivery.rs` which already implements a storefront inventory injection (albeit not entirely edge-native). However, advancing this capability requires explicit authorization.

  Loaded skills:
  - skills/using-superpowers/SKILL.md (8ca22dba9a94f28898bbce59f2537ff4d87c747d)
