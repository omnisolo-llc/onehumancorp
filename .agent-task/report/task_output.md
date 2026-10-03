outcome: no_work
issue_title: "Implement Zero-Data-Entry AI CRM with Automated Customer Profile Insights"
issue_description: |
  The requested "Zero-Data-Entry AI CRM" functionality is already fully implemented in the current codebase. The `customers` table effectively acts as the core profile ledger, and it has already been extended with a `profile_summary` JSONB column. Furthermore, the `CustomerMemoryGraphService` and `CustomerMemoryWorker` already automatically extract customer preferences, context, and insights from omnichannel conversations using LLMs and persist them in the `customer_memory_context` table and the `customers.profile_summary` column. The frontend (`src/ui/next/src/app/inbox/page.tsx`) consumes this unified memory via `/api/v1/memory/summary/{customer_id}`. Therefore, creating duplicate tables named `customer_profiles` and `customer_insights` and adding duplicate extraction logic in the `message_triage_worker.rs` would create unnecessary technical debt and duplicate existing functionality.

  Superpowers workflow loaded skills:
  - Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Loaded paths:
    - skills/using-superpowers/SKILL.md
    - skills/brainstorming/SKILL.md
