outcome: no_work
issue_title: "[Mobile Tap-to-Pay & Offline-First POS] Unified Physical and Digital Retail"
issue_description: |
  The request asks to implement an offline P2P mesh sync for POS devices using Apple Multipeer and Android Nearby Connections. According to RESEARCH.md and docs/research/business_capability_and_usage_economics_audit.md, expanding into new verticals, POS engines, or creating additional frontends is deferred or requires explicit authorization, evidence, and an expansion gate. Specifically, the capability audit explicitly marks POS and retail as vertical extensions to be explored only after reliable payment foundations. Additionally, the existing code heavily references "tap-to-pay" and "offline sync" capabilities via mock contracts and offline CRDT sync implementations in `src/server/services/sync/offline_pos.rs` and `src/ui/next/src/app/pos/terminal/StripeTerminalClient.tsx`, which indicates that the offline-first POS features are either already in progress, completed, or deferred for future verification. Therefore, this is a blocked no-work finding.

  Skill provenance:
  - Repository: https://github.com/obra/superpowers.git
  - Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Loaded skills:
    - .agent-scratch/superpowers/skills/using-superpowers/SKILL.md
    - .agent-scratch/superpowers/skills/brainstorming/SKILL.md
