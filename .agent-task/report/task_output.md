issue_title: "OHC Inventory Local-First Offline Sync architecture"
issue_description: |
  The issue requests implementing a local-first offline inventory sync architecture for POS systems using CRDTs and Operations Agent escalation. While there is partial implementation (e.g., CRDTOfflineSynchronizer in src/server/services/sync/offline_pos.rs and PN-counters in src/server/db/migrations/134_a_add_pn_counters.sql), this work falls under the POS engine category.

  According to RESEARCH.md and the system instructions: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require explicit evidence and authorization via the expansion gate in RESEARCH.md."

  Since there is no such explicit authorization for POS engines in the expansion gate, this work is unauthorized and blocked.

  Skills loaded: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)
outcome: "blocked"
