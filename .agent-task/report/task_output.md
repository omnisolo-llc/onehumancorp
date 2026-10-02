outcome: blocked
issue_title: "[no-work finding]"
issue_description: |
  **Task Issue:** GitHub Issue #36277: Design and Implement Native Rust Omnichannel Chat Engine

  **Outcome:** The requested feature (a native Rust omnichannel chat engine for Instagram DMs and SMS, involving real-time WebSockets, strict tenant isolation, a Flutter UI, and AI triage integration) is explicitly prohibited by the `OneHumanCorp operating contract — revision 2026-09-18-usage-audit` and the `RESEARCH.md` capability map.

  **Evidence / Blockers:**
  1. The contract states: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md." This issue describes new channels (Instagram DMs, SMS) and an entire native engine that is not present in the current validated capability map.
  2. The contract states: "Integrate existing tools instead of rebuilding them... Requests to build native replacements for external services (e.g., Chatwoot) require explicit authorization, evidence, and an expansion gate; otherwise, return a `no_work` finding." The issue mandate ("the legacy third-party chat service is 100% RETIRED... must improve upon native data models... Built in Rust inside onehumancorp/mono") is exactly what is forbidden without prior business validation.
  3. The contract instructs: "Do not build another generic provider abstraction first."
  4. The issue requests a Flutter UI ("Build a basic Mobile-First (375px) Flutter view"). The codebase audit establishes that the UI stack is built with Next.js and Tauri, not Flutter. The contract mandates: "Reuse the current Rust backend and trace actual UI packaging... Do not dismiss Next paths from a legacy label or create another frontend/rewrite."
  5. As verified by a `make test && make lint` dry run, the environment validation fails with `Next build failed: 1` and `Cannot find module 'pg'` because of systemic dependencies absent in the provided sandbox environment (and nextjs static build errors like `Dynamic server usage: Route /api/v1/growth/viral-countdown-widget/embed couldn't be rendered statically because it used request.url`). The environmental limitation is reported per standard instructions.

  Therefore, this is a blocked / no-work outcome. No code changes have been made to the repository.

  **Loaded Superpowers Skills:**
  - `using-superpowers`
  - `brainstorming`
  Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
