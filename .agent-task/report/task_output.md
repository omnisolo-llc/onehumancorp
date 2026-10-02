outcome: no_work
issue_title: OHC Native Omnichannel Customer Support & Chat Engine
issue_description: |
  # Findings

  The issue requested building a native Rust multi-tenant omnichannel chat engine to replace Chatwoot for unifying customer communications (Instagram, WhatsApp, Email, Web Widget).

  However, reviewing the core strategic directives in `RESEARCH.md` reveals a fundamental conflict:

  **"Integrate existing tools instead of rebuilding them."**

  The current capabilities audit and strategy explicitly mandate building workflows via integration (e.g., Google Workspace + Stripe connectors initially). Re-implementing a full omnichannel chat platform (like Chatwoot, Zendesk, or WeCom) natively inside the OHC codebase directly violates this constraint.

  Per the repository operating contract:
  > The core strategic directive in RESEARCH.md is to 'Integrate existing tools instead of rebuilding them.' Requests to build native replacements for external services (such as Chatwoot) without explicit authorization, evidence, and an expansion gate must be rejected with a no_work finding.

  Additionally, no real external provider credentials (e.g., WhatsApp sandbox, Instagram sandbox) were provided to integrate or simulate these channels. Without an explicit expansion gate decision backed by evidence, building a native Chatwoot replacement is out of scope.

  This task was evaluated using the Superpowers `using-superpowers` skill, loaded at revision `8ca22dba9a94f28898bbce59f2537ff4d87c747d`.
