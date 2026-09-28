issue_title: "No-work finding: Missing compute/API charging baseline metrics and usage identity"
issue_description: |
  Based on `docs/research/business_capability_and_usage_economics_audit.md` and `RESEARCH.md` (revision 2026-09-18-usage-audit), the required prerequisites for implementing trustworthy compute/API usage economics and billing rates are lacking.

  Specifically:
  - The current source does not supply a measured deployment cost, representative workload distribution, or reconciled provider invoice.
  - Idempotent usage tracking, tenant-specific reads, and durable usage identity for proper attribution are missing.
  - As per the research report, we must measure actual model/tool usage, compute, and reserved capacity before setting rates.

  Therefore, this task yields a 'no-work finding' as the necessary baseline metrics and evidence are absent. Building a billing system or selecting pricing segments without this evidence violates the established OHC operating contract.

  Skill Provenance:
  - loaded superpowers/using-superpowers from commit 8ca22dba9a94f28898bbce59f2537ff4d87c747d
issue_priority: "P0"
issue_category: "research"
issue_type: "finding"
issue_label: "no-work-finding"
assignees: ["miser-agent"]
