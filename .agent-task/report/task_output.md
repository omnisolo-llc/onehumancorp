outcome: blocked
issue_title: Instant Localized Invoicing & Multi-Currency Architecture
issue_description: |
  Blocked finding for Instant Localized Invoicing & Multi-Currency Architecture.

  The implementation is blocked because the required base prerequisite (OHC-05 "durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit", documented as F05) is incomplete and marked as "Blocked" in docs/research/native_migration_and_remediation.md.

  The system currently lacks the foundational unified ledger consistency (transactions recorded in the tenant's base currency with foreign currency amounts and exchange rates). The issue request requires integrating with this "Unified Ledger DB - PostgreSQL", which currently cannot support the multi-currency accounting required.

  Additionally, the project explicitly forbids new verticals, channels, or payment integrations without explicit evidence and authorization via the expansion gate in RESEARCH.md. The requested multi-currency and instant localized invoicing is an expansion of the billing and commerce capabilities, and requires prior authorization which is absent.

  Skills retrieved:
  - Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - skills/using-superpowers/SKILL.md
  - skills/brainstorming/SKILL.md
