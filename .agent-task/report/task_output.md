outcome: no_work
issue_title: "[Architecture] Native Rust Omnichannel Inbox & Chatwoot Replacement"
issue_description: |
  **Finding:** The requested feature (a native Rust Omnichannel Inbox replacement for external tools like Chatwoot) is explicitly forbidden by the `RESEARCH.md` repository constraint.

  **Evidence:**
  According to `RESEARCH.md` (revision `2026-09-18-usage-audit`), there is a strict "expansion gate":
  "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."

  And specifically regarding channels and new epics: "Keep all deployment, communication, spending and professional-action authorization boundaries." "Existing modules are reuse candidates, not a mandate to build every item below. Payroll/tax engines, MRP, POS, course platforms, voice and additional channels are deferred pending explicit expansion."

  "Evaluate managed API, customer API-key/cloud-account billing, provider-permitted native-client subscription access and local inference separately... Missing or materially conflicting strategy is a blocker to new scope, not a reason to improvise another product direction."

  "The product minimizes owner effort by completing everything it can and accurately coordinating the remaining real-world step. For physical businesses, an AI team can run operations while a person or supplier delivers the physical service."

  "Do not implement a generic assistant, duplicate subsystem or broad ERP on the basis of this research."

  The requested architectural feature explicitly requests replacing an existing system (Chatwoot) with a whole new "Native Rust Omnichannel Chat Engine" (multi-tenant persistent chat, native adapters, WebSocket server, agent orchestration) from the "Capability reference catalog" in `RESEARCH.md`, which is explicitly listed under "This scope gate overrides legacy mandates to cover ten pillars... No deferred feature is commissioned merely because an old prompt says mandatory or a generic TODO exists."

  Because there is no explicit expansion gate authorization in `RESEARCH.md` that opens up the "channels" expansion or approves building this new native omnichannel engine, this work cannot proceed.

  **Skill Provenance:**
  Loaded skills: `skills/using-superpowers/SKILL.md`
  Upstream revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
