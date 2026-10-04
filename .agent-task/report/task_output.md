outcome: blocked
issue_title: "Architectural Design: Zero-Click Autonomous Onboarding Agent"
issue_description: |-
  The task requires implementing a "Zero-Click Onboarding Agent" that autonomously provisions domain/tenant context, custom Stripe deposits, products from photos, and sets up a booking calendar.

  However, according to `docs/research/business_capability_and_usage_economics_audit.md` (and `RESEARCH.md`), "Days 15–45" mandate that we must "enforce isolation, budgets, authorization, deduplication, revocation and restart recovery **before** enabling external automation". Additionally, the current audit records state that "evaluation of managed API vs API-key/cloud-account billing is currently blocked pending real usage data and owner interviews," and "no supported native-client subscription mode was demonstrated end-to-end in OHC."

  As instructed in the user's explicit operating contract (`OneHumanCorp operating contract — revision 2026-09-18-usage-audit`), I must state the evidence and expected owner result:
  - **Evidence:** Code documentation and audit findings specifically list usage economics and external provider reconciliation/cost tracking as blocking prerequisites. No supported native-client subscription mode exists to pay for this LLM context processing securely without pooling consumer credentials. We also must not invent new broad agent epics.
  - **Expected Owner Result:** The task cannot be safely executed (money/data security) without the foundational budget/authority policies.
  - **Result:** Thus, this task is explicitly returned as a blocked finding. No dummy code has been generated.
