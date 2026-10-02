outcome: no_work
issue_title: "Implement Agentic Unresolved Pain Point Solutions based on Shopify Sidekick Deep Dive"
issue_description: |
  The requested feature aims to build a native Rust omnichannel engine replacing Chatwoot, and to implement new agentic action feeds for onboarding and messaging.

  Per the current `RESEARCH.md` and OHC operating contract:
  - "Integrate existing tools instead of rebuilding them. Initial connector hypothesis is Google Workspace plus Stripe, subject to code/API/owner validation. Reuse current Rust/Tauri architecture and preserve existing standalone behavior."
  - "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines, and harness adapters require explicit authorization, evidence, and an expansion gate in RESEARCH.md; otherwise, return a no_work finding."
  - "Requests to build native replacements for external services (e.g., Chatwoot) [...] require explicit authorization, evidence, and an expansion gate in RESEARCH.md; otherwise, return a no_work finding."

  The `RESEARCH.md` capability map does not contain an expansion gate or explicit approval to build a native Rust multi-channel replacement for Chatwoot, nor does it approve the specific new proactive action feed UI requested in the issue. Therefore, no work can be performed.

  Background verification (make test && make lint) failed due to environmental limitations (missing system dependencies/Next.js export errors) in the test environment, which are documented as outstanding blockers.

  Superpowers provenance:
  - Loaded skills: brainstorming, using-superpowers
  - Superpowers repository revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d