outcome: no_work
issue_title: "Implement Multi-Channel Redis Distributed Locks for POS & Inventory"
issue_description: |
  **No-Work / Blocked Justification**

  The issue requests implementing a Redis Redlock-based distributed lock for multi-channel inventory sync and POS transactions (`ohc:lock:{tenant_id}:inventory:{product_id}`).

  However, upon reviewing the codebase:
  1. The distributed lock mechanism `ohc:lock:{tenant_id}:inventory:{product_id}` is **already implemented** in `src/server/services/inventory/service.rs`, which provides a `reserve_inventory` function using `self.locker.acquire(&lock_key, &lock_id, ttl).await`.

  2. Integrating this lock into POS synchronization requires enabling multi-channel retail operations. However, `RESEARCH.md` specifically categorizes POS and physical retail under the **Gated Expansion** category. Expanding into POS requires explicit strategic approval, which is missing.

  Quote from `RESEARCH.md`:
  > "Gated expansion: Additional channels, industry packs, accounting integrations, physical operations, HR/payroll, POS, manufacturing, marketplaces and more harnesses. Require retained-customer need, measurable value and explicit strategy approval; existing capabilities are preserved."

  Therefore, since the Redis locking primitive is already available and extending it fully into the retail POS workflows is blocked by the strategic expansion gate and missing explicit authorization, no implementation work can proceed.

  Loaded skills:
  - skills/using-superpowers/SKILL.md (revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - skills/brainstorming/SKILL.md (revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
