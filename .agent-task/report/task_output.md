outcome: blocked
issue_title: Zero-Data-Entry AI CRM with Automated Customer Profile Insights - Implementation
issue_description: |
  Blocked due to conflicts with the current database schema. The issue requests implementing a new `customer_profiles` and `customer_insights` table schema. However, the current code already uses a `customers` table with a JSONB `profile_summary` and a `customer_memory_context` table (added in migration `153_omnichannel_customer_memory_graph.sql` and `216_customer_memory_context.sql`).

  Furthermore, the `CustomerSuccessAgent` (Ambassador) already queries the `customers` and `customer_memory_context` tables (via `CustomerMemoryGraphService::get_profile_summary`) to retrieve the "Customer Profile: Interactions: ..., Segments: ..., Preferences: ..., Summary: ..." string when drafting replies in the `tenant.message.received` event handler. The `CustomerMemoryGraphService::ingest_interaction` is also already called.

  Creating a new, separate `customer_profiles` and `customer_insights` schema would duplicate existing functionality, diverge from the existing data model, and break the current integration in `CustomerSuccessAgent`. Therefore, the feature requested is either already implemented using the existing schema, or the issue description is obsolete and contradicts the current architecture.

  Loaded skills:
  - skills/using-superpowers/SKILL.md

  Superpowers commit revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
